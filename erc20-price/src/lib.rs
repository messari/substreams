#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

mod keyer;
pub mod utils;

use abi::{chainlink_aggregator, factory, pair, price_feed};
use hex_literal::hex;
use lazy_static::__Deref;
use pb::chainlink::v1::Aggregator;
use pb::erc20::v1::Erc20Token;
use pb::erc20_price::v1::{Erc20Price, Erc20Prices};
use pb::uniswap::v1::PairCreatedEvent;
use std::ops::Not;
use std::str::FromStr;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::StoreNew;
use substreams::store::{StoreGet, StoreSet};
use substreams::store::{StoreGetProto, StoreSetProto};
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::{Event, Function};
use substreams_helper;
use substreams_helper::price;
use substreams_helper::types::Network;

#[substreams::handlers::map]
fn map_eth_price(block: eth::Block) -> Result<Erc20Prices, substreams::errors::Error> {
    map_price_for_tokens(
        block.number,
        vec![hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").to_vec()],
    )
}

fn map_price_for_tokens(
    block_number: u64,
    erc20_tokens: Vec<Vec<u8>>,
) -> Result<Erc20Prices, substreams::errors::Error> {
    let mut prices = Erc20Prices { items: vec![] };

    for erc20_token in erc20_tokens {
        let token_price = price::get_price(Network::Ethereum, block_number, erc20_token.clone())
            .map_err(|e| {
                substreams::errors::Error::Unexpected(format!("Failed to get price: {}", e))
            })?;

        let token = substreams_helper::erc20::get_erc20_token(Hex(erc20_token.clone()).to_string())
            .unwrap();

        prices.items.push(Erc20Price {
            token: Some(Erc20Token {
                address: token.address,
                name: token.name,
                symbol: token.symbol,
                decimals: token.decimals,
            }),
            block_number: block_number,
            price_usd: token_price.to_string(),
            source: 0,
        });
        log::info!("token {} price {}", Hex(erc20_token), token_price);
    }

    Ok(prices)
}

#[substreams::handlers::store]
fn store_chainlink_aggregator(block: eth::Block, output: StoreSetProto<Aggregator>) {
    for call in block.calls() {
        if let Some(decoded_call) = price_feed::functions::ConfirmAggregator::match_and_decode(call)
        {
            let decimals = chainlink_aggregator::functions::Decimals {}
                .call(decoded_call.aggregator.to_vec())
                .unwrap_or(BigInt::zero());
            let description = chainlink_aggregator::functions::Description {}
                .call(decoded_call.aggregator.to_vec())
                .unwrap_or(String::new());

            let base_quote: Vec<&str> = description.split(" / ").collect();

            if base_quote.len() != 2 {
                log::info!(
                    "[ChainlinkAggregator] Unexpected Description: {}",
                    description
                );
                continue;
            }

            let mut aggregator_address = decoded_call.aggregator;

            let nested_aggregator = (chainlink_aggregator::functions::Aggregator {})
                .call(aggregator_address.to_vec())
                .unwrap_or(Vec::<u8>::new());

            if nested_aggregator.is_empty().not() {
                // In `AggregatorFacade` contracts, the aggregator contract is nested two times.
                // Example: 0xb103ede8acd6f0c106b7a5772e9d24e34f5ebc2c

                aggregator_address = nested_aggregator;
            }

            let base_asset = match utils::TOKENS.get(base_quote[0]) {
                Some(base) => {
                    substreams_helper::erc20::get_erc20_token(String::from(base.deref())).unwrap()
                }
                _ => {
                    log::info!(
                        "Cannot find token mapping for base: {}",
                        base_quote[0].to_string()
                    );
                    continue;
                }
            };

            let quote_asset = match utils::TOKENS.get(base_quote[1]) {
                Some(quote) => {
                    substreams_helper::erc20::get_erc20_token(String::from(quote.deref())).unwrap()
                }
                _ => {
                    log::info!(
                        "Cannot find token mapping for quote: {}",
                        base_quote[1].to_string()
                    );
                    continue;
                }
            };

            let aggregator = Aggregator {
                address: Hex(&aggregator_address).to_string(),
                description: description.clone(),
                base_asset: Some(Erc20Token {
                    address: base_asset.address,
                    name: base_asset.name,
                    symbol: base_asset.symbol,
                    decimals: base_asset.decimals,
                }),
                quote_asset: Some(Erc20Token {
                    address: quote_asset.address,
                    name: quote_asset.name,
                    symbol: quote_asset.symbol,
                    decimals: quote_asset.decimals,
                }),
                decimals: decimals.to_u64(),
            };

            output.set(
                0,
                keyer::chainlink_aggregator_key(&aggregator.address),
                &aggregator,
            );
        }
    }
}

#[substreams::handlers::store]
fn store_chainlink_price(
    block: eth::Block,
    store: StoreGetProto<Aggregator>,
    output: StoreSetProto<Erc20Price>,
) {
    for log in block.logs() {
        if let Some(event) = chainlink_aggregator::events::AnswerUpdated::match_and_decode(log) {
            let aggregator_address = Hex(log.address()).to_string();

            if let Some(aggregator) =
                store.get_last(keyer::chainlink_aggregator_key(&aggregator_address))
            {
                if ["USD", "DAI", "USDC", "USDT"]
                    .contains(&aggregator.quote_asset.unwrap().symbol.as_str())
                    .not()
                {
                    // TODO: add logic for handling `ETH` quote.
                    continue;
                }

                let token_price = event.current.to_decimal(aggregator.decimals);
                let token_address = aggregator.base_asset.clone().unwrap().address;

                let erc20price = Erc20Price {
                    token: aggregator.base_asset,
                    price_usd: token_price.to_string(),
                    block_number: block.number,
                    source: 1,
                };

                output.set(0, keyer::chainlink_asset_key(&token_address), &erc20price);
            }
        }
    }
}

#[substreams::handlers::store]
fn store_pair_created_events(block: eth::Block, output: StoreSetProto<PairCreatedEvent>) {
    for log in block.logs() {
        if let Some(event) = factory::events::PairCreated::match_and_decode(log) {
            let token0_asset =
                substreams_helper::erc20::get_erc20_token(Hex(event.token0.clone()).to_string())
                    .unwrap();
            let token1_asset =
                substreams_helper::erc20::get_erc20_token(Hex(event.token1.clone()).to_string())
                    .unwrap();

            let pair_created_event = PairCreatedEvent {
                token0: Some(Erc20Token {
                    address: token0_asset.address,
                    name: token0_asset.name,
                    symbol: token0_asset.symbol,
                    decimals: token0_asset.decimals,
                }),
                token1: Some(Erc20Token {
                    address: token1_asset.address,
                    name: token1_asset.name,
                    symbol: token1_asset.symbol,
                    decimals: token1_asset.decimals,
                }),
                pair: Hex(event.pair.clone()).to_string(),
                factory: Hex(log.address()).to_string(),
            };

            output.set(
                0,
                keyer::pair_info_key(&pair_created_event.pair),
                &pair_created_event,
            );
        }
    }
}

#[substreams::handlers::store]
fn store_uniswap_price(
    block: eth::Block,
    chainlink_prices: StoreGetProto<Erc20Price>,
    store: StoreGetProto<PairCreatedEvent>,
    output: StoreSetProto<Erc20Price>,
) {
    for log in block.logs() {
        if let Some(event) = pair::events::Sync::match_and_decode(log) {
            let pair_address = Hex(log.address()).to_string();

            if let Some(pair) = store.get_last(keyer::pair_info_key(&pair_address)) {
                let reserve0 = event
                    .reserve0
                    .to_decimal(pair.token0.clone().unwrap().decimals);
                let reserve1 = event
                    .reserve1
                    .to_decimal(pair.token1.clone().unwrap().decimals);

                // TODO: Add a check for mininmum liquidity threshhold.

                match pair.token0.clone().unwrap().address.as_str() {
                    address if utils::STABLE_COINS.contains(&address) => {
                        let token_price = reserve0.clone() / reserve1.clone();

                        let erc20price = Erc20Price {
                            token: pair.token1.clone(),
                            price_usd: token_price.to_string(),
                            block_number: block.number,
                            source: 2,
                        };

                        output.set(
                            0,
                            keyer::uniswap_asset_key(&pair.token0.clone().unwrap().address),
                            &erc20price,
                        );
                    }
                    address if utils::TOKENS.get("ETH").unwrap().eq(&address) => {
                        let eth_price = match chainlink_prices.get_last(keyer::chainlink_asset_key(
                            &pair.token0.clone().unwrap().address,
                        )) {
                            Some(price) => BigDecimal::from_str(price.price_usd.as_str()).unwrap(),
                            None => BigDecimal::zero(),
                        };

                        if eth_price.eq(&BigDecimal::zero()) {
                            break;
                        };

                        let token_price = (reserve0.clone() / reserve1.clone()) * eth_price;

                        let erc20price = Erc20Price {
                            token: pair.token1.clone(),
                            price_usd: token_price.to_string(),
                            block_number: block.number,
                            source: 2,
                        };

                        output.set(
                            0,
                            keyer::uniswap_asset_key(&pair.token0.clone().unwrap().address),
                            &erc20price,
                        );
                    }
                    _ => {}
                }

                match pair.token1.clone().unwrap().address.as_str() {
                    address if utils::STABLE_COINS.contains(&address) => {
                        let token_price = reserve1.clone() / reserve0.clone();

                        let erc20price = Erc20Price {
                            token: pair.token0.clone(),
                            price_usd: token_price.to_string(),
                            block_number: block.number,
                            source: 2,
                        };

                        output.set(
                            0,
                            keyer::uniswap_asset_key(&pair.token1.clone().unwrap().address),
                            &erc20price,
                        );
                    }
                    address if utils::TOKENS.get("ETH").unwrap().eq(&address) => {
                        let eth_price = match chainlink_prices.get_last(keyer::chainlink_asset_key(
                            &pair.token1.clone().unwrap().address,
                        )) {
                            Some(price) => BigDecimal::from_str(price.price_usd.as_str()).unwrap(),
                            None => BigDecimal::zero(),
                        };

                        if eth_price.eq(&BigDecimal::zero()) {
                            break;
                        };
                        let token_price = (reserve1.clone() / reserve0.clone()) * eth_price;

                        let erc20price = Erc20Price {
                            token: pair.token0.clone(),
                            price_usd: token_price.to_string(),
                            block_number: block.number,
                            source: 2,
                        };

                        output.set(
                            0,
                            keyer::uniswap_asset_key(&pair.token1.clone().unwrap().address),
                            &erc20price,
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}

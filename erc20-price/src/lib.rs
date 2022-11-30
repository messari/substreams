#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

mod keyer;
pub mod utils;

use abi::{chainlink_aggregator, price_feed};
use hex_literal::hex;
use lazy_static::__Deref;
use pb::chainlink::v1::Aggregator;
use pb::erc20_price::v1::{Erc20Price, Erc20Prices};
use std::ops::Not;
use substreams::scalar::BigInt;
use substreams::store::StoreNew;
use substreams::store::{StoreGet, StoreSet};
use substreams::store::{StoreGetProto, StoreSetProto};
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::{Event, Function};
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

        prices.items.push(Erc20Price {
            token: utils::get_erc20_token(Hex(erc20_token.clone()).to_string()),
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

            let base_address = match utils::TOKENS.get(base_quote[0]) {
                Some(v) => String::from(v.deref()),
                _ => {
                    log::info!(
                        "Cannot find token mapping for base: {}",
                        base_quote[0].to_string()
                    );
                    continue;
                }
            };

            let quote_address = match utils::TOKENS.get(base_quote[1]) {
                Some(v) => String::from(v.deref()),
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
                base_asset: utils::get_erc20_token(base_address),
                quote_asset: utils::get_erc20_token(quote_address),
                decimals: decimals.to_u64(),
            };

            output.set(
                0,
                keyer::chainlink_aggregator_key(&Hex(&aggregator_address).to_string()),
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

#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

use crate::abi::access_controlled_aggregator;
use hex_literal::hex;
use pb::chainlink::v1::Aggregator;
use pb::erc20_price::v1::{Erc20Price, Erc20Prices};
use std::collections::{HashMap, HashSet};
use substreams::scalar::BigInt;
use substreams::store::StoreNew;
use substreams::store::{StoreAdd, StoreGet, StoreSet};
use substreams::store::{StoreGetProto, StoreSetProto};
use substreams::{log, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};
use substreams_helper::price;
use substreams_helper::types::Network;

mod keyer;

lazy_static::lazy_static! {
    static ref TOKEN_ADDRESS: HashMap<&'static str, &'static str> = vec![
        ("ETH", "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"),
    ].into_iter().collect();
}

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
            block_number,
            price_usd: token_price.to_string(),
            token_address: Hex(erc20_token.clone()).to_string(),
        });
        log::info!("token {} price {}", Hex(erc20_token), token_price);
    }

    Ok(prices)
}

#[substreams::handlers::map]
fn map_chainlink_eth_price(block: eth::Block) -> Result<Erc20Prices, substreams::errors::Error> {
    let mut prices = Erc20Prices { items: vec![] };

    Ok(prices)
}

#[substreams::handlers::store]
fn store_chainlink_aggregator(block: eth::Block, output: StoreSetProto<Aggregator>) {
    let mut set = HashSet::new();

    for log in block.logs() {
        // Use `OracleAdminUpdated` as a proxy to contract deployment as it's
        // one of the first events emitted by the Aggregator contract
        if access_controlled_aggregator::events::OracleAdminUpdated::match_log(&log.log) {
            let address = Hex(log.address()).to_string();

            // We only need to update the store once as the fields are immutable
            if set.contains(&address) {
                continue;
            }

            let decimals = access_controlled_aggregator::functions::Decimals {}
                .call(log.address().to_vec())
                .unwrap();
            let description = access_controlled_aggregator::functions::Description {}
                .call(log.address().to_vec())
                .unwrap();

            let trimmed = str::replace(&description, "\"", "");
            let base_quote: Vec<&str> = trimmed.split(" / ").collect();
            if base_quote.len() != 2 {
                log::info!("Unexpected description: {}", description);
                continue;
            }

            let aggregator = Aggregator {
                address: address.clone(),
                description: description.clone(),
                base: base_quote[0].to_string(),
                quote: base_quote[1].to_string(),
                decimals: decimals.to_u64(),
            };
            output.set(0, keyer::chainlink_aggregator_key(&address), &aggregator);
            set.insert(address);
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
        if let Some(event) =
            access_controlled_aggregator::events::AnswerUpdated::match_and_decode(log)
        {
            let aggregator_address = Hex(log.address()).to_string();
            let current: BigInt = event.current;
            match store.get_last(keyer::chainlink_aggregator_key(&aggregator_address)) {
                None => continue,
                Some(aggregator) => {
                    if aggregator.quote == "USD"
                        || aggregator.quote == "DAI"
                        || aggregator.quote == "USDC"
                        || aggregator.quote == "USDT"
                    {
                        let price = current.to_decimal(aggregator.decimals);
                        log::info!(
                            "Address: {}, desc: {}, price: {}",
                            aggregator_address,
                            aggregator.description,
                            price
                        );

                        let token_address = TOKEN_ADDRESS.get("ETH").unwrap().to_string();

                        let erc20price = Erc20Price {
                            block_number: block.number,
                            price_usd: price.to_string(),
                            token_address: aggregator.description.clone(),
                        };
                        output.set(
                            0,
                            keyer::chainlink_asset_key(&aggregator.description.clone()),
                            &erc20price,
                        );
                    }
                }
            }
        }
    }
}

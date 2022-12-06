use std::ops::Not;

use lazy_static::__Deref;
use substreams::scalar::BigInt;
use substreams::store::{StoreNew, StoreSet, StoreSetProto};
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Function;

use crate::abi::{chainlink_aggregator, price_feed};
use crate::pb::chainlink::v1::Aggregator;
use crate::pb::erc20::v1::Erc20Token;
use crate::{keyer, utils};

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

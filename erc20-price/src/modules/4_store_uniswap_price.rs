use crate::abi::pair;
use crate::pb::erc20_price::v1::Erc20Price;
use crate::pb::uniswap::v1::PairCreatedEvent;
use crate::{keyer, utils};

use std::str::FromStr;
use substreams::scalar::BigDecimal;
use substreams::store::StoreGet;
use substreams::store::{StoreGetProto, StoreSetProto};
use substreams::store::{StoreNew, StoreSet};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use substreams_helper::types::Source;

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
                    .to_decimal(pair.token0.as_ref().unwrap().decimals);
                let reserve1 = event
                    .reserve1
                    .to_decimal(pair.token1.as_ref().unwrap().decimals);

                // TODO: Add a check for mininmum liquidity threshhold.

                match pair.token0.as_ref().unwrap().address.as_str() {
                    address if utils::STABLE_COINS.contains(&address) => {
                        let token_price = reserve0.clone() / reserve1.clone();

                        let erc20price = Erc20Price {
                            token: pair.token1.clone(),
                            price_usd: token_price.to_string(),
                            block_number: block.number,
                            source: Source::UniswapFeeds as i32,
                        };

                        output.set(
                            log.ordinal(),
                            keyer::uniswap_asset_key(&pair.token0.as_ref().unwrap().address),
                            &erc20price,
                        );
                    }
                    address if utils::TOKENS.get("ETH").unwrap().eq(&address) => {
                        let eth_price = match chainlink_prices.get_last(keyer::chainlink_asset_key(
                            &pair.token0.as_ref().unwrap().address,
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
                            source: Source::UniswapFeeds as i32,
                        };

                        output.set(
                            log.ordinal(),
                            keyer::uniswap_asset_key(&pair.token0.as_ref().unwrap().address),
                            &erc20price,
                        );
                    }
                    _ => {}
                }

                match pair.token1.as_ref().unwrap().address.as_str() {
                    address if utils::STABLE_COINS.contains(&address) => {
                        let token_price = reserve1.clone() / reserve0.clone();

                        let erc20price = Erc20Price {
                            token: pair.token0.clone(),
                            price_usd: token_price.to_string(),
                            block_number: block.number,
                            source: Source::UniswapFeeds as i32,
                        };

                        output.set(
                            log.ordinal(),
                            keyer::uniswap_asset_key(&pair.token1.as_ref().unwrap().address),
                            &erc20price,
                        );
                    }
                    address if utils::TOKENS.get("ETH").unwrap().eq(&address) => {
                        let eth_price = match chainlink_prices.get_last(keyer::chainlink_asset_key(
                            &pair.token1.as_ref().unwrap().address,
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
                            source: Source::UniswapFeeds as i32,
                        };

                        output.set(
                            log.ordinal(),
                            keyer::uniswap_asset_key(&pair.token1.as_ref().unwrap().address),
                            &erc20price,
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}

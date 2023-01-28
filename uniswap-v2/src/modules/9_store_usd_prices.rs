use std::ops::{Div, Mul};
use std::str::FromStr;

use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::StoreNew;
use substreams::store::{StoreGet, StoreGetProto, StoreSet, StoreSetBigDecimal};
use substreams::store::{StoreGetBigInt, StoreGetRaw};
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::pair;
use crate::common::constants::STABLE_COINS;
use crate::pb::erc20_price::v1::Erc20Price;
use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_usd_prices(
    block: eth::Block,
    pool_store: StoreGetProto<Pool>,
    pool_liquidities_store: StoreGetBigInt,
    tokens_whitelist_pools_store: StoreGetRaw,
    pool_native_tvl_store: StoreGetBigInt,
    prices_store: StoreGetProto<Erc20Price>,
    output: StoreSetBigDecimal,
) {
    for log in block.logs() {
        if let Some(_) = pair::events::Sync::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();

            if let Some(pool) =
                pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                let input_tokens = pool.input_tokens.unwrap().items;

                for token in input_tokens {
                    let s;
                    let token_address = token.address;

                    let token_whitelist_pools = match tokens_whitelist_pools_store
                        .get_last(StoreKey::TokenWhitelist.get_unique_pool_key(&token_address))
                    {
                        Some(bytes) => {
                            s = String::from_utf8(bytes.to_vec()).unwrap();

                            s.split(";")
                                .filter(|&x| !x.is_empty())
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()
                        }
                        None => {
                            continue;
                        }
                    };

                    let token_usd_price = find_usd_price_per_token(
                        &token_address,
                        token_whitelist_pools,
                        &pool_store,
                        &pool_liquidities_store,
                        &pool_native_tvl_store,
                        &prices_store,
                    );

                    output.set(
                        log.ordinal(),
                        StoreKey::TokenPrice.get_unique_token_key(&token_address),
                        &token_usd_price,
                    )
                }
            }
        }
    }
}

pub fn find_usd_price_per_token(
    token_address: &String,
    whitelisted_pools: Vec<String>,
    pool_store: &StoreGetProto<Pool>,
    pool_liquidities_store: &StoreGetBigInt,
    token_native_tvl_store: &StoreGetBigInt,
    prices_store: &StoreGetProto<Erc20Price>,
) -> BigDecimal {
    if STABLE_COINS.contains(&token_address.as_str()) {
        return BigDecimal::one().with_prec(100);
    }

    let mut price_so_far = BigDecimal::zero().with_prec(100);

    for pool_address in whitelisted_pools.iter() {
        let pool = match pool_store.get_last(StoreKey::Pool.get_unique_pool_key(pool_address)) {
            Some(pool) => pool,
            None => continue,
        };

        let pool_liquidity = match pool_liquidities_store
            .get_last(StoreKey::PoolOutputTokenSupply.get_unique_pool_key(pool_address))
        {
            Some(value) => value,
            None => BigInt::zero(),
        };

        if pool_liquidity.le(&BigInt::zero()) {
            continue;
        }

        let input_tokens = pool.input_tokens.unwrap().items;

        let token0 = &input_tokens[0];
        let token0_native_tvl = match token_native_tvl_store.get_last(
            StoreKey::InputTokenBalance.get_pool_token_balance_key(pool_address, &token0.address),
        ) {
            Some(tvl) => tvl.to_decimal(token0.decimals),
            None => BigDecimal::zero().with_prec(100),
        };
        log::info!(
            "token0: {}, Decimal: {}, TVL: {}",
            &token0.name,
            &token0.decimals,
            &token0_native_tvl
        );

        let token1 = &input_tokens[1];
        let token1_native_tvl = match token_native_tvl_store.get_last(
            StoreKey::InputTokenBalance.get_pool_token_balance_key(pool_address, &token1.address),
        ) {
            Some(tvl) => tvl.to_decimal(token1.decimals),
            None => BigDecimal::zero().with_prec(100),
        };
        log::info!(
            "token1: {}, Decimal: {}, TVL: {}",
            &token1.name,
            &token1.decimals,
            &token1_native_tvl
        );

        if &token0.address == token_address {
            // TODO: Add checks for mininmum liquidity threshold
            if STABLE_COINS.contains(&token1.address.as_str()) {
                price_so_far = token1_native_tvl
                    .clone()
                    .div(token0_native_tvl.clone())
                    .with_prec(100);
            } else {
                let token1_usd_price: BigDecimal =
                    match prices_store.get_last(format!("chainlink_price:{}", &token1.address)) {
                        Some(price) => BigDecimal::from_str(price.price_usd.as_str()).unwrap(),
                        None => BigDecimal::zero(),
                    };
                price_so_far = token1_native_tvl
                    .clone()
                    .div(token0_native_tvl.clone())
                    .mul(token1_usd_price)
                    .with_prec(100);
            }
        }

        if &token1.address == token_address {
            // TODO: Add checks for mininmum liquidity threshold
            if STABLE_COINS.contains(&token0.address.as_str()) {
                price_so_far = token0_native_tvl
                    .clone()
                    .div(token1_native_tvl.clone())
                    .with_prec(100);
            } else {
                let token0_usd_price =
                    match prices_store.get_last(format!("chainlink_price:{}", &token0.address)) {
                        Some(price) => BigDecimal::from_str(price.price_usd.as_str()).unwrap(),
                        None => BigDecimal::zero(),
                    };

                price_so_far = token0_native_tvl
                    .clone()
                    .div(token1_native_tvl.clone())
                    .mul(token0_usd_price)
                    .with_prec(100);
            }
        }
    }

    return price_so_far;
}

use std::ops::{Div, Mul};
use std::str::FromStr;

use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreGet, StoreGetProto, StoreNew, StoreSetBigDecimal};
use substreams::store::{StoreGetBigDecimal, StoreSet};

use crate::common::constants;
use crate::pb::uniswap::v2::event::Type::SyncType;
use crate::pb::uniswap::v2::{Events, Pool};
use crate::store_key::StoreKey;
use crate::utils;

#[substreams::handlers::store]
pub fn store_usd_prices(
    pool_store: StoreGetProto<Pool>,
    pool_events: Events,
    native_prices_store: StoreGetBigDecimal,
    output_store: StoreSetBigDecimal,
) {
    let min_liquidity_threshold = BigDecimal::from_str("5000").unwrap();

    for event in pool_events.events {
        match event.r#type.unwrap() {
            SyncType(sync) => {
                let ordinal = event.log_ordinal;

                let pool =
                    pool_store.must_get_last(StoreKey::Pool.get_unique_pool_key(&event.pool));

                let token0 = pool.token0_ref();
                let token1 = pool.token1_ref();

                let reserve0 = BigInt::from_str(sync.reserve0.as_str())
                    .unwrap()
                    .to_decimal(token0.decimals);
                let reserve1 = BigInt::from_str(sync.reserve1.as_str())
                    .unwrap()
                    .to_decimal(token1.decimals);

                if reserve0.is_zero() || reserve1.is_zero() {
                    continue;
                }

                let token0_derived_price = reserve1.clone().div(reserve0.clone());
                let token1_derived_price = reserve0.clone().div(reserve1.clone());

                if utils::is_pricing_asset(&token1.address) {
                    let mut token0_price = BigDecimal::zero();

                    let token1_price =
                        get_price_from_native_store(ordinal, &token1.address, &native_prices_store);

                    if reserve1
                        .mul(token1_price.clone())
                        .ge(&min_liquidity_threshold)
                    {
                        token0_price = token0_derived_price * token1_price;
                    }

                    output_store.set(
                        ordinal,
                        StoreKey::TokenPrice.get_unique_pool_key(&token0.address),
                        &token0_price,
                    );
                }

                if utils::is_pricing_asset(&token0.address) {
                    let mut token1_price = BigDecimal::zero();

                    let token0_price =
                        get_price_from_native_store(ordinal, &token0.address, &native_prices_store);

                    if reserve0
                        .mul(token0_price.clone())
                        .ge(&min_liquidity_threshold)
                    {
                        token1_price = token1_derived_price * token0_price
                    }

                    output_store.set(
                        ordinal,
                        StoreKey::TokenPrice.get_unique_pool_key(&token1.address),
                        &token1_price,
                    );
                }
            }
            _ => {}
        }
    }
}

fn get_price_from_native_store(
    ordinal: u64,
    token_address: &String,
    native_prices_store: &StoreGetBigDecimal,
) -> BigDecimal {
    if token_address.eq(constants::WETH_ADDRESS) {
        return get_weth_price_in_usd(ordinal, native_prices_store);
    }
    if constants::STABLE_COINS.contains(&token_address.as_str()) {
        return BigDecimal::one();
    }

    for address in constants::PAIR_COINS.into_iter() {
        let whitelisted_token = address;

        let mut token_price = native_prices_store
            .get_at(
                ordinal,
                StoreKey::TokenPrice.get_unique_pair_key(token_address, whitelisted_token),
            )
            .unwrap_or(BigDecimal::zero());

        if token_price.le(&BigDecimal::zero()) {
            continue;
        }

        if whitelisted_token.eq(constants::WETH_ADDRESS) {
            token_price = token_price * get_weth_price_in_usd(ordinal, native_prices_store)
        }

        if min_pool_liquidity_check(
            ordinal,
            token_address,
            &whitelisted_token,
            native_prices_store,
        ) {
            return token_price;
        }
    }

    return BigDecimal::zero();
}

fn min_pool_liquidity_check(
    ordinal: u64,
    address1: &str,
    address2: &str,
    native_prices_store: &StoreGetBigDecimal,
) -> bool {
    let mut amount_locked_usd = native_prices_store
        .get_at(
            ordinal,
            StoreKey::TokenBalance.get_unique_pair_key(address1, address2),
        )
        .unwrap_or(BigDecimal::zero());

    if address2.eq(constants::WETH_ADDRESS) {
        amount_locked_usd = amount_locked_usd * get_weth_price_in_usd(ordinal, native_prices_store);
    }

    amount_locked_usd.ge(&BigDecimal::from_str("5000").unwrap())
}

fn get_weth_price_in_usd(ordinal: u64, native_prices_store: &StoreGetBigDecimal) -> BigDecimal {
    native_prices_store
        .get_at(
            ordinal,
            StoreKey::TokenPrice
                .get_unique_pair_key(constants::WETH_ADDRESS, constants::USDC_ADDRESS),
        )
        .unwrap_or(BigDecimal::zero())
}

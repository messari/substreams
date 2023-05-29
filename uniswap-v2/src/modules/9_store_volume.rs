use std::ops::Mul;

use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreAdd, StoreAddBigDecimal, StoreGet, StoreGetBigDecimal, StoreNew};

use crate::common::constants;
use crate::pb::erc20::v1::Erc20Token;
use crate::pb::uniswap::v2::event::Type::SwapType;
use crate::pb::uniswap::v2::Events;
use crate::store_key::StoreKey;
use crate::utils;

#[substreams::handlers::store]
pub fn store_volume(
    swap_events: Events,
    prices_store: StoreGetBigDecimal,
    output_store: StoreAddBigDecimal,
) {
    for event in swap_events.events {
        match event.r#type.unwrap() {
            SwapType(swap_event) => {
                let pool_address = event.pool;
                let ordinal = event.log_ordinal;

                let token_in = swap_event.token_in.unwrap();
                let token_out = swap_event.token_out.unwrap();

                let volume = get_tracked_volume_usd(
                    ordinal,
                    &token_in,
                    &token_out,
                    &swap_event.amount_in,
                    &swap_event.amount_out,
                    &prices_store,
                );

                output_store.add(
                    ordinal,
                    StoreKey::VolumeByTokenUSD
                        .get_unique_pair_key(&pool_address, &token_in.address),
                    &volume.clone(),
                );
                output_store.add(
                    ordinal,
                    StoreKey::VolumeByTokenUSD
                        .get_unique_pair_key(&pool_address, &token_out.address),
                    &volume.clone(),
                );

                output_store.add(
                    ordinal,
                    StoreKey::Volume.get_unique_pool_key(&pool_address),
                    &volume.clone(),
                );
            }
            _ => {}
        }
    }
}

fn get_tracked_volume_usd(
    ordinal: u64,
    token_in: &Erc20Token,
    token_out: &Erc20Token,
    amount_in: &String,
    amount_out: &String,
    prices_store: &StoreGetBigDecimal,
) -> BigDecimal {
    let token_in_price = utils::get_token_price(ordinal, prices_store, &token_in.address);
    let token_out_price = utils::get_token_price(ordinal, prices_store, &token_out.address);

    let amount_in = BigInt::try_from(amount_in)
        .unwrap()
        .to_decimal(token_in.decimals);
    let amount_out = BigInt::try_from(amount_out)
        .unwrap()
        .to_decimal(token_out.decimals);

    let amount_in_usd = amount_in.mul(token_in_price);
    let amount_out_usd = amount_out.mul(token_out_price);

    if constants::WHITELIST_TOKENS.contains(&token_in.address.as_str())
        && constants::WHITELIST_TOKENS.contains(&token_out.address.as_str())
    {
        return (amount_in_usd + amount_out_usd) / BigDecimal::from(2);
    }

    if constants::WHITELIST_TOKENS.contains(&token_in.address.as_str()) {
        return amount_in_usd;
    }

    if constants::WHITELIST_TOKENS.contains(&token_out.address.as_str()) {
        return amount_out_usd;
    }

    return BigDecimal::zero();
}

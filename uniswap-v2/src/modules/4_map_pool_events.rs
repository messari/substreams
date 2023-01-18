use crate::pb::uniswap::v2::event::Type::{Deposit, Swap, Withdraw};
use substreams::errors::Error;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigInt, Deltas, StoreGetBigDecimal};
use substreams::store::{StoreGet, StoreGetProto};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::pair;
use crate::pb::uniswap::v2::{
    Deposit as DepositEvent, Swap as SwapEvent, Withdraw as WithdrawEvent,
};
use crate::pb::uniswap::v2::{Event as PoolEvent, Events, Pool};
use crate::balance_updater::get_user_balance_diff;
use crate::store_key::StoreKey;
use crate::utils;

#[substreams::handlers::map]
pub fn map_pool_events(
    block: eth::Block,
    pool_store: StoreGetProto<Pool>,
    balance_deltas: Deltas<DeltaBigInt>,
    usd_price_store: StoreGetBigDecimal,
) -> Result<Events, Error> {
    let mut events = vec![];

    for log in block.logs() {
        if let Some(mint_event) = pair::events::Mint::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();
            let user_address = Hex(&log.receipt.transaction.from).to_string();

            if let Some(pool) =
                pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                let amount0_usd = match usd_price_store.get_last(
                    StoreKey::TokenPrice.get_unique_token_key(pool.clone().token0_address()),
                ) {
                    Some(price) => {
                        mint_event
                            .amount0
                            .clone()
                            .to_decimal(pool.clone().token0_ref().decimals)
                            * price
                    }
                    None => BigDecimal::zero(),
                };

                let amount1_usd = match usd_price_store.get_last(
                    StoreKey::TokenPrice.get_unique_token_key(pool.clone().token1_address()),
                ) {
                    Some(price) => {
                        mint_event
                            .amount1
                            .clone()
                            .to_decimal(pool.clone().token1_ref().decimals)
                            * price
                    }
                    None => BigDecimal::zero(),
                };

                let output_token_minted_amount =
                    get_user_balance_diff(&balance_deltas, &pool_address, &user_address);

                events.push(PoolEvent {
                    hash: Hex(&log.receipt.transaction.hash).to_string(),
                    log_index: log.index() as i64,
                    log_ordinal: log.ordinal() as i64,
                    block_number: block.number as i64,
                    timestamp: block.timestamp_seconds() as i64,
                    to: pool.address.clone(),
                    from: user_address,
                    pool: pool.address.clone(),
                    r#type: Some(Deposit(DepositEvent {
                        input_token_amounts: vec![mint_event.amount0, mint_event.amount1]
                            .iter()
                            .map(|x| x.to_u64())
                            .collect(),
                        output_token_amount: output_token_minted_amount,
                        amount_usd: (amount0_usd + amount1_usd).to_string(),
                    })),
                });
            }
        } else if let Some(burn_event) = pair::events::Burn::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();
            let user_address = Hex(&log.receipt.transaction.from).to_string();

            if let Some(pool) =
                pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                let output_token_minted_amount =
                    get_user_balance_diff(&balance_deltas, &pool_address, &user_address);

                let amount0_usd = match usd_price_store.get_last(
                    StoreKey::TokenPrice.get_unique_token_key(pool.clone().token0_address()),
                ) {
                    Some(price) => {
                        burn_event
                            .amount0
                            .clone()
                            .to_decimal(pool.clone().token0_ref().decimals)
                            * price
                    }
                    None => BigDecimal::zero(),
                };

                let amount1_usd = match usd_price_store.get_last(
                    StoreKey::TokenPrice.get_unique_token_key(pool.clone().token1_address()),
                ) {
                    Some(price) => {
                        burn_event
                            .amount1
                            .clone()
                            .to_decimal(pool.clone().token1_ref().decimals)
                            * price
                    }
                    None => BigDecimal::zero(),
                };

                events.push(PoolEvent {
                    hash: Hex(&log.receipt.transaction.hash).to_string(),
                    log_index: log.index() as i64,
                    log_ordinal: log.ordinal() as i64,
                    block_number: block.number as i64,
                    timestamp: block.timestamp_seconds() as i64,
                    to: pool.address.clone(),
                    from: user_address,
                    pool: pool.address.clone(),
                    r#type: Some(Withdraw(WithdrawEvent {
                        input_token_amounts: vec![burn_event.amount0, burn_event.amount1]
                            .iter()
                            .map(|x| x.to_u64())
                            .collect(),
                        output_token_amount: output_token_minted_amount,
                        amount_usd: (amount0_usd + amount1_usd).to_string(),
                    })),
                });
            }
        } else if let Some(swap_event) = pair::events::Swap::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();
            let user_address = Hex(&log.receipt.transaction.from).to_string();

            if let Some(pool) =
                pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                let pool_input_tokens = pool.clone().input_tokens.unwrap().items;

                let token0_usd_price = match usd_price_store.get_last(
                    StoreKey::TokenPrice.get_unique_token_key(pool.clone().token0_address()),
                ) {
                    Some(price) => price,
                    None => BigDecimal::zero(),
                };

                let token1_usd_price = match usd_price_store.get_last(
                    StoreKey::TokenPrice.get_unique_token_key(pool.clone().token1_address()),
                ) {
                    Some(price) => price,
                    None => BigDecimal::zero(),
                };

                let swapped_tokens = if swap_event.amount0_out.gt(&BigInt::zero()) {
                    utils::SwappedTokens {
                        token_in: Some(pool_input_tokens[1].clone()),
                        amount_in: swap_event.amount1_in.to_u64() - swap_event.amount1_out.to_u64(),
                        amount_in_usd: BigInt::from(
                            swap_event.amount1_in.to_u64() - swap_event.amount1_out.to_u64(),
                        )
                        .to_decimal(pool.clone().token1_ref().decimals)
                            * token1_usd_price,
                        token_out: Some(pool_input_tokens[0].clone()),
                        amount_out: swap_event.amount0_out.to_u64()
                            - swap_event.amount0_in.to_u64(),
                        amount_out_usd: BigInt::from(
                            swap_event.amount0_out.to_u64() - swap_event.amount0_in.to_u64(),
                        )
                        .to_decimal(pool.clone().token0_ref().decimals)
                            * token0_usd_price,
                    }
                } else {
                    utils::SwappedTokens {
                        token_in: Some(pool_input_tokens[0].clone()),
                        amount_in: swap_event.amount0_in.to_u64() - swap_event.amount0_out.to_u64(),
                        amount_in_usd: BigInt::from(
                            swap_event.amount0_in.to_u64() - swap_event.amount0_out.to_u64(),
                        )
                        .to_decimal(pool.clone().token0_ref().decimals)
                            * token0_usd_price,
                        token_out: Some(pool_input_tokens[1].clone()),
                        amount_out: swap_event.amount1_out.to_u64()
                            - swap_event.amount1_in.to_u64(),
                        amount_out_usd: BigInt::from(
                            swap_event.amount1_out.to_u64() - swap_event.amount1_in.to_u64(),
                        )
                        .to_decimal(pool.clone().token1_ref().decimals)
                            * token1_usd_price,
                    }
                };

                events.push(PoolEvent {
                    hash: Hex(&log.receipt.transaction.hash).to_string(),
                    log_index: log.index() as i64,
                    log_ordinal: log.ordinal() as i64,
                    block_number: block.number as i64,
                    timestamp: block.timestamp_seconds() as i64,
                    to: pool.address.clone(),
                    from: user_address,
                    pool: pool.address.clone(),
                    r#type: Some(Swap(SwapEvent {
                        token_in: swapped_tokens.token_in,
                        amount_in: swapped_tokens.amount_in,
                        amount_in_usd: swapped_tokens.amount_in_usd.to_string(),
                        token_out: swapped_tokens.token_out,
                        amount_out: swapped_tokens.amount_out,
                        amount_out_usd: swapped_tokens.amount_out_usd.to_string(),
                    })),
                });
            }
        }
    }

    Ok(Events { events })
}

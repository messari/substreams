use substreams::scalar::BigInt;
use substreams::store::StoreGet;
use substreams::Hex;
use substreams::{errors::Error, store::StoreGetProto};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::pair as PairContract;
use crate::pb::erc20::v1::Erc20Token;
use crate::pb::uniswap::v2::{
    event::Type::Swap, Event as PoolEvent, Events, Pool, Swap as SwapEvent,
};
use crate::store_key::StoreKey;

pub struct SwappedTokens {
    pub token_in: Option<Erc20Token>,
    pub amount_in: BigInt,
    pub token_out: Option<Erc20Token>,
    pub amount_out: BigInt,
}

#[substreams::handlers::map]
pub fn map_pool_swaps(block: eth::Block, pool_store: StoreGetProto<Pool>) -> Result<Events, Error> {
    let mut events = vec![];

    for log in block.logs() {
        if let Some(swap_event) = PairContract::events::Swap::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();

            if let Some(pool) =
                pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                let swapped_tokens = if swap_event.amount0_out.gt(BigInt::zero().as_ref()) {
                    SwappedTokens {
                        token_in: Some(pool.token1_ref()),
                        amount_in: swap_event.amount1_in - swap_event.amount1_out,
                        token_out: Some(pool.token0_ref()),
                        amount_out: swap_event.amount0_out - swap_event.amount0_in,
                    }
                } else {
                    SwappedTokens {
                        token_in: Some(pool.token0_ref()),
                        amount_in: swap_event.amount0_in - swap_event.amount0_out,
                        token_out: Some(pool.token1_ref()),
                        amount_out: swap_event.amount1_out - swap_event.amount1_in,
                    }
                };

                events.push(PoolEvent {
                    pool: pool.address.clone(),
                    to: pool.address.clone(),
                    from: Hex(&log.receipt.transaction.from).to_string(),
                    hash: Hex(&log.receipt.transaction.hash).to_string(),
                    log_index: log.index() as i64,
                    log_ordinal: log.ordinal() as i64,
                    timestamp: block.timestamp_seconds() as i64,
                    block_number: block.number as i64,
                    r#type: Some(Swap(SwapEvent {
                        token_in: swapped_tokens.token_in,
                        amount_in: swapped_tokens.amount_in.to_string(),
                        token_out: swapped_tokens.token_out,
                        amount_out: swapped_tokens.amount_out.to_string(),
                    })),
                })
            }
        }
    }

    Ok(Events { events })
}

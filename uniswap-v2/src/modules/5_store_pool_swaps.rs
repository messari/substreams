use substreams::scalar::BigInt;
use substreams::store::{StoreGet, StoreNew, StoreSet};
use substreams::store::{StoreGetProto, StoreSetProto};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::pair;
use crate::pb::dex_amm::v1::usage_event::EventType;
use crate::pb::dex_amm::v1::{Pool, Swap, Token, UsageEvent};
use crate::store_key::StoreKey;

struct SwappedTokens {
    token_in: Option<Token>,
    amount_in: u64,
    token_out: Option<Token>,
    amount_out: u64,
}

#[substreams::handlers::store]
pub fn store_pool_swaps(
    block: eth::Block,
    pool_store: StoreGetProto<Pool>,
    output: StoreSetProto<UsageEvent>,
) {
    for log in block.logs() {
        if let Some(event) = pair::events::Swap::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();
            let user_address = Hex(&log.receipt.transaction.from).to_string();

            if let Some(pool) =
                pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                let swapped_tokens = if event.amount0_out.gt(&BigInt::zero()) {
                    SwappedTokens {
                        token_in: Some(pool.input_tokens[1].clone()),
                        amount_in: event.amount1_in.to_u64() - event.amount1_out.to_u64(),
                        token_out: Some(pool.input_tokens[0].clone()),
                        amount_out: event.amount0_out.to_u64() - event.amount0_in.to_u64(),
                    }
                } else {
                    SwappedTokens {
                        token_in: Some(pool.input_tokens[0].clone()),
                        amount_in: event.amount0_in.to_u64() - event.amount0_out.to_u64(),
                        token_out: Some(pool.input_tokens[1].clone()),
                        amount_out: event.amount1_out.to_u64() - event.amount1_in.to_u64(),
                    }
                };

                let swap_event = UsageEvent {
                    hash: Hex(&log.receipt.transaction.hash).to_string(),
                    log_index: log.index() as u64,
                    to: pool_address.clone(),
                    from: user_address,
                    block_number: block.number,
                    timestamp: block.timestamp_seconds(),
                    pool: pool_address.clone(),
                    event_type: Some(EventType::Swap(Swap {
                        token_in: swapped_tokens.token_in,
                        amount_in: swapped_tokens.amount_in,
                        token_out: swapped_tokens.token_out,
                        amount_out: swapped_tokens.amount_out,

                        // TODO: Add amount_in_USD, amount_out_USD field
                        ..Default::default()
                    })),

                    // TODO: Add protocol field
                    ..Default::default()
                };

                output.set(
                    log.ordinal(),
                    StoreKey::SwapEvent.get_unique_event_key(&pool_address, &swap_event.hash),
                    &swap_event,
                )
            }
        }
    }
}

use substreams::store::{DeltaBigInt, Deltas, StoreNew};
use substreams::store::{StoreGet, StoreSet};
use substreams::store::{StoreGetProto, StoreSetProto};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::pair;
use crate::pb::dex_amm::v1::usage_event::EventType;
use crate::pb::dex_amm::v1::{Pool, UsageEvent, Withdraw};
use crate::pool_balance_retriever;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_pool_withdraws(
    block: eth::Block,
    pool_store: StoreGetProto<Pool>,
    balance_deltas: Deltas<DeltaBigInt>,
    output: StoreSetProto<UsageEvent>,
) {
    for log in block.logs() {
        if let Some(event) = pair::events::Burn::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();
            let user_address = Hex(&log.receipt.transaction.from).to_string();

            if let Some(pool) =
                pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                let output_token_amount = pool_balance_retriever::get_user_balance_diff(
                    &balance_deltas,
                    &pool_address,
                    &user_address,
                );

                let withdraw_event = UsageEvent {
                    hash: Hex(&log.receipt.transaction.hash).to_string(),
                    log_index: log.index() as u64,
                    to: pool_address.clone(),
                    from: user_address,
                    block_number: block.number,
                    timestamp: block.timestamp_seconds(),
                    pool: pool_address.clone(),
                    event_type: Some(EventType::Withdraw(Withdraw {
                        input_tokens: pool.input_tokens,
                        input_token_amounts: vec![event.amount0, event.amount1]
                            .iter()
                            .map(|x| x.to_u64())
                            .collect(),
                        output_token: pool.output_token,
                        output_token_amount: output_token_amount,

                        // TODO: Add amount_usd field
                        ..Default::default()
                    })),

                    // TODO: Add protocol field
                    ..Default::default()
                };

                output.set(
                    log.ordinal(),
                    StoreKey::WithdrawEvent
                        .get_unique_event_key(&pool_address, &withdraw_event.hash),
                    &withdraw_event,
                );
            }
        }
    }
}

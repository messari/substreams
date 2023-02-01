pub mod pb;

use substreams::Hex;
use substreams_ethereum::pb::eth as pbeth;
use substreams_helper::pb::erc20::v1 as proto;
use substreams_helper::token as token_helper;
use substreams_helper::token::ETH_ADDRESS;

#[substreams::handlers::map]
fn map_balances(
    block: pbeth::v2::Block,
) -> Result<proto::TransferEvents, substreams::errors::Error> {
    let mut transfers = vec![];

    for transaction in &block.transaction_traces {
        let mut balance_changes = vec![];
        for call in &transaction.calls {
            for balance_change in &call.balance_changes {
                balance_changes.push(proto::TokenBalance {
                    log_ordinal: Some(balance_change.ordinal),
                    token: token_helper::get_eth_token(),
                    address: Hex::encode(&balance_change.address),
                    old_balance: Some(token_helper::bigint_to_string(
                        balance_change.old_value.clone(),
                    )),
                    new_balance: token_helper::bigint_to_string(balance_change.new_value.clone()),
                    reason: Some(balance_change.reason),
                });
            }
        }
        transfers.push(proto::TransferEvent {
            tx_hash: Hex::encode(&transaction.hash),
            block_number: block.number,
            timestamp: block
                .header
                .as_ref()
                .unwrap()
                .timestamp
                .as_ref()
                .unwrap()
                .seconds as u64,
            log_index: transaction.index,
            log_ordinal: None,
            token_address: ETH_ADDRESS.to_string(),
            from: Hex::encode(&transaction.from),
            to: Hex::encode(&transaction.to),
            amount: token_helper::bigint_to_string(transaction.value.clone()),
            balance_changes: balance_changes,
        });
    }

    Ok(proto::TransferEvents { items: transfers })
}

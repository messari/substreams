#[rustfmt::skip]
pub mod pb;

use num_bigint;
use pb::evm_token::v1 as token;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_ethereum::pb::eth as pbeth;
use substreams_helper::token as token_helper;

#[substreams::handlers::map]
fn map_balances(block: pbeth::v2::Block) -> Result<token::Transfers, substreams::errors::Error> {
    let mut transfers = vec![];

    for transaction in &block.transaction_traces {
        let mut balance_changes = vec![];
        for call in &transaction.calls {
            for balance_change in &call.balance_changes {
                balance_changes.push(token::TokenBalance {
                    log_ordinal: balance_change.ordinal,
                    token: token_helper::get_eth_token(),
                    address: Hex(&balance_change.address).to_string(),
                    old_balance: token_helper::bigint_to_string(balance_change.old_value.clone()),
                    new_balance: token_helper::bigint_to_string(balance_change.new_value.clone()),
                    reason: Some(balance_change.reason),
                });
            }
        }
        transfers.push(token::Transfer {
            tx_hash: Hex(&transaction.hash).to_string(),
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
            token: token_helper::get_eth_token(),
            to: Hex(&transaction.to).to_string(),
            from: Hex(&transaction.from).to_string(),
            amount: token_helper::bigint_to_string(transaction.value.clone()),
            amount_usd: None,
            balance_changes: balance_changes,
        });
    }

    Ok(token::Transfers { items: transfers })
}

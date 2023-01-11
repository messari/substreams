#[rustfmt::skip]
pub mod pb;

use crate::pb::evm_token::v1::Token;
use num_bigint;
use pb::evm_token::v1 as token;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_ethereum::pb::eth as pbeth;
use substreams_helper::token::get_eth_token;


#[substreams::handlers::map]
fn map_balances(block: pbeth::v2::Block) -> Result<token::Accounts, substreams::errors::Error> {
    let mut accounts = vec![];

    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for balance_change in &call.balance_changes {
                // TODO: replace this with substreams::scalar::BigInt once the wrapper is integrated
                let new_value = balance_change
                    .new_value
                    .as_ref()
                    .map(|value| {
                        num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &value.bytes)
                            .into()
                    })
                    .unwrap_or(BigInt::zero());
                let new_token_balance = vec![token::TokenBalance {
                    token: get_eth_token(),
                    balance: new_value.to_string(),
                    block_number: block.number,
                    timestamp: block.header.timestamp
                }];
                let account = token::Account {
                    address: Hex(&balance_change.address).to_string(),
                    balances: new_token_balance,
                };
                accounts.push(account);
            }
        }
    }

    Ok(token::Accounts { items: accounts })
}

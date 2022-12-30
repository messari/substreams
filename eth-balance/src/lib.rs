#[rustfmt::skip]
pub mod pb;

use crate::pb::token::v1::Token;
use num_bigint;
use pb::token::v1 as token;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_ethereum::pb::eth as pbeth;

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
                    token: Some(get_eth_token()),
                    balance: new_value.to_string(),
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

//////////////////////////
//// Helper Functions ////
//////////////////////////

fn get_eth_token() -> token::Token {
    let eth_token = Token {
        address: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE".to_string(), // TODO: do we need to append "address: "?
        name: "Ethereum".to_string(),
        symbol: "ETH".to_string(),
        decimals: 18 as u64,
    };
    // let tokens = vec![eth_token.clone()]; // TODO: does this set the Tokens protbuf definition

    eth_token
}

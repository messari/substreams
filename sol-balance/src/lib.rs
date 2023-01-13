use crate::pb::sol::v1 as token; // TODO: I can't get this import to work :(
use bs58;
use prost::Message;
use substreams::store::StoreSet;
use substreams_solana::pb;
use substreams_solana::pb::sol as solana;

#[substreams::handlers::map]
fn map_balances(block: solana::v1::Block) -> Result<token::Accounts, substreams::errors::Error> {
    let accounts = vec![];
    for tx in block.transactions {
        if let Some(meta) = tx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            for i in 0..meta.post_balances.len() {
                let new_balance = &meta.post_balances[i].encode_to_vec();
                let new_token_balance = vec![token::Balance {
                    token: get_sol_token(),
                    balance: new_balance,
                    block_number: block.block_height.block_height,
                    timestamp: block.block_time.timestamp,
                }];
                // TODO: Not sure if this is the right place to get the address
                // AND loaded_writeable_addresses is not being built in the proto stucts
                // let account_id = bs58::encode(&meta.loaded_writable_addresses[i]).into_string();
                let account = token::Account {
                    account_id: "0x0",
                    balances: new_token_balance,
                };
                accounts.push(account);
            }
        }
    }

    Ok(token::Accounts { items: accounts })
}

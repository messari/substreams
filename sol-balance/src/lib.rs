use bs58;
use substreams_helper::pb::sol_token::v1 as proto;
use substreams_helper::token::get_sol_token;
use substreams_solana::pb::sol as solana;

#[substreams::handlers::map]
fn map_balances(
    block: solana::v1::Block,
) -> Result<proto::BalanceChanges, substreams::errors::Error> {
    let mut balances = vec![];
    for tx in block.transactions {
        if let Some(meta) = tx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            if let Some(transaction) = tx.transaction {
                if let Some(msg) = transaction.message {
                    for i in 0..meta.post_balances.len() {
                        let account_id: String;
                        if i < msg.account_keys.len() {
                            account_id = bs58::encode(&msg.account_keys[i]).into_string();
                        } else {
                            account_id = "TODO".to_string();
                            // TODO: use msg.address_table_lookup table, but it is not avail
                        }
                        balances.push(proto::TokenBalance {
                            token: get_sol_token(),
                            transaction_id: bs58::encode(&transaction.signatures[0]).into_string(),
                            block_height: block.block_height.as_ref().unwrap().block_height,
                            address: account_id,
                            pre_balance: meta.pre_balances[i].to_string(),
                            post_balance: meta.post_balances[i].to_string(),
                        });
                    }
                }
            }
        }
    }

    Ok(proto::BalanceChanges { items: balances })
}

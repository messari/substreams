use substreams_solana::pb::sol as solana;
use substreams_helper::token::get_sol_token;
use substreams_helper::pb::sol_token::v1 as proto;

#[substreams::handlers::map]
fn map_balances(block: solana::v1::Block) -> Result<proto::BalanceChanges, substreams::errors::Error> {
    let mut balances = vec![];
    for tx in block.transactions {
        if let Some(meta) = tx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            for i in 0..meta.post_balances.len() {
                balances.push(proto::TokenBalance {
                    token: get_sol_token(),
                    block_height: block.block_height.as_ref().unwrap().block_height,
                    address: "TODO".to_string(),
                    pre_balance: meta.pre_balances[i].to_string(),
                    post_balance: meta.post_balances[i].to_string()
                });
            }
        }
    }

    Ok(proto::BalanceChanges { items: balances})
}

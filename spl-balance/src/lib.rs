pub mod pb;

use bs58;
use num_bigint;
use pb::sol_token::v1 as proto;
use substreams::scalar::BigInt;
use substreams::store::{StoreNew, StoreSet, StoreSetRaw};
use substreams::{log, store, Hex};
use substreams_solana::pb::sol as solana;

// Map SPL token balance changes
#[substreams::handlers::map]
fn map_balances(
    block: solana::v1::Block,
) -> Result<proto::BalanceChanges, substreams::errors::Error> {
    log::info!("extracting SPL balance changes");
    let mut balance_changes = proto::BalanceChanges { items: vec![] };

    for trx in block.transactions {
        if let Some(meta) = trx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            if let Some(transaction) = trx.transaction {
                log::info!("{}", bs58::encode(&transaction.signatures[0]).into_string())
            }

            for i in 0..meta.pre_token_balances.len() {
                let pre_balance = &meta.pre_token_balances[i];
                let post_balance = &meta.post_token_balances[i];
                let mut pre_balance_amount = "";
                let mut post_balance_amount = "";
                // pre.owner = user address
                // pre.mint = token address
                if let Some(pre_token_amount) = &pre_balance.ui_token_amount {
                    pre_balance_amount = &pre_token_amount.amount;
                }
                if let Some(post_token_amount) = &post_balance.ui_token_amount {
                    post_balance_amount = &post_token_amount.amount;
                }

                log::info!("{} {} {}", pre_balance_amount, post_balance_amount, meta.pre_token_balances.len());

                // balance_changes.push(proto::TokenBalance {
                //     token: proto::Token {}, // TODO: this should be fed from store_tokens using pre.mint
                //     transaction_id: bs58::encode(&transaction.signatures[0]).into_string(),
                //     block_height: block.block_height.block_height,
                //     account: pre_balance.owner,
                //     pre_balance: pre_balance_amount,
                //     post_balance: post_balance_amount,
                // });
            }
        }
    }

    Ok(balance_changes)
}

// #[substreams::handlers::store]
// fn store_tokens(block: solana::v1::Block, output: store::StoreSet) {
//     // TODO
// }

pub mod pb;

use num_bigint;
use pb::sol_token::v1 as proto;
use substreams::scalar::BigInt;
use substreams::store::{StoreNew, StoreSet, StoreSetRaw};
use substreams::{store, Hex, log};
use substreams_solana::pb::sol as solana;
use bs58;

// Map SPL token balance changes
#[substreams::handlers::map]
fn map_balances(
    block: solana::v1::Block,
) -> Result<proto::BalanceChanges, substreams::errors::Error> {
    log::info!("extracting SPL balance changes");
    let mut balance_changes = proto::BalanceChanges { items: vec![] };
    let mut index = 0;

    for trx in block.transactions {
        if let Some(meta) = trx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            if let Some(transaction) = trx.transaction {
                log::info!("{}", bs58::encode(&transaction.signatures[0]).into_string())
            }

            // let vec_string: String = meta.pre_balances.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
            log::info!("pre_balances: {} {} {} {}", meta.pre_balances.len(), meta.post_balances.len(), meta.pre_token_balances.len(), meta.post_token_balances.len());
        }
    }


    Ok(balance_changes)
}

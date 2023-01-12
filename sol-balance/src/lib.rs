use crate::pb::sol as token;
use bs58;
// use pb::sol::Account;
use prost::Message;
use substreams::store::StoreSet;
use substreams_solana::pb;
use substreams_solana::pb::sol as solana;

#[substreams::handlers::store]
fn store_balance(block: solana::v1::Block, output: StoreSet) {
    output.set(
        0,
        "block".to_string(),
        &format!("{:?}", block).as_bytes().to_vec(),
    );
}

// #[substreams::handlers::map]
// fn map(block: solana::v1::Block) -> Result<Account:> {
//     for tx in block.transactions {
//         if let Some(meta) = tx.meta {
//             if let Some(_) = meta.err {
//                 continue;
//             }
//             for i in 0..meta.post_balances.len() {
//                 // TODO: I don't think this is the right address, but I am struggling to figure out another way
//                 let account_id = bs58::encode(&meta.loaded_writable_addresses[i]).into_string();
//                 output.set(
//                     i as u64,
//                     account_id,
//                     &meta.post_balances[i].encode_to_vec()
//                 );
//             }
//         }
//     }
// }

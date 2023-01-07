use substreams::store::StoreSet;
use crate::pb::token::v1::Token;
use pb::token::v1 as proto;
use substreams_solana::pb::sol as solana;
use bs58;
use prost::Message;
use substreams_solana::pb;

#[substreams::handlers::map]
fn map(block: solana::v1::Block) -> Result<proto:> {
    for tx in block.transactions {
        if let Some(meta) = tx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            for i in 0..meta.post_balances.len() {
                // TODO: I don't think this is the right address, but I am struggling to figure out another way
                let account_id = bs58::encode(&meta.loaded_writable_addresses[i]).into_string();
                output.set(
                    i as u64,
                    account_id,
                    &meta.post_balances[i].encode_to_vec()
                );
            }
        }
    }
}

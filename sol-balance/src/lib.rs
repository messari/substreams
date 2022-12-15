use substreams::log;
use substreams::store::StoreSet;
use substreams_solana::pb::sol as solana;
use bs58;

#[substreams::handlers::store]
fn store_balance(block: solana::v1::Block, output: StoreSet) {
    for tx in block.transactions {
        if let Some(meta) = tx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            log::info!("bal len: {}", &meta.post_balances.len()); // TODO: remove
            for i in 0..meta.post_balances.len() {
                // TODO: I don't think this is the right address, but I am struggling to figure out another way
                let account_id = bs58::encode(&meta.loaded_writable_addresses[i]).into_string();
                output.set(
                    i as u64,
                    account_id,
                    &meta.post_balances[i] // TODO: not sure why this wants a &Vec<u8>
                );
            }
        }
    }
}

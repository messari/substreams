use substreams::log;
use substreams::store::StoreSet;
use substreams_solana::pb::sol as solana;
// use bs58;

#[substreams::handlers::store]
fn store_balance(block: solana::v1::Block, output: StoreSet) {
    log::info!("block hash: {}", block.previous_blockhash);
    log::info!("block hash: {}", block.parent_slot);

    // TODO: try to get balances in this for loop

    for tx in block.transactions {
        if let Some(meta) = tx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            log::info!("bal len: {}", meta.post_balances.len());

            for bal in meta.post_balances {
                // TODO: I don't think this is the right address, but I am struggling to figure out another way

                let change_address =

                output.set(
                    0,

                )
            }
            // log::info!("{}", meta.log_messages_none);
            // for bal in meta.post_balances {
            //     log::info!("post bal: {}", bal);
            // }
            if let Some(transaction) = tx.transaction {
                if let Some(msg) = transaction.message {
                    log::info!("acct len: {}", msg.account_keys.len());
                    for accts in msg.account_keys {
                        // let program_id = &msg.account_keys[inst.program_id_index as usize];
                        // log::info!("acc: {}", bs58::encode(program_id).into_string());
                    }
                }
            }
        }
    }
}

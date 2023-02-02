pub mod instruction;
pub mod pb;

use bs58;
use pb::sol_token::v1 as proto;
use substreams::log;
use substreams::store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto};
use substreams_solana::pb::sol as solana;

#[substreams::handlers::map]
fn map_balances(
    block: solana::v1::Block,
    token_store: StoreGetProto<proto::TokenAccount>,
) -> Result<proto::BalanceChanges, substreams::errors::Error> {
    log::info!("extracting SPL balance changes");
    let mut balance_changes = vec![];

    for tx in block.transactions {
        if let Some(meta) = tx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            if let Some(transaction) = tx.transaction {
                if let Some(msg) = transaction.message {
                    for i in 0..meta.pre_token_balances.len() {
                        let pre_balance = &meta.pre_token_balances[i];
                        let post_balance = &meta.post_token_balances[i];
                        // TODO pre.mint = token address
                        if let Some(pre_token_amount) = &pre_balance.ui_token_amount {
                            if let Some(post_token_amount) = &post_balance.ui_token_amount {
                                let try_token_store =
                                    token_store.get_last(&format!("address:{}", &pre_balance.mint));
                                if let Some(token) = try_token_store {
                                    balance_changes.push(proto::TokenBalance {
                                        token: Some(token),
                                        transaction_id: bs58::encode(&transaction.signatures[0])
                                            .into_string(),
                                        block_height: block
                                            .block_height
                                            .as_ref()
                                            .unwrap()
                                            .block_height,
                                        address: bs58::encode(
                                            &msg.account_keys[post_balance.account_index as usize],
                                        )
                                        .into_string(),
                                        pre_balance: pre_token_amount.amount.to_string(),
                                        post_balance: post_token_amount.amount.to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(proto::BalanceChanges {
        items: balance_changes,
    })
}

#[substreams::handlers::store]
fn store_tokens(block: solana::v1::Block, output: StoreSetProto<proto::TokenAccount>) {
    log::info!("extracting token mints");
    for tx in block.transactions {
        if let Some(meta) = tx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            if let Some(transaction) = tx.transaction {
                if let Some(msg) = transaction.message {
                    for inst in msg.instructions {
                        let program_id = &msg.account_keys[inst.program_id_index as usize];

                        // check if the token program is being called to create a token
                        if bs58::encode(program_id).into_string()
                            != "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                        {
                            continue;
                        }
                        let instruction = instruction::TokenInstruction::unpack(&inst.data);
                        match instruction {
                            Ok(instruction::TokenInstruction::InitializeMint {
                                decimals,
                                mint_authority,
                                freeze_authority,
                            }) => {
                                log::info!("Instruction: InitializeMint");
                                output.set(
                                    0,
                                    format!(
                                        "address:{}",
                                        bs58::encode(msg.account_keys[inst.accounts[0] as usize])
                                            .into_string()
                                    ),
                                    &get_token(
                                        msg.account_keys[inst.accounts[0] as usize].to_vec(),
                                        decimals,
                                        mint_authority,
                                        freeze_authority,
                                        bs58::encode(&transaction.signatures[0]).into_string(),
                                        block.block_height.as_ref().unwrap().block_height,
                                    ),
                                );
                            }
                            Ok(instruction::TokenInstruction::InitializeMint2 {
                                decimals,
                                mint_authority,
                                freeze_authority,
                            }) => {
                                log::info!("Instruction: InitializeMint2");
                                output.set(
                                    0,
                                    format!(
                                        "address:{}",
                                        bs58::encode(msg.account_keys[inst.accounts[0] as usize])
                                            .into_string()
                                    ),
                                    &get_token(
                                        msg.account_keys[inst.accounts[0] as usize].to_vec(),
                                        decimals,
                                        mint_authority,
                                        freeze_authority,
                                        bs58::encode(&transaction.signatures[0]).into_string(),
                                        block.block_height.as_ref().unwrap().block_height,
                                    ),
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

// helper to create a TokenAccount entity
fn get_token(
    token_address: Vec<u8>,
    decimal: u8,
    mint_authority: Vec<u8>,
    freeze_authority_opt: Option<Vec<u8>>,
    tx_id: String,
    block_height: u64,
) -> proto::TokenAccount {
    let mut token = proto::TokenAccount {
        address: bs58::encode(&token_address).into_string(),
        name: "".to_string(),
        symbol: "".to_string(),
        decimals: decimal.into(),
        freeze_authority: "".to_string(),
        mint_authority: bs58::encode(&mint_authority).into_string(),
        tx_created: tx_id,
        block_created: block_height,
    };

    // set freeze authority if it exists
    if freeze_authority_opt.is_some() {
        token.freeze_authority = bs58::encode(&freeze_authority_opt.unwrap()).into_string();
    }
    return token;
}

// pull the TokenAccount using the key provided
// fn get_token_store(
//     token_store: StoreGetProto<proto::TokenAccount>,
//     token_address: &String,
// ) -> Result<proto::TokenAccount, Error> {
//     return match &token_store.get_last(format!("address:{}", &token_address)) {
//         None => Err(Error::Unexpected(
//             format!("token {} not found", token_address).to_string(),
//         )),
//         Some(token) => token,
//     };
// }

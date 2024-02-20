mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::prelude::*;
use substreams::store;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const VAULT_TRACKED_CONTRACT: [u8; 20] = hex!("ba12222222228d8ba445958a75a0704d566bf2c8");

fn map_vault_events(blk: &eth::Block, events: &mut contract::Events) {
    events.vault_authorizer_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::vault_contract::events::AuthorizerChanged::match_and_decode(log) {
                        return Some(contract::VaultAuthorizerChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_authorizer: event.new_authorizer,
                        });
                    }

                    None
                })
        })
        .collect());
    events.vault_external_balance_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::vault_contract::events::ExternalBalanceTransfer::match_and_decode(log) {
                        return Some(contract::VaultExternalBalanceTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            recipient: event.recipient,
                            sender: event.sender,
                            token: event.token,
                        });
                    }

                    None
                })
        })
        .collect());
    events.vault_flash_loans.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::vault_contract::events::FlashLoan::match_and_decode(log) {
                        return Some(contract::VaultFlashLoan {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            fee_amount: event.fee_amount.to_string(),
                            recipient: event.recipient,
                            token: event.token,
                        });
                    }

                    None
                })
        })
        .collect());
    events.vault_internal_balance_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::vault_contract::events::InternalBalanceChanged::match_and_decode(log) {
                        return Some(contract::VaultInternalBalanceChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            delta: event.delta.to_string(),
                            token: event.token,
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.vault_paused_state_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::vault_contract::events::PausedStateChanged::match_and_decode(log) {
                        return Some(contract::VaultPausedStateChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            paused: event.paused,
                        });
                    }

                    None
                })
        })
        .collect());
    events.vault_pool_balance_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::vault_contract::events::PoolBalanceChanged::match_and_decode(log) {
                        return Some(contract::VaultPoolBalanceChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            deltas: event.deltas.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                            liquidity_provider: event.liquidity_provider,
                            pool_id: Vec::from(event.pool_id),
                            protocol_fee_amounts: event.protocol_fee_amounts.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                            tokens: event.tokens.into_iter().map(|x| x).collect::<Vec<_>>(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.vault_pool_balance_manageds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::vault_contract::events::PoolBalanceManaged::match_and_decode(log) {
                        return Some(contract::VaultPoolBalanceManaged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            asset_manager: event.asset_manager,
                            cash_delta: event.cash_delta.to_string(),
                            managed_delta: event.managed_delta.to_string(),
                            pool_id: Vec::from(event.pool_id),
                            token: event.token,
                        });
                    }

                    None
                })
        })
        .collect());
    events.vault_pool_registereds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::vault_contract::events::PoolRegistered::match_and_decode(log) {
                        return Some(contract::VaultPoolRegistered {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            pool_address: event.pool_address,
                            pool_id: Vec::from(event.pool_id),
                            specialization: event.specialization.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.vault_relayer_approval_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::vault_contract::events::RelayerApprovalChanged::match_and_decode(log) {
                        return Some(contract::VaultRelayerApprovalChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            relayer: event.relayer,
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());
    events.vault_swaps.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::vault_contract::events::Swap::match_and_decode(log) {
                        return Some(contract::VaultSwap {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount_in: event.amount_in.to_string(),
                            amount_out: event.amount_out.to_string(),
                            pool_id: Vec::from(event.pool_id),
                            token_in: event.token_in,
                            token_out: event.token_out,
                        });
                    }

                    None
                })
        })
        .collect());
    events.vault_tokens_deregistereds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::vault_contract::events::TokensDeregistered::match_and_decode(log) {
                        return Some(contract::VaultTokensDeregistered {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            pool_id: Vec::from(event.pool_id),
                            tokens: event.tokens.into_iter().map(|x| x).collect::<Vec<_>>(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.vault_tokens_registereds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::vault_contract::events::TokensRegistered::match_and_decode(log) {
                        return Some(contract::VaultTokensRegistered {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            asset_managers: event.asset_managers.into_iter().map(|x| x).collect::<Vec<_>>(),
                            pool_id: Vec::from(event.pool_id),
                            tokens: event.tokens.into_iter().map(|x| x).collect::<Vec<_>>(),
                        });
                    }

                    None
                })
        })
        .collect());
}

fn map_vault_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.vault_call_batch_swaps.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::BatchSwap::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::BatchSwap::decode(call) {
                        Ok(decoded_call) => {
                            let output_asset_deltas = match abi::vault_contract::functions::BatchSwap::output(&call.return_data) {
                                Ok(output_asset_deltas) => {output_asset_deltas}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::VaultBatchSwapCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                assets: decoded_call.assets.into_iter().map(|x| x).collect::<Vec<_>>(),
                                deadline: decoded_call.deadline.to_string(),
                                kind: decoded_call.kind.to_u64(),
                                limits: decoded_call.limits.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                output_asset_deltas: output_asset_deltas.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_deregister_tokens.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::DeregisterTokens::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::DeregisterTokens::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::VaultDeregisterTokensCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                pool_id: Vec::from(decoded_call.pool_id),
                                tokens: decoded_call.tokens.into_iter().map(|x| x).collect::<Vec<_>>(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_exit_pools.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::ExitPool::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::ExitPool::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::VaultExitPoolCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                pool_id: Vec::from(decoded_call.pool_id),
                                recipient: decoded_call.recipient,
                                sender: decoded_call.sender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_flash_loans.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::FlashLoan::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::FlashLoan::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::VaultFlashLoanCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amounts: decoded_call.amounts.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                recipient: decoded_call.recipient,
                                tokens: decoded_call.tokens.into_iter().map(|x| x).collect::<Vec<_>>(),
                                user_data: decoded_call.user_data,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_join_pools.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::JoinPool::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::JoinPool::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::VaultJoinPoolCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                pool_id: Vec::from(decoded_call.pool_id),
                                recipient: decoded_call.recipient,
                                sender: decoded_call.sender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_manage_pool_balances.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::ManagePoolBalance::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::ManagePoolBalance::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::VaultManagePoolBalanceCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_manage_user_balances.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::ManageUserBalance::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::ManageUserBalance::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::VaultManageUserBalanceCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_query_batch_swaps.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::QueryBatchSwap::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::QueryBatchSwap::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::vault_contract::functions::QueryBatchSwap::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::VaultQueryBatchSwapCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                assets: decoded_call.assets.into_iter().map(|x| x).collect::<Vec<_>>(),
                                kind: decoded_call.kind.to_u64(),
                                output_param0: output_param0.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_register_pools.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::RegisterPool::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::RegisterPool::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::vault_contract::functions::RegisterPool::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::VaultRegisterPoolCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: Vec::from(output_param0),
                                specialization: decoded_call.specialization.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_register_tokens.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::RegisterTokens::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::RegisterTokens::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::VaultRegisterTokensCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                asset_managers: decoded_call.asset_managers.into_iter().map(|x| x).collect::<Vec<_>>(),
                                pool_id: Vec::from(decoded_call.pool_id),
                                tokens: decoded_call.tokens.into_iter().map(|x| x).collect::<Vec<_>>(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_set_authorizers.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::SetAuthorizer::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::SetAuthorizer::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::VaultSetAuthorizerCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_authorizer: decoded_call.new_authorizer,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_set_pauseds.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::SetPaused::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::SetPaused::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::VaultSetPausedCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                paused: decoded_call.paused,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_set_relayer_approvals.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::SetRelayerApproval::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::SetRelayerApproval::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::VaultSetRelayerApprovalCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                approved: decoded_call.approved,
                                relayer: decoded_call.relayer,
                                sender: decoded_call.sender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.vault_call_swaps.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VAULT_TRACKED_CONTRACT && abi::vault_contract::functions::Swap::match_call(call))
                .filter_map(|call| {
                    match abi::vault_contract::functions::Swap::decode(call) {
                        Ok(decoded_call) => {
                            let output_amount_calculated = match abi::vault_contract::functions::Swap::output(&call.return_data) {
                                Ok(output_amount_calculated) => {output_amount_calculated}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::VaultSwapCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                deadline: decoded_call.deadline.to_string(),
                                limit: decoded_call.limit.to_string(),
                                output_amount_calculated: output_amount_calculated.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

fn is_declared_dds_address(addr: &Vec<u8>, ordinal: u64, dds_store: &store::StoreGetInt64) -> bool {
    //    substreams::log::info!("Checking if address {} is declared dds address", Hex(addr).to_string());
    if dds_store.get_at(ordinal, Hex(addr).to_string()).is_some() {
        return true;
    }
    return false;
}

fn map_pools_events(
    blk: &eth::Block,
    dds_store: &store::StoreGetInt64,
    events: &mut contract::Events,
) {

    events.pools_amp_update_starteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::AmpUpdateStarted::match_and_decode(log) {
                        return Some(contract::PoolsAmpUpdateStarted {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            end_time: event.end_time.to_string(),
                            end_value: event.end_value.to_string(),
                            start_time: event.start_time.to_string(),
                            start_value: event.start_value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_amp_update_stoppeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::AmpUpdateStopped::match_and_decode(log) {
                        return Some(contract::PoolsAmpUpdateStopped {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            current_value: event.current_value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::PoolsApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            owner: event.owner,
                            spender: event.spender,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_paused_state_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::PausedStateChanged::match_and_decode(log) {
                        return Some(contract::PoolsPausedStateChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            paused: event.paused,
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_protocol_fee_percentage_cache_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::ProtocolFeePercentageCacheUpdated::match_and_decode(log) {
                        return Some(contract::PoolsProtocolFeePercentageCacheUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            fee_type: event.fee_type.to_string(),
                            protocol_fee_percentage: event.protocol_fee_percentage.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_recovery_mode_state_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::RecoveryModeStateChanged::match_and_decode(log) {
                        return Some(contract::PoolsRecoveryModeStateChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            enabled: event.enabled,
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_swap_fee_percentage_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::SwapFeePercentageChanged::match_and_decode(log) {
                        return Some(contract::PoolsSwapFeePercentageChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            swap_fee_percentage: event.swap_fee_percentage.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_token_rate_cache_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::TokenRateCacheUpdated::match_and_decode(log) {
                        return Some(contract::PoolsTokenRateCacheUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            rate: event.rate.to_string(),
                            token_index: event.token_index.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_token_rate_provider_sets.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::TokenRateProviderSet::match_and_decode(log) {
                        return Some(contract::PoolsTokenRateProviderSet {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            cache_duration: event.cache_duration.to_string(),
                            provider: event.provider,
                            token_index: event.token_index.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::PoolsTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            from: event.from,
                            to: event.to,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_pools_calls(
    blk: &eth::Block,
    dds_store: &store::StoreGetInt64,
    calls: &mut contract::Calls,
) {
    calls.pools_call_approves.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::Approve::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::Approve::decode(call) {
                            Ok(decoded_call) => {
                            let output_param0 = match abi::pools_contract::functions::Approve::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolsApproveCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                amount: decoded_call.amount.to_string(),
                                output_param0: output_param0,
                                spender: decoded_call.spender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_decrease_allowances.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::DecreaseAllowance::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::DecreaseAllowance::decode(call) {
                            Ok(decoded_call) => {
                            let output_param0 = match abi::pools_contract::functions::DecreaseAllowance::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolsDecreaseAllowanceCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                amount: decoded_call.amount.to_string(),
                                output_param0: output_param0,
                                spender: decoded_call.spender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_disable_recovery_modes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::DisableRecoveryMode::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::DisableRecoveryMode::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolsDisableRecoveryModeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_enable_recovery_modes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::EnableRecoveryMode::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::EnableRecoveryMode::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolsEnableRecoveryModeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_increase_allowances.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::IncreaseAllowance::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::IncreaseAllowance::decode(call) {
                            Ok(decoded_call) => {
                            let output_param0 = match abi::pools_contract::functions::IncreaseAllowance::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolsIncreaseAllowanceCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                added_value: decoded_call.added_value.to_string(),
                                output_param0: output_param0,
                                spender: decoded_call.spender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_on_exit_pools.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::OnExitPool::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::OnExitPool::decode(call) {
                            Ok(decoded_call) => {
                            let (output_param0, output_param1) = match abi::pools_contract::functions::OnExitPool::output(&call.return_data) {
                                Ok((output_param0, output_param1)) => {(output_param0, output_param1)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolsOnExitPoolCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                balances: decoded_call.balances.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                last_change_block: decoded_call.last_change_block.to_string(),
                                output_param0: output_param0.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                output_param1: output_param1.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                pool_id: Vec::from(decoded_call.pool_id),
                                protocol_swap_fee_percentage: decoded_call.protocol_swap_fee_percentage.to_string(),
                                recipient: decoded_call.recipient,
                                sender: decoded_call.sender,
                                user_data: decoded_call.user_data,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_on_join_pools.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::OnJoinPool::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::OnJoinPool::decode(call) {
                            Ok(decoded_call) => {
                            let (output_param0, output_param1) = match abi::pools_contract::functions::OnJoinPool::output(&call.return_data) {
                                Ok((output_param0, output_param1)) => {(output_param0, output_param1)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolsOnJoinPoolCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                balances: decoded_call.balances.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                last_change_block: decoded_call.last_change_block.to_string(),
                                output_param0: output_param0.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                output_param1: output_param1.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                pool_id: Vec::from(decoded_call.pool_id),
                                protocol_swap_fee_percentage: decoded_call.protocol_swap_fee_percentage.to_string(),
                                recipient: decoded_call.recipient,
                                sender: decoded_call.sender,
                                user_data: decoded_call.user_data,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_on_swaps.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::OnSwap::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::OnSwap::decode(call) {
                            Ok(decoded_call) => {
                            let output_param0 = match abi::pools_contract::functions::OnSwap::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolsOnSwapCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                balances: decoded_call.balances.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                index_in: decoded_call.index_in.to_string(),
                                index_out: decoded_call.index_out.to_string(),
                                output_param0: output_param0.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_pauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::Pause::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::Pause::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolsPauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_permits.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::Permit::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::Permit::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolsPermitCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                deadline: decoded_call.deadline.to_string(),
                                owner: decoded_call.owner,
                                r: Vec::from(decoded_call.r),
                                s: Vec::from(decoded_call.s),
                                spender: decoded_call.spender,
                                v: decoded_call.v.to_u64(),
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_query_exits.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::QueryExit::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::QueryExit::decode(call) {
                            Ok(decoded_call) => {
                            let (output_bpt_in, output_amounts_out) = match abi::pools_contract::functions::QueryExit::output(&call.return_data) {
                                Ok((output_bpt_in, output_amounts_out)) => {(output_bpt_in, output_amounts_out)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolsQueryExitCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                balances: decoded_call.balances.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                last_change_block: decoded_call.last_change_block.to_string(),
                                output_amounts_out: output_amounts_out.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                output_bpt_in: output_bpt_in.to_string(),
                                pool_id: Vec::from(decoded_call.pool_id),
                                protocol_swap_fee_percentage: decoded_call.protocol_swap_fee_percentage.to_string(),
                                recipient: decoded_call.recipient,
                                sender: decoded_call.sender,
                                user_data: decoded_call.user_data,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_query_joins.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::QueryJoin::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::QueryJoin::decode(call) {
                            Ok(decoded_call) => {
                            let (output_bpt_out, output_amounts_in) = match abi::pools_contract::functions::QueryJoin::output(&call.return_data) {
                                Ok((output_bpt_out, output_amounts_in)) => {(output_bpt_out, output_amounts_in)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolsQueryJoinCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                balances: decoded_call.balances.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                last_change_block: decoded_call.last_change_block.to_string(),
                                output_amounts_in: output_amounts_in.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                output_bpt_out: output_bpt_out.to_string(),
                                pool_id: Vec::from(decoded_call.pool_id),
                                protocol_swap_fee_percentage: decoded_call.protocol_swap_fee_percentage.to_string(),
                                recipient: decoded_call.recipient,
                                sender: decoded_call.sender,
                                user_data: decoded_call.user_data,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_set_asset_manager_pool_configs.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::SetAssetManagerPoolConfig::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::SetAssetManagerPoolConfig::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolsSetAssetManagerPoolConfigCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                pool_config: decoded_call.pool_config,
                                token: decoded_call.token,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_set_swap_fee_percentages.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::SetSwapFeePercentage::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::SetSwapFeePercentage::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolsSetSwapFeePercentageCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                swap_fee_percentage: decoded_call.swap_fee_percentage.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_set_token_rate_cache_durations.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::SetTokenRateCacheDuration::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::SetTokenRateCacheDuration::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolsSetTokenRateCacheDurationCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                duration: decoded_call.duration.to_string(),
                                token: decoded_call.token,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_start_amplification_parameter_updates.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::StartAmplificationParameterUpdate::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::StartAmplificationParameterUpdate::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolsStartAmplificationParameterUpdateCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                end_time: decoded_call.end_time.to_string(),
                                raw_end_value: decoded_call.raw_end_value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_stop_amplification_parameter_updates.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::StopAmplificationParameterUpdate::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::StopAmplificationParameterUpdate::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolsStopAmplificationParameterUpdateCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_transfers.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::Transfer::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::Transfer::decode(call) {
                            Ok(decoded_call) => {
                            let output_param0 = match abi::pools_contract::functions::Transfer::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolsTransferCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                amount: decoded_call.amount.to_string(),
                                output_param0: output_param0,
                                recipient: decoded_call.recipient,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_transfer_froms.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::TransferFrom::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::TransferFrom::decode(call) {
                            Ok(decoded_call) => {
                            let output_param0 = match abi::pools_contract::functions::TransferFrom::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolsTransferFromCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                amount: decoded_call.amount.to_string(),
                                output_param0: output_param0,
                                recipient: decoded_call.recipient,
                                sender: decoded_call.sender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_unpauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::Unpause::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::Unpause::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolsUnpauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_update_protocol_fee_percentage_caches.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::UpdateProtocolFeePercentageCache::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::UpdateProtocolFeePercentageCache::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolsUpdateProtocolFeePercentageCacheCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pools_call_update_token_rate_caches.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pools_contract::functions::UpdateTokenRateCache::match_call(call))
                .filter_map(|call| {
                    match abi::pools_contract::functions::UpdateTokenRateCache::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolsUpdateTokenRateCacheCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                token: decoded_call.token,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}



fn db_vault_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.vault_authorizer_changeds.iter().for_each(|evt| {
        tables
            .create_row("vault_authorizer_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_authorizer", Hex(&evt.new_authorizer).to_string());
    });
    events.vault_external_balance_transfers.iter().for_each(|evt| {
        tables
            .create_row("vault_external_balance_transfer", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("recipient", Hex(&evt.recipient).to_string())
            .set("sender", Hex(&evt.sender).to_string())
            .set("token", Hex(&evt.token).to_string());
    });
    events.vault_flash_loans.iter().for_each(|evt| {
        tables
            .create_row("vault_flash_loan", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("fee_amount", BigDecimal::from_str(&evt.fee_amount).unwrap())
            .set("recipient", Hex(&evt.recipient).to_string())
            .set("token", Hex(&evt.token).to_string());
    });
    events.vault_internal_balance_changeds.iter().for_each(|evt| {
        tables
            .create_row("vault_internal_balance_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("delta", BigDecimal::from_str(&evt.delta).unwrap())
            .set("token", Hex(&evt.token).to_string())
            .set("user", Hex(&evt.user).to_string());
    });
    events.vault_paused_state_changeds.iter().for_each(|evt| {
        tables
            .create_row("vault_paused_state_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("paused", evt.paused);
    });
    events.vault_pool_balance_changeds.iter().for_each(|evt| {
        tables
            .create_row("vault_pool_balance_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set_psql_array("deltas", evt.deltas.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("liquidity_provider", Hex(&evt.liquidity_provider).to_string())
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set_psql_array("protocol_fee_amounts", evt.protocol_fee_amounts.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set_psql_array("tokens", evt.tokens.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>());
    });
    events.vault_pool_balance_manageds.iter().for_each(|evt| {
        tables
            .create_row("vault_pool_balance_managed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("asset_manager", Hex(&evt.asset_manager).to_string())
            .set("cash_delta", BigDecimal::from_str(&evt.cash_delta).unwrap())
            .set("managed_delta", BigDecimal::from_str(&evt.managed_delta).unwrap())
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set("token", Hex(&evt.token).to_string());
    });
    events.vault_pool_registereds.iter().for_each(|evt| {
        tables
            .create_row("vault_pool_registered", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("pool_address", Hex(&evt.pool_address).to_string())
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set("specialization", evt.specialization);
    });
    events.vault_relayer_approval_changeds.iter().for_each(|evt| {
        tables
            .create_row("vault_relayer_approval_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("approved", evt.approved)
            .set("relayer", Hex(&evt.relayer).to_string())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.vault_swaps.iter().for_each(|evt| {
        tables
            .create_row("vault_swap", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount_in", BigDecimal::from_str(&evt.amount_in).unwrap())
            .set("amount_out", BigDecimal::from_str(&evt.amount_out).unwrap())
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set("token_in", Hex(&evt.token_in).to_string())
            .set("token_out", Hex(&evt.token_out).to_string());
    });
    events.vault_tokens_deregistereds.iter().for_each(|evt| {
        tables
            .create_row("vault_tokens_deregistered", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set_psql_array("tokens", evt.tokens.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>());
    });
    events.vault_tokens_registereds.iter().for_each(|evt| {
        tables
            .create_row("vault_tokens_registered", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set_psql_array("asset_managers", evt.asset_managers.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>())
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set_psql_array("tokens", evt.tokens.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>());
    });
}
fn db_vault_calls_out(calls: &contract::Calls, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis calls to create table changes
    calls.vault_call_batch_swaps.iter().for_each(|call| {
        tables
            .create_row("vault_call_batch_swap", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set_psql_array("assets", call.assets.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>())
            .set("deadline", BigDecimal::from_str(&call.deadline).unwrap())
            .set("kind", call.kind)
            .set_psql_array("limits", call.limits.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set_psql_array("output_asset_deltas", call.output_asset_deltas.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>());
    });
    calls.vault_call_deregister_tokens.iter().for_each(|call| {
        tables
            .create_row("vault_call_deregister_tokens", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set_psql_array("tokens", call.tokens.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>());
    });
    calls.vault_call_exit_pools.iter().for_each(|call| {
        tables
            .create_row("vault_call_exit_pool", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string());
    });
    calls.vault_call_flash_loans.iter().for_each(|call| {
        tables
            .create_row("vault_call_flash_loan", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set_psql_array("amounts", call.amounts.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("recipient", Hex(&call.recipient).to_string())
            .set_psql_array("tokens", call.tokens.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>())
            .set("user_data", Hex(&call.user_data).to_string());
    });
    calls.vault_call_join_pools.iter().for_each(|call| {
        tables
            .create_row("vault_call_join_pool", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string());
    });
    calls.vault_call_manage_pool_balances.iter().for_each(|call| {
        tables
            .create_row("vault_call_manage_pool_balance", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.vault_call_manage_user_balances.iter().for_each(|call| {
        tables
            .create_row("vault_call_manage_user_balance", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.vault_call_query_batch_swaps.iter().for_each(|call| {
        tables
            .create_row("vault_call_query_batch_swap", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set_psql_array("assets", call.assets.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>())
            .set("kind", call.kind)
            .set_psql_array("output_param0", call.output_param0.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>());
    });
    calls.vault_call_register_pools.iter().for_each(|call| {
        tables
            .create_row("vault_call_register_pool", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("output_param0", Hex(&call.output_param0).to_string())
            .set("specialization", call.specialization);
    });
    calls.vault_call_register_tokens.iter().for_each(|call| {
        tables
            .create_row("vault_call_register_tokens", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set_psql_array("asset_managers", call.asset_managers.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>())
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set_psql_array("tokens", call.tokens.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>());
    });
    calls.vault_call_set_authorizers.iter().for_each(|call| {
        tables
            .create_row("vault_call_set_authorizer", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("new_authorizer", Hex(&call.new_authorizer).to_string());
    });
    calls.vault_call_set_pauseds.iter().for_each(|call| {
        tables
            .create_row("vault_call_set_paused", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("paused", call.paused);
    });
    calls.vault_call_set_relayer_approvals.iter().for_each(|call| {
        tables
            .create_row("vault_call_set_relayer_approval", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("approved", call.approved)
            .set("relayer", Hex(&call.relayer).to_string())
            .set("sender", Hex(&call.sender).to_string());
    });
    calls.vault_call_swaps.iter().for_each(|call| {
        tables
            .create_row("vault_call_swap", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("deadline", BigDecimal::from_str(&call.deadline).unwrap())
            .set("limit", BigDecimal::from_str(&call.limit).unwrap())
            .set("output_amount_calculated", BigDecimal::from_str(&call.output_amount_calculated).unwrap());
    });
}
fn db_pools_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.pools_amp_update_starteds.iter().for_each(|evt| {
        tables
            .create_row("pools_amp_update_started", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("end_time", BigDecimal::from_str(&evt.end_time).unwrap())
            .set("end_value", BigDecimal::from_str(&evt.end_value).unwrap())
            .set("start_time", BigDecimal::from_str(&evt.start_time).unwrap())
            .set("start_value", BigDecimal::from_str(&evt.start_value).unwrap());
    });
    events.pools_amp_update_stoppeds.iter().for_each(|evt| {
        tables
            .create_row("pools_amp_update_stopped", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("current_value", BigDecimal::from_str(&evt.current_value).unwrap());
    });
    events.pools_approvals.iter().for_each(|evt| {
        tables
            .create_row("pools_approval", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.pools_paused_state_changeds.iter().for_each(|evt| {
        tables
            .create_row("pools_paused_state_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("paused", evt.paused);
    });
    events.pools_protocol_fee_percentage_cache_updateds.iter().for_each(|evt| {
        tables
            .create_row("pools_protocol_fee_percentage_cache_updated", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("fee_type", BigDecimal::from_str(&evt.fee_type).unwrap())
            .set("protocol_fee_percentage", BigDecimal::from_str(&evt.protocol_fee_percentage).unwrap());
    });
    events.pools_recovery_mode_state_changeds.iter().for_each(|evt| {
        tables
            .create_row("pools_recovery_mode_state_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("enabled", evt.enabled);
    });
    events.pools_swap_fee_percentage_changeds.iter().for_each(|evt| {
        tables
            .create_row("pools_swap_fee_percentage_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("swap_fee_percentage", BigDecimal::from_str(&evt.swap_fee_percentage).unwrap());
    });
    events.pools_token_rate_cache_updateds.iter().for_each(|evt| {
        tables
            .create_row("pools_token_rate_cache_updated", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("rate", BigDecimal::from_str(&evt.rate).unwrap())
            .set("token_index", BigDecimal::from_str(&evt.token_index).unwrap());
    });
    events.pools_token_rate_provider_sets.iter().for_each(|evt| {
        tables
            .create_row("pools_token_rate_provider_set", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("cache_duration", BigDecimal::from_str(&evt.cache_duration).unwrap())
            .set("provider", Hex(&evt.provider).to_string())
            .set("token_index", BigDecimal::from_str(&evt.token_index).unwrap());
    });
    events.pools_transfers.iter().for_each(|evt| {
        tables
            .create_row("pools_transfer", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
}
fn db_pools_calls_out(calls: &contract::Calls, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis calls to create table changes
    calls.pools_call_approves.iter().for_each(|call| {
        tables
            .create_row("pools_call_approve", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string());
    });
    calls.pools_call_decrease_allowances.iter().for_each(|call| {
        tables
            .create_row("pools_call_decrease_allowance", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string());
    });
    calls.pools_call_disable_recovery_modes.iter().for_each(|call| {
        tables
            .create_row("pools_call_disable_recovery_mode", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address);
    });
    calls.pools_call_enable_recovery_modes.iter().for_each(|call| {
        tables
            .create_row("pools_call_enable_recovery_mode", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address);
    });
    calls.pools_call_increase_allowances.iter().for_each(|call| {
        tables
            .create_row("pools_call_increase_allowance", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("added_value", BigDecimal::from_str(&call.added_value).unwrap())
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string());
    });
    calls.pools_call_on_exit_pools.iter().for_each(|call| {
        tables
            .create_row("pools_call_on_exit_pool", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set_psql_array("balances", call.balances.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("last_change_block", BigDecimal::from_str(&call.last_change_block).unwrap())
            .set_psql_array("output_param0", call.output_param0.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set_psql_array("output_param1", call.output_param1.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("protocol_swap_fee_percentage", BigDecimal::from_str(&call.protocol_swap_fee_percentage).unwrap())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string())
            .set("user_data", Hex(&call.user_data).to_string());
    });
    calls.pools_call_on_join_pools.iter().for_each(|call| {
        tables
            .create_row("pools_call_on_join_pool", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set_psql_array("balances", call.balances.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("last_change_block", BigDecimal::from_str(&call.last_change_block).unwrap())
            .set_psql_array("output_param0", call.output_param0.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set_psql_array("output_param1", call.output_param1.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("protocol_swap_fee_percentage", BigDecimal::from_str(&call.protocol_swap_fee_percentage).unwrap())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string())
            .set("user_data", Hex(&call.user_data).to_string());
    });
    calls.pools_call_on_swaps.iter().for_each(|call| {
        tables
            .create_row("pools_call_on_swap", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set_psql_array("balances", call.balances.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("index_in", BigDecimal::from_str(&call.index_in).unwrap())
            .set("index_out", BigDecimal::from_str(&call.index_out).unwrap())
            .set("output_param0", BigDecimal::from_str(&call.output_param0).unwrap());
    });
    calls.pools_call_pauses.iter().for_each(|call| {
        tables
            .create_row("pools_call_pause", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address);
    });
    calls.pools_call_permits.iter().for_each(|call| {
        tables
            .create_row("pools_call_permit", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("deadline", BigDecimal::from_str(&call.deadline).unwrap())
            .set("owner", Hex(&call.owner).to_string())
            .set("r", Hex(&call.r).to_string())
            .set("s", Hex(&call.s).to_string())
            .set("spender", Hex(&call.spender).to_string())
            .set("v", call.v)
            .set("value", BigDecimal::from_str(&call.value).unwrap());
    });
    calls.pools_call_query_exits.iter().for_each(|call| {
        tables
            .create_row("pools_call_query_exit", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set_psql_array("balances", call.balances.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("last_change_block", BigDecimal::from_str(&call.last_change_block).unwrap())
            .set_psql_array("output_amounts_out", call.output_amounts_out.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("output_bpt_in", BigDecimal::from_str(&call.output_bpt_in).unwrap())
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("protocol_swap_fee_percentage", BigDecimal::from_str(&call.protocol_swap_fee_percentage).unwrap())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string())
            .set("user_data", Hex(&call.user_data).to_string());
    });
    calls.pools_call_query_joins.iter().for_each(|call| {
        tables
            .create_row("pools_call_query_join", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set_psql_array("balances", call.balances.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("last_change_block", BigDecimal::from_str(&call.last_change_block).unwrap())
            .set_psql_array("output_amounts_in", call.output_amounts_in.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("output_bpt_out", BigDecimal::from_str(&call.output_bpt_out).unwrap())
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("protocol_swap_fee_percentage", BigDecimal::from_str(&call.protocol_swap_fee_percentage).unwrap())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string())
            .set("user_data", Hex(&call.user_data).to_string());
    });
    calls.pools_call_set_asset_manager_pool_configs.iter().for_each(|call| {
        tables
            .create_row("pools_call_set_asset_manager_pool_config", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("pool_config", Hex(&call.pool_config).to_string())
            .set("token", Hex(&call.token).to_string());
    });
    calls.pools_call_set_swap_fee_percentages.iter().for_each(|call| {
        tables
            .create_row("pools_call_set_swap_fee_percentage", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("swap_fee_percentage", BigDecimal::from_str(&call.swap_fee_percentage).unwrap());
    });
    calls.pools_call_set_token_rate_cache_durations.iter().for_each(|call| {
        tables
            .create_row("pools_call_set_token_rate_cache_duration", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("duration", BigDecimal::from_str(&call.duration).unwrap())
            .set("token", Hex(&call.token).to_string());
    });
    calls.pools_call_start_amplification_parameter_updates.iter().for_each(|call| {
        tables
            .create_row("pools_call_start_amplification_parameter_update", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("end_time", BigDecimal::from_str(&call.end_time).unwrap())
            .set("raw_end_value", BigDecimal::from_str(&call.raw_end_value).unwrap());
    });
    calls.pools_call_stop_amplification_parameter_updates.iter().for_each(|call| {
        tables
            .create_row("pools_call_stop_amplification_parameter_update", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address);
    });
    calls.pools_call_transfers.iter().for_each(|call| {
        tables
            .create_row("pools_call_transfer", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("recipient", Hex(&call.recipient).to_string());
    });
    calls.pools_call_transfer_froms.iter().for_each(|call| {
        tables
            .create_row("pools_call_transfer_from", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string());
    });
    calls.pools_call_unpauses.iter().for_each(|call| {
        tables
            .create_row("pools_call_unpause", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address);
    });
    calls.pools_call_update_protocol_fee_percentage_caches.iter().for_each(|call| {
        tables
            .create_row("pools_call_update_protocol_fee_percentage_cache", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address);
    });
    calls.pools_call_update_token_rate_caches.iter().for_each(|call| {
        tables
            .create_row("pools_call_update_token_rate_cache", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("token", Hex(&call.token).to_string());
    });
}


fn graph_vault_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.vault_authorizer_changeds.iter().for_each(|evt| {
        tables
            .create_row("vault_authorizer_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_authorizer", Hex(&evt.new_authorizer).to_string());
    });
    events.vault_external_balance_transfers.iter().for_each(|evt| {
        tables
            .create_row("vault_external_balance_transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("recipient", Hex(&evt.recipient).to_string())
            .set("sender", Hex(&evt.sender).to_string())
            .set("token", Hex(&evt.token).to_string());
    });
    events.vault_flash_loans.iter().for_each(|evt| {
        tables
            .create_row("vault_flash_loan", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("fee_amount", BigDecimal::from_str(&evt.fee_amount).unwrap())
            .set("recipient", Hex(&evt.recipient).to_string())
            .set("token", Hex(&evt.token).to_string());
    });
    events.vault_internal_balance_changeds.iter().for_each(|evt| {
        tables
            .create_row("vault_internal_balance_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("delta", BigDecimal::from_str(&evt.delta).unwrap())
            .set("token", Hex(&evt.token).to_string())
            .set("user", Hex(&evt.user).to_string());
    });
    events.vault_paused_state_changeds.iter().for_each(|evt| {
        tables
            .create_row("vault_paused_state_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("paused", evt.paused);
    });
    events.vault_pool_balance_changeds.iter().for_each(|evt| {
        tables
            .create_row("vault_pool_balance_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("deltas", evt.deltas.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("liquidity_provider", Hex(&evt.liquidity_provider).to_string())
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set("protocol_fee_amounts", evt.protocol_fee_amounts.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("tokens", evt.tokens.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>());
    });
    events.vault_pool_balance_manageds.iter().for_each(|evt| {
        tables
            .create_row("vault_pool_balance_managed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("asset_manager", Hex(&evt.asset_manager).to_string())
            .set("cash_delta", BigDecimal::from_str(&evt.cash_delta).unwrap())
            .set("managed_delta", BigDecimal::from_str(&evt.managed_delta).unwrap())
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set("token", Hex(&evt.token).to_string());
    });
    events.vault_pool_registereds.iter().for_each(|evt| {
        tables
            .create_row("vault_pool_registered", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("pool_address", Hex(&evt.pool_address).to_string())
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set("specialization", evt.specialization);
    });
    events.vault_relayer_approval_changeds.iter().for_each(|evt| {
        tables
            .create_row("vault_relayer_approval_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("approved", evt.approved)
            .set("relayer", Hex(&evt.relayer).to_string())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.vault_swaps.iter().for_each(|evt| {
        tables
            .create_row("vault_swap", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount_in", BigDecimal::from_str(&evt.amount_in).unwrap())
            .set("amount_out", BigDecimal::from_str(&evt.amount_out).unwrap())
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set("token_in", Hex(&evt.token_in).to_string())
            .set("token_out", Hex(&evt.token_out).to_string());
    });
    events.vault_tokens_deregistereds.iter().for_each(|evt| {
        tables
            .create_row("vault_tokens_deregistered", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set("tokens", evt.tokens.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>());
    });
    events.vault_tokens_registereds.iter().for_each(|evt| {
        tables
            .create_row("vault_tokens_registered", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("asset_managers", evt.asset_managers.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>())
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set("tokens", evt.tokens.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>());
    });
}
fn graph_vault_calls_out(calls: &contract::Calls, tables: &mut EntityChangesTables) {
    // Loop over all the abis calls to create table changes
    calls.vault_call_batch_swaps.iter().for_each(|call| {
        tables
            .create_row("vault_call_batch_swap", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("assets", call.assets.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>())
            .set("deadline", BigDecimal::from_str(&call.deadline).unwrap())
            .set("kind", call.kind)
            .set("limits", call.limits.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("output_asset_deltas", call.output_asset_deltas.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>());
    });
    calls.vault_call_deregister_tokens.iter().for_each(|call| {
        tables
            .create_row("vault_call_deregister_tokens", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("tokens", call.tokens.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>());
    });
    calls.vault_call_exit_pools.iter().for_each(|call| {
        tables
            .create_row("vault_call_exit_pool", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string());
    });
    calls.vault_call_flash_loans.iter().for_each(|call| {
        tables
            .create_row("vault_call_flash_loan", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amounts", call.amounts.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("tokens", call.tokens.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>())
            .set("user_data", Hex(&call.user_data).to_string());
    });
    calls.vault_call_join_pools.iter().for_each(|call| {
        tables
            .create_row("vault_call_join_pool", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string());
    });
    calls.vault_call_manage_pool_balances.iter().for_each(|call| {
        tables
            .create_row("vault_call_manage_pool_balance", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.vault_call_manage_user_balances.iter().for_each(|call| {
        tables
            .create_row("vault_call_manage_user_balance", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.vault_call_query_batch_swaps.iter().for_each(|call| {
        tables
            .create_row("vault_call_query_batch_swap", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("assets", call.assets.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>())
            .set("kind", call.kind)
            .set("output_param0", call.output_param0.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>());
    });
    calls.vault_call_register_pools.iter().for_each(|call| {
        tables
            .create_row("vault_call_register_pool", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("output_param0", Hex(&call.output_param0).to_string())
            .set("specialization", call.specialization);
    });
    calls.vault_call_register_tokens.iter().for_each(|call| {
        tables
            .create_row("vault_call_register_tokens", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("asset_managers", call.asset_managers.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>())
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("tokens", call.tokens.iter().map(|x| Hex(&x).to_string()).collect::<Vec<_>>());
    });
    calls.vault_call_set_authorizers.iter().for_each(|call| {
        tables
            .create_row("vault_call_set_authorizer", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("new_authorizer", Hex(&call.new_authorizer).to_string());
    });
    calls.vault_call_set_pauseds.iter().for_each(|call| {
        tables
            .create_row("vault_call_set_paused", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("paused", call.paused);
    });
    calls.vault_call_set_relayer_approvals.iter().for_each(|call| {
        tables
            .create_row("vault_call_set_relayer_approval", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("approved", call.approved)
            .set("relayer", Hex(&call.relayer).to_string())
            .set("sender", Hex(&call.sender).to_string());
    });
    calls.vault_call_swaps.iter().for_each(|call| {
        tables
            .create_row("vault_call_swap", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("deadline", BigDecimal::from_str(&call.deadline).unwrap())
            .set("limit", BigDecimal::from_str(&call.limit).unwrap())
            .set("output_amount_calculated", BigDecimal::from_str(&call.output_amount_calculated).unwrap());
    });
  }
fn graph_pools_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.pools_amp_update_starteds.iter().for_each(|evt| {
        tables
            .create_row("pools_amp_update_started", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("end_time", BigDecimal::from_str(&evt.end_time).unwrap())
            .set("end_value", BigDecimal::from_str(&evt.end_value).unwrap())
            .set("start_time", BigDecimal::from_str(&evt.start_time).unwrap())
            .set("start_value", BigDecimal::from_str(&evt.start_value).unwrap());
    });
    events.pools_amp_update_stoppeds.iter().for_each(|evt| {
        tables
            .create_row("pools_amp_update_stopped", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("current_value", BigDecimal::from_str(&evt.current_value).unwrap());
    });
    events.pools_approvals.iter().for_each(|evt| {
        tables
            .create_row("pools_approval", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.pools_paused_state_changeds.iter().for_each(|evt| {
        tables
            .create_row("pools_paused_state_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("paused", evt.paused);
    });
    events.pools_protocol_fee_percentage_cache_updateds.iter().for_each(|evt| {
        tables
            .create_row("pools_protocol_fee_percentage_cache_updated", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("fee_type", BigDecimal::from_str(&evt.fee_type).unwrap())
            .set("protocol_fee_percentage", BigDecimal::from_str(&evt.protocol_fee_percentage).unwrap());
    });
    events.pools_recovery_mode_state_changeds.iter().for_each(|evt| {
        tables
            .create_row("pools_recovery_mode_state_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("enabled", evt.enabled);
    });
    events.pools_swap_fee_percentage_changeds.iter().for_each(|evt| {
        tables
            .create_row("pools_swap_fee_percentage_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("swap_fee_percentage", BigDecimal::from_str(&evt.swap_fee_percentage).unwrap());
    });
    events.pools_token_rate_cache_updateds.iter().for_each(|evt| {
        tables
            .create_row("pools_token_rate_cache_updated", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("rate", BigDecimal::from_str(&evt.rate).unwrap())
            .set("token_index", BigDecimal::from_str(&evt.token_index).unwrap());
    });
    events.pools_token_rate_provider_sets.iter().for_each(|evt| {
        tables
            .create_row("pools_token_rate_provider_set", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("cache_duration", BigDecimal::from_str(&evt.cache_duration).unwrap())
            .set("provider", Hex(&evt.provider).to_string())
            .set("token_index", BigDecimal::from_str(&evt.token_index).unwrap());
    });
    events.pools_transfers.iter().for_each(|evt| {
        tables
            .create_row("pools_transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
}
fn graph_pools_calls_out(calls: &contract::Calls, tables: &mut EntityChangesTables) {
    // Loop over all the abis calls to create table changes
    calls.pools_call_approves.iter().for_each(|call| {
        tables
            .create_row("pools_call_approve", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string());
    });
    calls.pools_call_decrease_allowances.iter().for_each(|call| {
        tables
            .create_row("pools_call_decrease_allowance", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string());
    });
    calls.pools_call_disable_recovery_modes.iter().for_each(|call| {
        tables
            .create_row("pools_call_disable_recovery_mode", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address);
    });
    calls.pools_call_enable_recovery_modes.iter().for_each(|call| {
        tables
            .create_row("pools_call_enable_recovery_mode", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address);
    });
    calls.pools_call_increase_allowances.iter().for_each(|call| {
        tables
            .create_row("pools_call_increase_allowance", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("added_value", BigDecimal::from_str(&call.added_value).unwrap())
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string());
    });
    calls.pools_call_on_exit_pools.iter().for_each(|call| {
        tables
            .create_row("pools_call_on_exit_pool", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("balances", call.balances.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("last_change_block", BigDecimal::from_str(&call.last_change_block).unwrap())
            .set("output_param0", call.output_param0.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("output_param1", call.output_param1.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("protocol_swap_fee_percentage", BigDecimal::from_str(&call.protocol_swap_fee_percentage).unwrap())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string())
            .set("user_data", Hex(&call.user_data).to_string());
    });
    calls.pools_call_on_join_pools.iter().for_each(|call| {
        tables
            .create_row("pools_call_on_join_pool", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("balances", call.balances.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("last_change_block", BigDecimal::from_str(&call.last_change_block).unwrap())
            .set("output_param0", call.output_param0.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("output_param1", call.output_param1.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("protocol_swap_fee_percentage", BigDecimal::from_str(&call.protocol_swap_fee_percentage).unwrap())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string())
            .set("user_data", Hex(&call.user_data).to_string());
    });
    calls.pools_call_on_swaps.iter().for_each(|call| {
        tables
            .create_row("pools_call_on_swap", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("balances", call.balances.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("index_in", BigDecimal::from_str(&call.index_in).unwrap())
            .set("index_out", BigDecimal::from_str(&call.index_out).unwrap())
            .set("output_param0", BigDecimal::from_str(&call.output_param0).unwrap());
    });
    calls.pools_call_pauses.iter().for_each(|call| {
        tables
            .create_row("pools_call_pause", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address);
    });
    calls.pools_call_permits.iter().for_each(|call| {
        tables
            .create_row("pools_call_permit", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("deadline", BigDecimal::from_str(&call.deadline).unwrap())
            .set("owner", Hex(&call.owner).to_string())
            .set("r", Hex(&call.r).to_string())
            .set("s", Hex(&call.s).to_string())
            .set("spender", Hex(&call.spender).to_string())
            .set("v", call.v)
            .set("value", BigDecimal::from_str(&call.value).unwrap());
    });
    calls.pools_call_query_exits.iter().for_each(|call| {
        tables
            .create_row("pools_call_query_exit", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("balances", call.balances.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("last_change_block", BigDecimal::from_str(&call.last_change_block).unwrap())
            .set("output_amounts_out", call.output_amounts_out.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("output_bpt_in", BigDecimal::from_str(&call.output_bpt_in).unwrap())
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("protocol_swap_fee_percentage", BigDecimal::from_str(&call.protocol_swap_fee_percentage).unwrap())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string())
            .set("user_data", Hex(&call.user_data).to_string());
    });
    calls.pools_call_query_joins.iter().for_each(|call| {
        tables
            .create_row("pools_call_query_join", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("balances", call.balances.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("last_change_block", BigDecimal::from_str(&call.last_change_block).unwrap())
            .set("output_amounts_in", call.output_amounts_in.iter().map(|x| BigDecimal::from_str(&x).unwrap()).collect::<Vec<_>>())
            .set("output_bpt_out", BigDecimal::from_str(&call.output_bpt_out).unwrap())
            .set("pool_id", Hex(&call.pool_id).to_string())
            .set("protocol_swap_fee_percentage", BigDecimal::from_str(&call.protocol_swap_fee_percentage).unwrap())
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string())
            .set("user_data", Hex(&call.user_data).to_string());
    });
    calls.pools_call_set_asset_manager_pool_configs.iter().for_each(|call| {
        tables
            .create_row("pools_call_set_asset_manager_pool_config", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("pool_config", Hex(&call.pool_config).to_string())
            .set("token", Hex(&call.token).to_string());
    });
    calls.pools_call_set_swap_fee_percentages.iter().for_each(|call| {
        tables
            .create_row("pools_call_set_swap_fee_percentage", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("swap_fee_percentage", BigDecimal::from_str(&call.swap_fee_percentage).unwrap());
    });
    calls.pools_call_set_token_rate_cache_durations.iter().for_each(|call| {
        tables
            .create_row("pools_call_set_token_rate_cache_duration", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("duration", BigDecimal::from_str(&call.duration).unwrap())
            .set("token", Hex(&call.token).to_string());
    });
    calls.pools_call_start_amplification_parameter_updates.iter().for_each(|call| {
        tables
            .create_row("pools_call_start_amplification_parameter_update", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("end_time", BigDecimal::from_str(&call.end_time).unwrap())
            .set("raw_end_value", BigDecimal::from_str(&call.raw_end_value).unwrap());
    });
    calls.pools_call_stop_amplification_parameter_updates.iter().for_each(|call| {
        tables
            .create_row("pools_call_stop_amplification_parameter_update", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address);
    });
    calls.pools_call_transfers.iter().for_each(|call| {
        tables
            .create_row("pools_call_transfer", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("recipient", Hex(&call.recipient).to_string());
    });
    calls.pools_call_transfer_froms.iter().for_each(|call| {
        tables
            .create_row("pools_call_transfer_from", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string());
    });
    calls.pools_call_unpauses.iter().for_each(|call| {
        tables
            .create_row("pools_call_unpause", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address);
    });
    calls.pools_call_update_protocol_fee_percentage_caches.iter().for_each(|call| {
        tables
            .create_row("pools_call_update_protocol_fee_percentage_cache", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address);
    });
    calls.pools_call_update_token_rate_caches.iter().for_each(|call| {
        tables
            .create_row("pools_call_update_token_rate_cache", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("call_address", &call.call_address)
            .set("token", Hex(&call.token).to_string());
    });
  }
#[substreams::handlers::store]
fn store_vault_pools_created(blk: eth::Block, store: StoreSetInt64) {
    for rcpt in blk.receipts() {
        for log in rcpt
            .receipt
            .logs
            .iter()
            .filter(|log| log.address == VAULT_TRACKED_CONTRACT)
        {
            if let Some(event) = abi::vault_contract::events::PoolRegistered::match_and_decode(log) {
                store.set(log.ordinal, Hex(event.pool_address).to_string(), &1);
            }
        }
    }
}

#[substreams::handlers::map]
fn map_events(
    blk: eth::Block,
    store_pools: StoreGetInt64,
) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_vault_events(&blk, &mut events);
    map_pools_events(&blk, &store_pools, &mut events);
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(
    blk: eth::Block,
    store_pools: StoreGetInt64,
) -> Result<contract::Calls, substreams::errors::Error> {
    let mut calls = contract::Calls::default();
    map_vault_calls(&blk, &mut calls);
    map_pools_calls(&blk, &store_pools, &mut calls);
    Ok(calls)
}

#[substreams::handlers::map]
fn db_out(events: contract::Events, calls: contract::Calls) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = DatabaseChangeTables::new();
    db_vault_out(&events, &mut tables);
    db_vault_calls_out(&calls, &mut tables);
    db_pools_out(&events, &mut tables);
    db_pools_calls_out(&calls, &mut tables);
    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events, calls: contract::Calls) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_vault_out(&events, &mut tables);
    graph_vault_calls_out(&calls, &mut tables);
    graph_pools_out(&events, &mut tables);
    graph_pools_calls_out(&calls, &mut tables);
    Ok(tables.to_entity_changes())
}

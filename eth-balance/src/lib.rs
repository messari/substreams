pub mod pb;

use substreams::pb::substreams::store_delta::Operation;
use substreams::Hex;
// use substreams::scalar::BigInt;
use substreams_entity_change::change::ToField;
use substreams_ethereum::pb::eth as pbeth;
use substreams_helper::pb::erc20::v1 as proto;
use substreams_helper::token as token_helper;
use substreams_helper::token::ETH_ADDRESS;

use substreams_entity_change::pb::entity::entity_change::Operation as EntityChangeOperation;
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

#[substreams::handlers::map]
fn map_balances(
    block: pbeth::v2::Block,
) -> Result<proto::TransferEvents, substreams::errors::Error> {
    let mut transfers = vec![];

    for transaction in &block.transaction_traces {
        let mut balance_changes = vec![];
        for call in &transaction.calls {
            for balance_change in &call.balance_changes {
                balance_changes.push(proto::TokenBalance {
                    log_ordinal: Some(balance_change.ordinal),
                    token: token_helper::get_eth_token(),
                    address: Hex::encode(&balance_change.address),
                    old_balance: Some(token_helper::bigint_to_string(
                        balance_change.old_value.clone(),
                    )),
                    new_balance: token_helper::bigint_to_string(balance_change.new_value.clone()),
                    reason: Some(balance_change.reason),
                });
            }
        }
        transfers.push(proto::TransferEvent {
            tx_hash: Hex::encode(&transaction.hash),
            block_number: block.number,
            timestamp: block
                .header
                .as_ref()
                .unwrap()
                .timestamp
                .as_ref()
                .unwrap()
                .seconds as u64,
            log_index: transaction.index,
            log_ordinal: None,
            token_address: ETH_ADDRESS.to_string(),
            from: Hex::encode(&transaction.from),
            to: Hex::encode(&transaction.to),
            amount: token_helper::bigint_to_string(transaction.value.clone()),
            balance_changes: balance_changes,
        });
    }

    Ok(proto::TransferEvents { items: transfers })
}

#[substreams::handlers::map]
fn map_entity_changes(
    transfer_events: proto::TransferEvents,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut transfer_enitites = vec![];
    let mut balance_change_entities = vec![];

    for transfer in transfer_events.items {
        // extract balance changes
        for balance_change in transfer.balance_changes {
            balance_change_entities.push(EntityChange {
                entity: "TokenBalance".to_string(),
                id: transfer.tx_hash.clone()
                    + transfer
                        .log_ordinal
                        .unwrap_or_default()
                        .to_string()
                        .as_str(),
                ordinal: transfer.log_ordinal.unwrap_or_default(),
                operation: EntityChangeOperation::Create.into(),
                fields: vec![
                    transfer.tx_hash.clone().to_field("transfer".to_string()),
                    transfer
                        .log_ordinal
                        .unwrap_or_default()
                        .to_field("logOrdinal".to_string()),
                    balance_change.address.to_field("account".to_string()),
                    balance_change
                        .old_balance
                        .unwrap()
                        .to_field("oldBalance".to_string()),
                    balance_change
                        .new_balance
                        .to_field("newBalance".to_string()),
                    balance_change
                        .reason
                        .unwrap()
                        .to_field("reason".to_string()),
                ],
            });
        }

        // map transfer entity changes
        transfer_enitites.push(EntityChange {
            entity: "Transfer".to_string(),
            id: transfer.tx_hash.clone(),
            ordinal: transfer.block_number,
            operation: Operation::Create.into(),
            fields: vec![
                transfer.block_number.to_field("blockNumber".to_string()),
                transfer.timestamp.to_field("timestamp".to_string()),
                transfer.log_index.to_string().to_field("logIndex".to_string()),
                transfer
                    .log_ordinal
                    .unwrap_or_default()
                    .to_field("logOrdinal".to_string()),
                transfer.token_address.to_field("tokenAddress".to_string()),
                transfer.from.to_field("from".to_string()),
                transfer.to.to_field("to".to_string()),
                transfer.amount.to_field("amount".to_string()),
            ],
        });
    }

    Ok(EntityChanges {
        entity_changes: [transfer_enitites, balance_change_entities].concat(),
    })
}

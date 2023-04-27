use hex::FromHex;

use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams_entity_change::change::ToField;
use substreams_entity_change::pb::entity::entity_change::Operation;
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

use substreams_helper::convert::BigIntDeserializeExt;

use crate::pb::eth_supply::v1::EthSupply;

#[substreams::handlers::map]
fn map_entity_changes(
    clock: Clock,
    block_delta: EthSupply,
    cumulative: EthSupply,
) -> Result<EntityChanges, substreams::errors::Error> {
    let block_hash: String = clock.id;
    let hash_bytes: Vec<u8> = FromHex::from_hex::<&String>(&block_hash).unwrap();
    let timestamp = BigInt::from(clock.timestamp.unwrap().seconds);
    let block_num = clock.number;

    let entity_changes = vec![EntityChange {
        entity: "FeesBurnt".to_string(),
        id: block_hash.to_string(),
        ordinal: 1,
        operation: Operation::Create.into(),
        fields: vec![
            hash_bytes.to_field("blockHash".to_string()),
            block_num.to_field("blockNumber".to_string()),
            timestamp.to_field("timestamp".to_string()),
            cumulative
                .burned
                .unwrap()
                .deserialize()
                .to_field("cumulativeBurnedFees"),
            block_delta
                .burned
                .unwrap()
                .deserialize()
                .to_field("blockBurnedFees"),
        ],
    }];
    Ok(EntityChanges { entity_changes })
}

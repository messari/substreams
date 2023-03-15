use hex::ToHex;

use substreams::scalar::BigInt;
use substreams_entity_change::change::ToField;
use substreams_entity_change::pb::entity::entity_change::Operation;
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

use substreams_helper::convert::BigIntDeserializeExt;

use crate::pb::eth_supply::v1::EthSupply;

#[substreams::handlers::map]
fn map_entity_changes(
    block_delta: EthSupply,
    cumulative: EthSupply,
) -> Result<EntityChanges, substreams::errors::Error> {
    let hash_bytes: &Vec<u8> = cumulative.block_hash.as_ref();
    let block_hash: String = hash_bytes.encode_hex::<String>();
    let timestamp = BigInt::from(0);
    let entity_changes = vec![EntityChange {
        entity: "Supply".to_string(),
        id: block_hash.to_string(),
        ordinal: 1,
        operation: Operation::Create.into(),
        fields: vec![
            hash_bytes.to_field("blockHash".to_string()),
            cumulative.block_number.to_field("blockNumber".to_string()),
            timestamp.to_field("timestamp".to_string()),
            cumulative
                .total
                .unwrap()
                .deserialize()
                .to_field("currentSupply"),
            cumulative
                .genesis
                .unwrap()
                .deserialize()
                .to_field("genesisSupply"),
            cumulative
                .block_rewards
                .unwrap()
                .deserialize()
                .to_field("cumulativeMiningRewards"),
            cumulative
                .uncle_rewards
                .unwrap()
                .deserialize()
                .to_field("cumulativeUncleRewards"),
            cumulative
                .burned
                .unwrap()
                .deserialize()
                .to_field("cumulativeBurnedFees"),
            block_delta
                .block_rewards
                .unwrap()
                .deserialize()
                .to_field("blockMiningReward"),
            block_delta
                .uncle_rewards
                .unwrap()
                .deserialize()
                .to_field("blockUncleReward"),
            block_delta
                .burned
                .unwrap()
                .deserialize()
                .to_field("blockBurnedFees"),
        ],
    }];
    Ok(EntityChanges { entity_changes })
}

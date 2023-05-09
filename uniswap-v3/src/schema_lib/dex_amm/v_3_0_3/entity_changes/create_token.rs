use substreams::{Hex};
use substreams::prelude::*;
use substreams::pb::substreams::Clock;
use substreams_entity_change::pb::entity::{EntityChange, entity_change::Operation};

use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, CreateToken};
use crate::schema_lib::dex_amm::v_3_0_3::keys;

pub fn create_token_entity_change(
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    create_token: CreateToken,
) -> EntityChange {
    let mut token_change: EntityChange =
        EntityChange::new("Token", &format!("0x{}", hex::encode(create_token.token_address)), 0, Operation::Create);
    
        token_change
        .change("name", create_token.name)
        .change("symbol", create_token.symbol)
        .change("decimals", create_token.decimals);
    
        token_change
}

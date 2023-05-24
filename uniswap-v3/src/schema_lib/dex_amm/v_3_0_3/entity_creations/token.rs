use substreams::Hex;

use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, TokenEntityCreation};
use crate::tables::{Tables};

pub fn create_token_entity(
    tables: &mut Tables,
    entity_id: &Vec<u8>,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    token_entity_creation: &TokenEntityCreation,
) {
    tables.create_row("Token", Hex(entity_id).to_string())
        .set("name", &token_entity_creation.name)
        .set("symbol", &token_entity_creation.symbol)
        .set("decimals", token_entity_creation.decimals);
}

use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, TokenEntityCreation};
use crate::tables::{Tables};

pub fn create_token_entity(
    tables: &mut Tables,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    token_entity_creation: &TokenEntityCreation,
) {
    tables.create_row("Token", &format!("0x{}", hex::encode(&token_entity_creation.token_address)))
        .set("name", &token_entity_creation.name)
        .set("symbol", &token_entity_creation.symbol)
        .set("decimals", token_entity_creation.decimals);
}

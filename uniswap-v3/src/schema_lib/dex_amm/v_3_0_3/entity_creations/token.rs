use crate::pb::dex_amm::v3_0_3::TokenEntityCreation;
use crate::tables::{Tables};

pub fn create_token_entity(
    tables: &mut Tables,
    entity_id: &Vec<u8>,
    token_entity_creation: &TokenEntityCreation,
) {
    tables.create_row("Token", std::str::from_utf8(entity_id).unwrap())
        .set("name", &token_entity_creation.name)
        .set("symbol", &token_entity_creation.symbol)
        .set("decimals", token_entity_creation.decimals);
}

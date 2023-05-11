use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, CreateToken};
use crate::tables::{Tables};

pub fn create_token_entity_change(
    tables: &mut Tables,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    create_token: &CreateToken,
) {
    tables.create_row("Token", &format!("0x{}", hex::encode(&create_token.token_address)))
        .set("name", &create_token.name)
        .set("symbol", &create_token.symbol)
        .set("decimals", create_token.decimals);
}

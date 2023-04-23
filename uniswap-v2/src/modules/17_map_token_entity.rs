use substreams::scalar::BigDecimal;
use substreams_entity_change::pb::entity::{entity_change, EntityChange, EntityChanges};

use crate::pb::erc20::v1::Erc20Token;
use crate::pb::uniswap::v2::Pools;

#[substreams::handlers::map]
pub fn map_token_entity(pools_created: Pools) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];

    for pool in pools_created.pools {
        let mut tokens = pool.input_tokens.unwrap().items;
        tokens.push(pool.output_token.unwrap());

        for token in tokens {
            entity_changes.push(create_token_entity(token));
        }
    }

    Ok(EntityChanges { entity_changes })
}

fn create_token_entity(token: Erc20Token) -> EntityChange {
    let token_address = &token.address;

    let mut token_entity_change =
        EntityChange::new("Token", token_address, 0, entity_change::Operation::Create);

    token_entity_change
        .change("id", token_address)
        .change("name", &token.name)
        .change("symbol", &token.symbol)
        .change("decimals", token.decimals as i32)
        .change("lastPriceUSD", BigDecimal::zero());

    token_entity_change
}

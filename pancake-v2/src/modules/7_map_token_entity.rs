use substreams::scalar::BigDecimal;
use substreams::store::{DeltaBigDecimal, Deltas};
use substreams_entity_change::pb::entity::{entity_change, EntityChange, EntityChanges};

use crate::pb::erc20::v1::Erc20Token;
use crate::pb::pancake::v2::Pools;
use crate::store_key::StoreKey;

#[substreams::handlers::map]
pub fn map_token_entity(
    pools_created: Pools
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];

    for pool in pools_created.pools {
        for token in [
            pool.token0_ref(),
            pool.token1_ref(),
            pool.output_token_ref(),
        ] {
            entity_changes.push(init_token(&token))
        }
    }

    Ok(EntityChanges { entity_changes })
}

fn init_token(token: &Erc20Token) -> EntityChange {
    let mut token_entity_change =
        EntityChange::new("Token", &token.address, 0, entity_change::Operation::Create);

    token_entity_change
        .change("id", &token.address)
        .change("name", &token.name)
        .change("symbol", &token.symbol)
        .change("decimals", token.decimals as i32);

    token_entity_change
}


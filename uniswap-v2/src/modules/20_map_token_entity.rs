use substreams::scalar::BigDecimal;
use substreams::store::{DeltaBigDecimal, Deltas};
use substreams_entity_change::pb::entity::{entity_change, EntityChange, EntityChanges};

use crate::pb::erc20::v1::Erc20Token;
use crate::pb::uniswap::v2::Pools;
use crate::store_key::StoreKey;

#[substreams::handlers::map]
pub fn map_token_entity(
    pools_created: Pools,
    prices_delta: Deltas<DeltaBigDecimal>,
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

    for delta in prices_delta.deltas {
        if let Some(token_address) = StoreKey::TokenPrice.get_pool(&delta.key) {
            entity_changes.push(update_token_price(&token_address, delta.new_value));
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

fn update_token_price(address: &String, price: BigDecimal) -> EntityChange {
    let mut token_entity_change =
        EntityChange::new("Token", address, 0, entity_change::Operation::Update);

    token_entity_change.change("lastPriceUSD", price);
    token_entity_change
}

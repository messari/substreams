use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

#[substreams::handlers::map]
pub fn graph_out(
    liquidity_pool_map: EntityChanges,
    liquidity_pool_snapshots_map: EntityChanges,
    liquidity_protocol_map: EntityChanges,
    financial_daily_snapshot_map: EntityChanges,
    token_map: EntityChanges,
    events_map: EntityChanges,
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];

    entity_changes.extend(liquidity_pool_map.entity_changes);
    entity_changes.extend(liquidity_pool_snapshots_map.entity_changes);
    entity_changes.extend(liquidity_protocol_map.entity_changes);
    entity_changes.extend(financial_daily_snapshot_map.entity_changes);
    entity_changes.extend(token_map.entity_changes);
    entity_changes.extend(events_map.entity_changes);

    Ok(EntityChanges { entity_changes })
}

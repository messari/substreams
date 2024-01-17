use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

#[substreams::handlers::map]
pub fn graph_out(
    token_map: EntityChanges,
    events_map: EntityChanges,
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];

    entity_changes.extend(token_map.entity_changes);
    entity_changes.extend(events_map.entity_changes);

    Ok(EntityChanges { entity_changes })
}

use ethabi::ethereum_types::Address;
use substreams::store;
use substreams::store::StoreGet;
use substreams::store::StoreGetRaw;
use substreams_ethereum::pb::eth as pbeth;

use substreams_helper::block::BlockHandler;
use substreams_helper::hex::Hexable;

use crate::pb::aave_v2::v1::{AaveV2Event, AaveV2Events};
use crate::togen::log_to_event;
use crate::togen::EventMeta;

/// Extracts transfer events from the blocks for all observed contracts given in the store.
#[substreams::handlers::map]
fn map_raw_events(
    block: pbeth::v2::Block,
    store: store::StoreGetRaw,
) -> Result<AaveV2Events, substreams::errors::Error> {
    let bh = BlockHandler::new(&block);
    let mut events: Vec<AaveV2Event> = vec![];
    for log in block.logs() {
        let addr = Address::from_slice(&log.address()).to_hex();
        if store.get_last(addr).is_none() {
            continue;
        }

        let meta = &EventMeta {
            timestamp: bh.timestamp(),
            transaction_hash: log.receipt.transaction.hash.clone(),
            block_hash: block.hash.clone(),
            address: log.address().into(),
            log_index: log.index(),
        };
        let event = log_to_event(log.log, meta);
        if let Some(ev) = event {
            events.push(ev);
        }
    }

    Ok(AaveV2Events { events })
}

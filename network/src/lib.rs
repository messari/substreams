pub mod pb;

use prost::Message;
use substreams::{proto, store, Hex};
use substreams_ethereum::pb::eth::v2 as eth;

use crate::pb::network::v1::Network;

#[substreams::handlers::store]
fn map_network(
    block: eth::Block,
    daily_snapshots: store::StoreGet,
    hourly_snapshots: store::StoreGet,
    network: store::StoreSet,
) {
    let network = Network {
        ..Default::default()
    };

    // daily_snapshots.set(0, block.number.to_string(), &network.encode_to_vec());
}

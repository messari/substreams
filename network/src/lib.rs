pub mod pb;

use prost::Message;
use substreams::{proto, store, Hex};
use substreams_ethereum::pb::eth::v2 as eth;

use crate::pb::network::v1::Network;

#[substreams::handlers::store]
fn store_network_stats(block: eth::Block, output: store::StoreSet) {
    let network = Network {
        ..Default::default()
    };

    output.set(0, block.number.to_string(), &network.encode_to_vec());
}

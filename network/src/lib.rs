extern crate core;

pub mod pb;

use prost::Message;
use prost_types::Timestamp;
use substreams::scalar::BigInt;
use substreams::store::StoreAddBigInt;
use substreams::store::StoreGetBigInt;
use substreams::store::StoreGetRaw;
use substreams::store::StoreNew;
use substreams::store::StoreSetRaw;
use substreams::store::{StoreAdd, StoreGet, StoreSet};
use substreams::{log, store};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::scalar::BigIntSign;

use crate::pb::network::v1::{self as network, Network};

const CUMULATIVE_KEY: &str = "cumulative";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ProstDecode(#[from] prost::DecodeError),
}

pub struct BlockHandler<'a>(&'a eth::Block);

fn from_option_eth_bigint(bigint: Option<eth::BigInt>) -> BigInt {
    bigint
        .map(|b| BigInt::from_bytes_le(BigIntSign::Plus, b.bytes.as_slice()))
        .unwrap_or(BigInt::zero())
}

impl<'a> BlockHandler<'a> {
    pub fn new(block: &'a eth::Block) -> Self {
        Self(block)
    }

    // Return difficulty from block header as BigInt
    pub fn difficulty(&self) -> BigInt {
        self.0
            .header
            .clone()
            .map(|header| from_option_eth_bigint(header.difficulty))
            .unwrap_or(BigInt::zero())
    }

    // Return gas used from block header as BigInt
    pub fn gas_used(&self) -> BigInt {
        self.0
            .header
            .clone()
            .map(|header| BigInt::from(header.gas_used))
            .unwrap_or(BigInt::zero())
    }

    // Return burnt fees from block header as BigInt
    pub fn burnt_fees(&self) -> BigInt {
        self.0
            .transaction_traces
            .iter()
            .map(|t| from_option_eth_bigint(t.gas_price.clone()) * BigInt::from(t.gas_used))
            .fold(BigInt::zero(), |sum, val| sum + val)
    }
}

pub struct CumulativeValuesStore<'a>(&'a store::StoreGetBigInt);

impl<'a> CumulativeValuesStore<'a> {
    pub fn new(store: &'a store::StoreGetBigInt) -> Self {
        Self(store)
    }

    // Return cumulative value from store as network BigInt
    pub fn get_value(&self, ordinal: u64, key: &str) -> Option<network::BigInt> {
        self.0
            .get_at(ordinal, format!("{}:{}", CUMULATIVE_KEY, key))
            .map(|bigint| network::BigInt {
                bytes: bigint.to_bytes_le().1,
            })
    }

    pub fn get_network(&self, ordinal: u64) -> Network {
        Network {
            // TODO: track known authors
            cumulative_unique_authors: self.get_value(ordinal, "network:unique_authors"),
            cumulative_difficulty: self.get_value(ordinal, "network:difficulty"),
            cumulative_burnt_fees: self.get_value(ordinal, "network:burnt_fees"),
            cumulative_rewards: self.get_value(ordinal, "network:rewards"),
            cumulative_transactions: self.get_value(ordinal, "network:transactions"),
            cumulative_size: self.get_value(ordinal, "network:size"),
            ..Default::default()
        }
    }

    pub fn get_daily_snapshot(&self, ordinal: u64, day: i64) -> network::DailySnapshot {
        network::DailySnapshot {
            cumulative_unique_authors: self
                .get_value(ordinal, &format!("day:{}:unique_authors", day)),
            cumulative_difficulty: self.get_value(ordinal, &format!("day:{}:difficulty", day)),
            cumulative_gas_used: self.get_value(ordinal, &format!("day:{}:gas_used", day)),
            cumulative_burnt_fees: self.get_value(ordinal, &format!("day:{}:burnt_fees", day)),
            cumulative_rewards: self.get_value(ordinal, &format!("day:{}:rewards", day)),
            cumulative_size: self.get_value(ordinal, &format!("day:{}:size", day)),
            cumulative_transactions: self.get_value(ordinal, &format!("day:{}:transactions", day)),
            ..Default::default()
        }
    }
}

impl Network {
    fn from_store(store: &store::StoreGetRaw) -> Result<Self, Error> {
        let network = store
            .get_last("network".to_string())
            .map(|network| Network::decode(network.as_slice()))
            .transpose()?
            .unwrap_or_default();
        Ok(network)
    }
}

fn timestamp_day(timestamp: Option<Timestamp>) -> i64 {
    let seconds_in_day = 86400 as i64;
    let timestamp = timestamp.map(|t| t.seconds).unwrap_or_default();

    timestamp / seconds_in_day
}

#[substreams::handlers::store]
fn store_cumulative_values(block: eth::Block, output: store::StoreAddBigInt) {
    let block_handler = BlockHandler::new(&block);

    // Network cumulative values
    output.add(
        0,
        format!("{}:network:difficulty", CUMULATIVE_KEY),
        &block_handler.difficulty(),
    );
    output.add(
        0,
        format!("{}:network:gas_used", CUMULATIVE_KEY),
        &block_handler.gas_used(),
    );
    output.add(
        0,
        format!("{}:network:burnt_fees", CUMULATIVE_KEY),
        &block_handler.burnt_fees(),
    );

    output.add(
        0,
        format!("{}:network:rewards", CUMULATIVE_KEY),
        &block_handler.burnt_fees(),
    );

    output.add(
        0,
        format!("{}:network:size", CUMULATIVE_KEY),
        &BigInt::from(block.size),
    );

    output.add(
        0,
        format!("{}:network:transactions", CUMULATIVE_KEY),
        &BigInt::from(block.transaction_traces.len() as u64),
    );

    // Daily cumulative values
    let day = block
        .header
        .clone()
        .map(|h| timestamp_day(h.timestamp))
        .unwrap_or_default();

    output.add(
        0,
        format!("{}:day:{}:unique_authors", CUMULATIVE_KEY, day),
        &BigInt::zero(),
    );

    output.add(
        0,
        format!("{}:day:{}:difficulty", CUMULATIVE_KEY, day),
        &block_handler.difficulty(),
    );

    output.add(
        0,
        format!("{}:day:{}:gas_used", CUMULATIVE_KEY, day),
        &block_handler.gas_used(),
    );

    output.add(
        0,
        format!("{}:day:{}:burnt_fees", CUMULATIVE_KEY, day),
        &block_handler.burnt_fees(),
    );

    output.add(
        0,
        format!("{}:day:{}:rewards", CUMULATIVE_KEY, day),
        &block_handler.burnt_fees(),
    );

    output.add(
        0,
        format!("{}:day:{}:size", CUMULATIVE_KEY, day),
        &BigInt::from(block.size),
    );

    output.add(
        0,
        format!("{}:day:{}:transactions", CUMULATIVE_KEY, day),
        &BigInt::from(block.transaction_traces.len() as u64),
    );
}

#[substreams::handlers::store]
fn store_daily_snapshots(
    block: eth::Block,
    store_cumulative_values: store::StoreGetBigInt,
    output: store::StoreSetRaw,
) {
    let cumulative_store = CumulativeValuesStore::new(&store_cumulative_values);
    let day = block
        .header
        .clone()
        .map(|h| timestamp_day(h.timestamp))
        .unwrap_or_default();

    let daily_snapshot = cumulative_store.get_daily_snapshot(0, day);

    output.set(
        0,
        "daily_snapshot".to_string(),
        &daily_snapshot.encode_to_vec(),
    );
}

#[substreams::handlers::store]
fn store_network(
    block: eth::Block,
    store_cumulative_values: store::StoreGetBigInt,
    output: store::StoreSetRaw,
) {
    let cumulative_store = CumulativeValuesStore::new(&store_cumulative_values);

    let day = block
        .header
        .clone()
        .map(|h| timestamp_day(h.timestamp))
        .unwrap_or_default();

    let _daily_snapshot = cumulative_store.get_daily_snapshot(0, day);
    let mut network = cumulative_store.get_network(0);

    // Set network values;
    network.id = String::from("MAINNET");
    network.gas_limit = block
        .header
        .map(|header| header.gas_limit)
        .unwrap_or_default();
    network.block_height = block.number;

    // Finally, store the network state
    output.set(0, "network".to_string(), &network.encode_to_vec());
}

#[substreams::handlers::map]
fn map_network(
    _block: eth::Block,
    store_network: store::StoreGetRaw,
) -> Result<Network, substreams::errors::Error> {
    log::info!("Map Network");

    let network = Network::from_store(&store_network)
        .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?;

    log::info!("Network: {:?}", network);

    Ok(network)
}

// fn is_author_known(
//     block: &eth::Block,
//     store_known_block_authors: &store::StoreGet,
// ) -> Result<bool, substreams::errors::Error> {
//     let author = block
//         .header
//         .clone()
//         .map(|h| Address::from_slice(&h.coinbase))
//         .map(|a| a.to_string())
//         .unwrap_or_default();
//     Ok(store_known_block_authors.get_last(author.clone()).is_some())
// }

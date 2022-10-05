pub mod pb;

use prost::Message;
use prost_types::Timestamp;
use substreams::{log, proto, store, Hex};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_helper::{
    bigdecimal::{BigDecimal, Zero},
    math::decimal_from_bytes,
    num_bigint::{BigInt, Sign},
};

use crate::pb::network::v1::{self as network, DailySnapshots, Network};

const CUMULATIVE_KEY: &str = "cumulative";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ProstDecode(#[from] prost::DecodeError),
}

pub struct BlockHandler<'a>(&'a eth::Block);

fn from_option_eth_bigint(bigint: Option<eth::BigInt>) -> BigInt {
    bigint
        .map(|b| BigInt::from_bytes_le(Sign::Plus, b.bytes.as_slice()))
        .unwrap_or_default()
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
            .unwrap_or_default()
    }

    // Return gas used from block header as BigInt
    pub fn gas_used(&self) -> BigInt {
        self.0
            .header
            .clone()
            .map(|header| BigInt::from(header.gas_used))
            .unwrap_or_default()
    }

    // Return burnt fees from block header as BigInt
    pub fn burnt_fees(&self) -> BigInt {
        self.0
            .transaction_traces
            .iter()
            .map(|t| from_option_eth_bigint(t.gas_price.clone()) * BigInt::from(t.gas_used))
            .sum()
    }
}

pub struct CumulativeValuesStore<'a>(&'a store::StoreGet);

impl<'a> CumulativeValuesStore<'a> {
    pub fn new(store: &'a store::StoreGet) -> Self {
        Self(store)
    }

    // Return cumulative value from store as network BigInt
    pub fn get_value(&self, ordinal: u64, key: &str) -> Option<network::BigInt> {
        self.0
            .get_at(ordinal, format!("{}:{}", CUMULATIVE_KEY, key))
            .map(|bytes| network::BigInt { bytes })
    }

    pub fn get_daily_snapshot(&self, ordinal: u64, date: &str) -> network::DailySnapshot {
        network::DailySnapshot {
            cumulative_unique_authors: self
                .get_value(ordinal, &format!("daily:{}:unique_authors", date)),
            cumulative_difficulty: self.get_value(ordinal, &format!("daily:{}:difficulty", date)),
            cumulative_gas_used: self.get_value(ordinal, &format!("daily:{}:gas_used", date)),
            cumulative_burnt_fees: self.get_value(ordinal, &format!("daily:{}:burnt_fees", date)),
            cumulative_rewards: self.get_value(ordinal, &format!("daily:{}:rewards", date)),
            cumulative_size: self.get_value(ordinal, &format!("daily:{}:size", date)),
            cumulative_transactions: self
                .get_value(ordinal, &format!("daily:{}:transactions", date)),
            ..Default::default()
        }
    }
}

impl Network {
    fn from_store(store: &store::StoreGet) -> Result<Self, Error> {
        let network = store
            .get_last("network".to_string())
            .map(|network| Network::decode(network.as_slice()))
            .transpose()?
            .unwrap_or_default();
        Ok(network)
    }
}

fn timestamp_date_ymd(timestamp: Option<Timestamp>) -> String {
    let timestamp = timestamp.map(|t| t.seconds).unwrap_or_default();

    // convert to date string in format YYYY-MM-DD
    chrono::NaiveDateTime::from_timestamp(timestamp, 0)
        .date()
        .format("%Y-%m-%d")
        .to_string()
}

#[substreams::handlers::store]
fn store_cumulative_values(block: eth::Block, output: store::StoreAddBigInt) {
    let block_handler = BlockHandler::new(&block);

    // Network cumulative values
    output.add(
        block.number,
        format!("{}:network:difficulty", CUMULATIVE_KEY),
        &block_handler.difficulty(),
    );
    output.add(
        block.number,
        format!("{}:network:gas_used", CUMULATIVE_KEY),
        &block_handler.gas_used(),
    );
    output.add(
        block.number,
        format!("{}:network:burnt_fees", CUMULATIVE_KEY),
        &block_handler.burnt_fees(),
    );

    output.add(
        block.number,
        format!("{}:network:rewards", CUMULATIVE_KEY),
        &block_handler.burnt_fees(),
    );

    output.add(
        block.number,
        format!("{}:network:size", CUMULATIVE_KEY),
        &BigInt::from(block.size),
    );

    output.add(
        block.number,
        format!("{}:network:transactions", CUMULATIVE_KEY),
        &BigInt::from(block.transaction_traces.len()),
    );

    // Daily cumulative values
    let date = block
        .header
        .clone()
        .map(|h| timestamp_date_ymd(h.timestamp))
        .unwrap_or_default();

    output.add(
        block.number,
        format!("{}:daily:{}:unique_authors", CUMULATIVE_KEY, date),
        &BigInt::zero(),
    );

    output.add(
        block.number,
        format!("{}:daily:{}:difficulty", CUMULATIVE_KEY, date),
        &block_handler.difficulty(),
    );

    output.add(
        block.number,
        format!("{}:daily:{}:gas_used", CUMULATIVE_KEY, date),
        &block_handler.gas_used(),
    );

    output.add(
        block.number,
        format!("{}:daily:{}:burnt_fees", CUMULATIVE_KEY, date),
        &block_handler.burnt_fees(),
    );

    output.add(
        block.number,
        format!("{}:daily:{}:rewards", CUMULATIVE_KEY, date),
        &block_handler.burnt_fees(),
    );

    output.add(
        block.number,
        format!("{}:daily:{}:size", CUMULATIVE_KEY, date),
        &BigInt::from(block.size),
    );

    output.add(
        block.number,
        format!("{}:daily:{}:transactions", CUMULATIVE_KEY, date),
        &BigInt::from(block.transaction_traces.len()),
    );
}

#[substreams::handlers::store]
fn store_network(
    block: eth::Block,
    store_cumulative_values: store::StoreGet,
    output: store::StoreSet,
) {
    let cumulative_store = CumulativeValuesStore::new(&store_cumulative_values);

    let date = block
        .header
        .clone()
        .map(|h| timestamp_date_ymd(h.timestamp))
        .unwrap_or_default();

    let mut daily_snapshot = cumulative_store.get_daily_snapshot(block.number, &date);

    let network = Network {
        // TODO: determine what the network id should be
        id: String::from("ethereum.mainnet"),
        // TODO: track known authors
        cumulative_unique_authors: cumulative_store
            .get_value(block.number, "network:unique_authors"),
        cumulative_difficulty: cumulative_store.get_value(block.number, "network:difficulty"),
        cumulative_burnt_fees: cumulative_store.get_value(block.number, "network:burnt_fees"),
        cumulative_rewards: cumulative_store.get_value(block.number, "network:rewards"),
        cumulative_transactions: cumulative_store.get_value(block.number, "network:transactions"),
        cumulative_size: cumulative_store.get_value(block.number, "network:size"),
        gas_limit: block
            .header
            .map(|header| header.gas_limit)
            .unwrap_or_default(),
        block_height: block.number,
        daily_snapshots: Some(DailySnapshots {
            snapshots: vec![daily_snapshot],
        }),
        ..Default::default()
    };

    output.set(
        block.number,
        "network".to_string(),
        &network.encode_to_vec(),
    );
}

#[substreams::handlers::map]
fn map_network(
    block: eth::Block,
    store_network: store::StoreGet,
) -> Result<Network, substreams::errors::Error> {
    log::info!("Map Network");

    let network = Network::from_store(&store_network)
        .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?;

    log::info!("Network: {:?}", network);

    Ok(Network::default())
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

// fn get_cumulative_value(
//     key: &str,
//     store: &store::StoreGet,
// ) -> Result<BigInt, substreams::errors::Error> {
//     let value = store
//         .get_last(format!("cumulative:{}", key))
//         .unwrap_or_default();

//     let value_str = String::from_utf8(value)
//         .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?;

//     Ok(BigInt::from_str(&value_str)
//         .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?)
// }

// let cumulative_unique_authors =
//     get_cumulative_value("unique_authors", &store_cumulative_values)?
//         + if is_author_known(&block, &store_known_block_authors)? {
//             BigInt::zero()
//         } else {
//             BigInt::from(1)
//         };

// let cumulative_difficulty = store_cumulative_values
//     .get_last("cumulative:difficulty".to_string())
//     .map(|v| decimal_from_bytes(&v))
//     .transpose()
//     .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?
//     .unwrap_or_default();
// let cumulative_gas_used = store_cumulative_values
//     .get_last("cumulative:gas_used".to_string())
//     .map(|v| decimal_from_bytes(&v))
//     .transpose()
//     .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?
//     .unwrap_or_default();
// let cumulative_burnt_fees = store_cumulative_values
//     .get_last("cumulative:burnt_fees".to_string())
//     .map(|v| decimal_from_bytes(&v))
//     .transpose()
//     .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?
//     .unwrap_or_default();
// let cumulative_rewards = store_cumulative_values
//     .get_last("cumulative:rewards".to_string())
//     .map(|v| decimal_from_bytes(&v))
//     .transpose()
//     .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?
//     .unwrap_or_default();
// let cumulative_transactions = store_cumulative_values
//     .get_last("cumulative:transactions".to_string())
//     .map(|v| decimal_from_bytes(&v))
//     .transpose()
//     .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?
//     .unwrap_or_default();
// let cumulative_size = store_cumulative_values
//     .get_last("cumulative:size".to_string())
//     .map(|v| decimal_from_bytes(&v))
//     .transpose()
//     .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?
//     .unwrap_or_default();

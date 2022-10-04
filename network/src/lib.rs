pub mod pb;

use std::str::FromStr;

use ethabi::Address;
use hex_literal::hex;
use prost::Message;
use substreams::{proto, store, Hex};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_helper::{
    bigdecimal::{BigDecimal, Zero},
    math::decimal_from_bytes,
    num_bigint::BigInt,
};

use crate::pb::network::v1::Network;

fn is_author_known(
    block: &eth::Block,
    store_known_block_authors: &store::StoreGet,
) -> Result<bool, substreams::errors::Error> {
    let author = block
        .header
        .clone()
        .map(|h| Address::from_slice(&h.coinbase))
        .map(|a| a.to_string())
        .unwrap_or_default();
    Ok(store_known_block_authors.get_last(author.clone()).is_some())
}

fn get_cumulative_value(
    key: &str,
    store: &store::StoreGet,
) -> Result<BigInt, substreams::errors::Error> {
    let value = store
        .get_last(format!("cumulative:{}", key))
        .unwrap_or_default();

    let value_str = String::from_utf8(value)
        .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?;

    Ok(BigInt::from_str(&value_str)
        .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?)
}

#[substreams::handlers::map]
fn map_network(
    block: eth::Block,
    store_daily_snapshots: store::StoreGet,
    store_hourly_snapshots: store::StoreGet,
    store_cumulative_values: store::StoreGet,
    store_known_block_authors: store::StoreGet,
) -> Result<Network, substreams::errors::Error> {
    let cumulative_unique_authors =
        get_cumulative_value("unique_authors", &store_cumulative_values)?
            + if is_author_known(&block, &store_known_block_authors)? {
                BigInt::zero()
            } else {
                BigInt::from(1)
            };

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

    Ok(Network {
        ..Default::default() // cumulative_unique_authors:
    })
}

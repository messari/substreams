use std::ops::Not;

use substreams::store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;
use substreams_helper::types::Source;

use crate::abi::chainlink_aggregator;
use crate::keyer;
use crate::pb::chainlink::v1::Aggregator;
use crate::pb::erc20_price::v1::Erc20Price;

#[substreams::handlers::store]
fn store_chainlink_price(
    block: eth::Block,
    store: StoreGetProto<Aggregator>,
    output: StoreSetProto<Erc20Price>,
) {
    for log in block.logs() {
        if let Some(event) = chainlink_aggregator::events::AnswerUpdated::match_and_decode(log) {
            let aggregator_address = Hex(log.address()).to_string();

            if let Some(aggregator) =
                store.get_last(keyer::chainlink_aggregator_key(&aggregator_address))
            {
                if ["USD", "DAI", "USDC", "USDT"]
                    .contains(&aggregator.quote_asset.unwrap().symbol.as_str())
                    .not()
                {
                    // TODO: add logic for handling `ETH` quote.
                    continue;
                }

                let token_price = event.current.to_decimal(aggregator.decimals);
                let token_address = aggregator.base_asset.clone().unwrap().address;

                let erc20price = Erc20Price {
                    token: aggregator.base_asset,
                    price_usd: token_price.to_string(),
                    block_number: block.number,
                    source: Source::ChainlinkAggregators as i32,
                };

                output.set(
                    log.ordinal(),
                    keyer::chainlink_asset_key(&token_address),
                    &erc20price,
                );
            }
        }
    }
}

use substreams::store::{StoreNew, StoreSet, StoreSetProto};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::factory;
use crate::keyer;
use crate::pb::erc20::v1::Erc20Token;
use crate::pb::uniswap::v1::PairCreatedEvent;

#[substreams::handlers::store]
fn store_pair_created_events(block: eth::Block, output: StoreSetProto<PairCreatedEvent>) {
    for log in block.logs() {
        if let Some(event) = factory::events::PairCreated::match_and_decode(log) {
            let token0_asset =
                substreams_helper::erc20::get_erc20_token(Hex(&event.token0).to_string()).unwrap();
            let token1_asset =
                substreams_helper::erc20::get_erc20_token(Hex(&event.token1).to_string()).unwrap();

            let pair_created_event = PairCreatedEvent {
                token0: Some(Erc20Token {
                    address: token0_asset.address,
                    name: token0_asset.name,
                    symbol: token0_asset.symbol,
                    decimals: token0_asset.decimals,
                }),
                token1: Some(Erc20Token {
                    address: token1_asset.address,
                    name: token1_asset.name,
                    symbol: token1_asset.symbol,
                    decimals: token1_asset.decimals,
                }),
                pair: Hex(event.pair.clone()).to_string(),
                factory: Hex(log.address()).to_string(),
            };

            output.set(
                log.ordinal(),
                keyer::pair_info_key(&pair_created_event.pair),
                &pair_created_event,
            );
        }
    }
}

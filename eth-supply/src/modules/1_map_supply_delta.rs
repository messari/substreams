use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth as pbeth;

use substreams_helper::block::BlockHandler;

use crate::helpers::{get_genesis_value, BigIntPbSerialize};
use crate::pb::eth_supply::v1::EthSupply;

#[substreams::handlers::map]
fn map_supply_delta(block: pbeth::v2::Block) -> Result<EthSupply, substreams::errors::Error> {
    let bh = BlockHandler::new(&block);
    let issuance = bh.issuance();

    Ok(EthSupply {
        genesis: get_genesis_value(&block).serialize().into(),
        block_rewards: issuance.block_rewards.serialize().into(),
        uncle_rewards: issuance.uncle_rewards.serialize().into(),
        burned: bh.burnt_fees().serialize().into(),
        total: BigInt::from(0).serialize().into(),

        block_hash: bh.hash(),
        block_number: bh.block_number(),
    })
}

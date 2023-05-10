use substreams::prelude::*;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::{pb::eth::v2::{self as eth}, Event, NULL_ADDRESS};
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreGetProto};
use crate::pb;

use crate::{pb::dex_amm::v3_0_3::{
    DataSource, MappedDataSources, PrunedTransaction, StoreInstruction, 
    Update, Swap, Deposit, Withdraw, CreateLiquidityPool, AddInt64, AddManyInt64, AddBigInt, AddManyBigInt}, 
    utils::UNISWAP_V3_FACTORY_SLICE, dex_amm::v_3_0_3::map

};
use crate::pb::dex_amm::v3_0_3::update::Type::{Swap as SwapType, Deposit as DepositType, Withdraw as WithdrawType, CreateLiquidityPool as CreateLiquidityPoolType};
use crate::pb::dex_amm::v3_0_3::store_instruction;

use crate::abi::factory as FactoryContract;

use crate::schema_lib::dex_amm::v_3_0_3::store_keys;

use crate::keyer::{get_data_source_key};

pub fn handle_pool_created(
    mapped_data_sources: &mut MappedDataSources, 
    pruned_transaction: &mut PrunedTransaction,
    pool_created_event: FactoryContract::events::PoolCreated, 
    call: &eth::Call, 
    log: &eth::Log
) {
    pruned_transaction.updates.push(
        Update {
            r#type: Some(CreateLiquidityPoolType(CreateLiquidityPool {
                protocol: UNISWAP_V3_FACTORY_SLICE.to_vec(),
                pool_address: pool_created_event.pool.clone(),
                input_tokens: vec![pool_created_event.token0.clone(), pool_created_event.token1.clone()],
                reward_tokens: vec![],
                fees: vec![],
                is_single_sided: false,
                tick: None,
                liquidity_token: None,
                liquidity_token_type: None,
            }))
        }
    );
}

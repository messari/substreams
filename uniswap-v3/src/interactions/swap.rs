use substreams::Hex;
use substreams_ethereum::{pb::eth::v2::{self as eth}};

use crate::{pb::dex_amm::v3_0_3::{
    MappedDataSources, PrunedTransaction, 
    Update, Swap}, 
    utils::UNISWAP_V3_FACTORY_SLICE
};

use crate::abi::pool as PoolContract;
use crate::schema_lib::dex_amm::v_3_0_3::store_keys;
use crate::sdk;
use crate::utils;

pub fn handle_swap(
    mapped_data_sources: &mut MappedDataSources, 
    pruned_transaction: &mut PrunedTransaction,
    swap_event: PoolContract::events::Swap, 
    call: &eth::Call, 
    log: &eth::Log
) {
    let pool_address_string: String = Hex(&call.address).to_string();
    pruned_transaction.create_swap(
        &call.address, 
        &UNISWAP_V3_FACTORY_SLICE.to_vec(), 
        &pruned_transaction.from.clone(), 
        &vec![swap_event.amount0.clone(), swap_event.amount1.clone()], 
        &swap_event.liquidity, 
        Some(&swap_event.tick), 
        log.index, 
        log.ordinal,
    );

    mapped_data_sources.add_liquidity_pool_cumulative_swap_count(
        &pool_address_string, 
        0, 
        1
    );
    mapped_data_sources.add_liquidity_pool_input_token_balances(
        &pool_address_string, 
        0, 
        &vec![swap_event.amount0.clone(), swap_event.amount1.clone()]
    );
    mapped_data_sources.add_liquidity_pool_cumulative_volume_token_amounts(
        &pool_address_string, 
        0, 
        &vec![utils::abs_bigint(&swap_event.amount0), utils::abs_bigint(&swap_event.amount1)]
    );
    mapped_data_sources.set_liquidity_pool_active_liquidity(
        &pool_address_string, 
        0, 
        &swap_event.liquidity
    );
}

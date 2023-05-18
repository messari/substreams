use substreams::Hex;
use substreams_ethereum::{pb::eth::v2::{self as eth}};

use crate::{pb::dex_amm::v3_0_3::{
    MappedDataSources, PrunedTransaction}, 
    utils::UNISWAP_V3_FACTORY_SLICE
};

use crate::abi::pool as PoolContract;

pub fn handle_burn(
    mapped_data_sources: &mut MappedDataSources, 
    pruned_transaction: &mut PrunedTransaction,
    burn_event: PoolContract::events::Burn, 
    call: &eth::Call, 
    log: &eth::Log
) {
    let pool_address_string: String = Hex(&call.address).to_string();
    pruned_transaction.create_withdraw(
        &call.address,
        &UNISWAP_V3_FACTORY_SLICE.to_vec(),
        &pruned_transaction.from.clone(),
        &burn_event.amount,
        &vec![burn_event.amount0.clone(), burn_event.amount1.clone()],
        None,
        Some(&burn_event.tick_lower),
        Some(&burn_event.tick_upper),
        log.index,
        log.ordinal,
    );

    mapped_data_sources.add_liquidity_pool_input_token_balances(
        &pool_address_string, 
        0, 
        &vec![burn_event.amount0.clone().neg(), burn_event.amount1.clone().neg()]
    );
    mapped_data_sources.add_liquidity_pool_total_liquidity(
        &pool_address_string, 
        0, 
        &burn_event.amount.neg()
    );
}

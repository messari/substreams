use substreams::{Hex, store::StoreGet};
use substreams_ethereum::{pb::eth::v2::{self as eth}};
use substreams_ethereum::NULL_ADDRESS;
use substreams::store;
use substreams::log;

use crate::{pb::dex_amm::v3_0_3::{
    MappedDataSources, PrunedTransaction}, 
    utils::UNISWAP_V3_FACTORY_SLICE
};

use crate::abi::pool as PoolContract;

pub fn prepare_burn_entity_changes(
    mapped_data_sources: &mut MappedDataSources, 
    pruned_transaction: &mut PrunedTransaction,
    burn_event: PoolContract::events::Burn, 
    call: &eth::Call, 
    log: &eth::Log,
    append_string_l1_store: &store::StoreGetArray<String>,
) {
    let pool_address_string: String = Hex(&call.address).to_string();
    let input_tokens = match append_string_l1_store.get_last([pool_address_string.as_str(), "inputTokens"].join(":")) {
        Some(input_tokens) => input_tokens.into_iter().map(|s| s.into_bytes()).collect(),
        None => {
            panic!("No input tokens found for pool address: {}", pool_address_string)
        }
    };
    pruned_transaction.create_withdraw_entity(
        &call.address,
        &UNISWAP_V3_FACTORY_SLICE.to_vec(),
        &pruned_transaction.from.clone(),
        &burn_event.amount,
        &input_tokens,
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
    mapped_data_sources.add_liquidity_pool_cumulative_withdraw_count(
        &pool_address_string, 
        0, 
        1
    );
}

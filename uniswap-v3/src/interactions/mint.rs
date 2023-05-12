use substreams::Hex;
use substreams_ethereum::{pb::eth::v2::{self as eth}};

use crate::{pb::dex_amm::v3_0_3::{
    MappedDataSources, PrunedTransaction, 
    Update, Deposit}, 
    utils::UNISWAP_V3_FACTORY_SLICE
};
use crate::pb::dex_amm::v3_0_3::update::Type::{Deposit as DepositType};
use crate::abi::pool as PoolContract;


pub fn handle_mint(
    mapped_data_sources: &mut MappedDataSources, 
    pruned_transaction: &mut PrunedTransaction,
    mint_event: PoolContract::events::Mint, 
    call: &eth::Call, 
    log: &eth::Log
) {
    let pool_address_string: String = Hex(&call.address).to_string();
    pruned_transaction.create_deposit(
        &call.address,
        &UNISWAP_V3_FACTORY_SLICE.to_vec(),
        &pruned_transaction.from.clone(),
        &mint_event.amount,
        &vec![mint_event.amount0.clone(), mint_event.amount1.clone()],
        None,
        Some(&mint_event.tick_lower),
        Some(&mint_event.tick_upper),
        log.index,
        log.ordinal,
    );

    mapped_data_sources.add_liquidity_pool_input_token_balances(
        &pool_address_string, 
        0, 
        &vec![mint_event.amount0.clone(), mint_event.amount1.clone()]
    );
    mapped_data_sources.add_liquidity_pool_total_liquidity(
        &pool_address_string, 
        0, 
        &mint_event.amount
    );
}

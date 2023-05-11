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
    pruned_transaction.updates.push(
        Update {
            r#type: Some(DepositType(Deposit {
                pool: call.address.clone(),
                protocol: UNISWAP_V3_FACTORY_SLICE.to_vec(),
                account: pruned_transaction.from.clone(),
                position: None,
                liquidity: Some(mint_event.amount.clone().into()),
                input_token_amounts: vec![mint_event.amount0.clone().into(), mint_event.amount1.clone().into()],
                tick_lower: Some(mint_event.tick_lower.clone().into()),
                tick_upper:Some(mint_event.tick_upper.clone().into()),
                log_index: Some(log.index.into()),
                log_ordinal: Some(log.ordinal.into()),
            }))
        }
    );
}

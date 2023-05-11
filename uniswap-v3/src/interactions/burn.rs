use substreams_ethereum::{pb::eth::v2::{self as eth}};

use crate::{pb::dex_amm::v3_0_3::{
    MappedDataSources, PrunedTransaction, 
    Update, Withdraw}, 
    utils::UNISWAP_V3_FACTORY_SLICE

};
use crate::pb::dex_amm::v3_0_3::update::Type::{Withdraw as WithdrawType};

use crate::abi::pool as PoolContract;

pub fn handle_burn(
    mapped_data_sources: &mut MappedDataSources, 
    pruned_transaction: &mut PrunedTransaction,
    burn_event: PoolContract::events::Burn, 
    call: &eth::Call, 
    log: &eth::Log
) {
    pruned_transaction.updates.push(
        Update {
            r#type: Some(WithdrawType(Withdraw {
                pool: call.address.clone(),
                protocol: UNISWAP_V3_FACTORY_SLICE.to_vec(),
                account: pruned_transaction.from.clone(),
                position: None,
                liquidity: Some(burn_event.amount.clone().into()),
                input_token_amounts: vec![burn_event.amount0.clone().into(), burn_event.amount1.clone().into()],
                tick_lower: Some(burn_event.tick_lower.clone().into()),
                tick_upper:Some(burn_event.tick_upper.clone().into()),
                log_index: Some(log.index.into()),
                log_ordinal: Some(log.ordinal.into()),
            }))
        }
    );
}

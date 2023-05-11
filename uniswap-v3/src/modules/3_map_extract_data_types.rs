use substreams::prelude::*;
use substreams::errors::Error;
use substreams_ethereum::{pb::eth::v2::{self as eth}, Event};
use substreams::store::{StoreGetProto};

use crate::{pb::dex_amm::v3_0_3::{
    DataSource, MappedDataSources, PrunedTransaction, StoreInstruction, 
    Update}
};
use crate::abi::pool as PoolContract;
use crate::abi::factory as FactoryContract;

use crate::interactions;
use crate::constants;

use crate::keyer::{get_data_source_key};


fn initialize_pruned_transaction(transaction_trace: eth::TransactionTrace) -> PrunedTransaction {
    PrunedTransaction {
        hash: transaction_trace.hash.clone(),
        from: transaction_trace.from.clone(),
        to: transaction_trace.to.clone(),
        nonce: Some(transaction_trace.nonce.into()),
        gas_limit: Some(transaction_trace.gas_limit.into()),
        gas_used: Some(transaction_trace.gas_used.into()),
        gas_price: Some(constants::BIGINT_ZERO.clone().into()),
        updates: Vec::<Update>::new(),
    }
}

#[substreams::handlers::map]
pub fn map_extract_data_types(
    block: eth::Block,
    data_sources_store: StoreGetProto<DataSource>,
) -> Result<MappedDataSources, Error>{
    let mut mapped_data_sources = MappedDataSources {
        pruned_transactions: Vec::<PrunedTransaction>::new(),
        store_instructions: Vec::<StoreInstruction>::new(),
    };

    for transaction_trace in block.transaction_traces {
        let mut pruned_transaction: PrunedTransaction = initialize_pruned_transaction(transaction_trace.clone());
 
        for call_view in transaction_trace.calls() {
            if let Some(data_source) = data_sources_store.get_last(get_data_source_key(&call_view.call.address)) {
                match data_source.data_source_type {
                    0 => {
                        for log in &call_view.call.logs {
                            if let Some(swap_event) = PoolContract::events::Swap::match_and_decode(&log) {
                                interactions::swap::handle_swap(&mut mapped_data_sources, &mut pruned_transaction, swap_event, call_view.call, log);
                            } else if let Some(mint_event) = PoolContract::events::Mint::match_and_decode(&log) {
                                interactions::mint::handle_mint(&mut mapped_data_sources, &mut pruned_transaction, mint_event, call_view.call, log);
                            } else if let Some(burn_event) = PoolContract::events::Burn::match_and_decode(&log) {
                                interactions::burn::handle_burn(&mut mapped_data_sources, &mut pruned_transaction, burn_event, call_view.call, log);
                            }
                        }
                    }
                    1 => {
                        for log in &call_view.call.logs {
                            if let Some(pool_created_event) = FactoryContract::events::PoolCreated::match_and_decode(&log) {
                                interactions::pool_created::handle_pool_created(&mut mapped_data_sources, &mut pruned_transaction, pool_created_event, call_view.call, log);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        if pruned_transaction.updates.len() > 0 {
            mapped_data_sources.pruned_transactions.push(pruned_transaction);
        }
    }
    Ok(mapped_data_sources)
}

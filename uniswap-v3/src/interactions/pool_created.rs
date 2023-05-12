use substreams_ethereum::{pb::eth::v2::{self as eth}};

use crate::{pb::dex_amm::v3_0_3::{
    MappedDataSources, PrunedTransaction, 
    Update, CreateLiquidityPool, CreateToken}, 
    utils::UNISWAP_V3_FACTORY_SLICE

};
use crate::pb::dex_amm::v3_0_3::update::Type::{CreateLiquidityPool as CreateLiquidityPoolType, CreateToken as CreateTokenType};
use crate::abi::factory as FactoryContract;
use crate::contract::erc20;

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
    let token0 = erc20::Erc20::new(pool_created_event.token0.clone()).as_struct();
    let token1 = erc20::Erc20::new(pool_created_event.token1.clone()).as_struct();
    pruned_transaction.create_token(
        &pool_created_event.token0,
        &token0.name,
        &token0.symbol,
        token0.decimals,
    );
    pruned_transaction.create_token(
        &pool_created_event.token1,
        &token1.name,
        &token1.symbol,
        token1.decimals,
    );
    // if let Some(create_token) = create_token(&pool_created_event.token0) {
    //     pruned_transaction.updates.push(
    //         Update {
    //             r#type: Some(CreateTokenType(create_token))
    //         }
    //     );
    // }
    // if let Some(create_token) = create_token(&pool_created_event.token1) {
    //     pruned_transaction.updates.push(
    //         Update {
    //             r#type: Some(CreateTokenType(create_token))
    //         }
    //     );
    // }
}

use substreams::store::{StoreGet, StoreGetString};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::{Event, NULL_ADDRESS};
use substreams_helper::event_handler::EventHandler;
use substreams_helper::hex::Hexable;

use crate::abi::erc20::events::Transfer;
use crate::abi::pool_contract::functions::Minter;
use crate::abi::registry_contract::events::{
    BasePoolAdded1 as BasePoolAdded, BasePoolAdded1 as BasePoolAddedWithImplementation,
    PlainPoolDeployed1 as PlainPoolDeployed, PlainPoolDeployed2 as PlainPoolDeployedWithPool,
};
use crate::abi::registry_contract::events::{
    CryptoPoolDeployed, MetaPoolDeployed, PoolAdded, TricryptoPoolDeployed,
};
use crate::pb::contract::v1::{Pool, Pools};

#[substreams::handlers::map]
pub fn map_pool_created(
    blk: eth::Block,
    factories_store: StoreGetString,
) -> Result<Pools, substreams::errors::Error> {
    let mut pools: Vec<Pool> = vec![];

    get_pool_from_deploy_crypto_pool_deployed_events(&blk, &factories_store, &mut pools);
    get_pool_from_tri_deploy_crypto_pool_deployed_events(&blk, &factories_store, &mut pools);

    get_pool_from_plain_pool_deployed_events(&blk, &factories_store, &mut pools);
    get_pool_from_plain_pool_deployed_with_pool_events(&blk, &factories_store, &mut pools);

    get_pool_from_base_pool_added_events(&blk, &factories_store, &mut pools);
    get_pool_from_base_pool_added_with_implementation_events(&blk, &factories_store, &mut pools);

    get_pool_from_meta_pool_deployed_events(&blk, &factories_store, &mut pools);
    get_pool_pool_addded_events(&blk, &factories_store, &mut pools);

    Ok(Pools { pools })
}

fn get_pool_from_deploy_crypto_pool_deployed_events(
    blk: &eth::Block,
    factories: &StoreGetString,
    pools: &mut Vec<Pool>,
) {
    let mut on_crypto_pool_deployed =
        |event: CryptoPoolDeployed, tx: &eth::TransactionTrace, log: &eth::Log| {
            let lp_address = event.token;
            let pool_address_option = Minter {}.call(lp_address);

            let pool_address = match pool_address_option {
                Some(x) => x.to_hex(),
                None => return,
            };

            pools.push(Pool {
                address: pool_address,
                log_index: log.index as i64,
                block_number: blk.number as i64,
                block_timestmap: Some(blk.timestamp().to_owned()),
                transaction_index: tx.index as i64,
                transaction_hash: tx.hash.to_hex(),
            })
        };

    let mut eh = EventHandler::new(blk);
    eh.filter_by_address(factories);

    eh.on::<CryptoPoolDeployed, _>(&mut on_crypto_pool_deployed);
    eh.handle_events();
}

fn get_pool_from_tri_deploy_crypto_pool_deployed_events(
    blk: &eth::Block,
    factories: &StoreGetString,
    pools: &mut Vec<Pool>,
) {
    let mut on_tri_crypto_pool_deployed =
        |event: TricryptoPoolDeployed, tx: &eth::TransactionTrace, log: &eth::Log| {
            pools.push(Pool {
                address: event.pool.to_hex(),
                log_index: log.index as i64,
                block_number: blk.number as i64,
                block_timestmap: Some(blk.timestamp().to_owned()),
                transaction_index: tx.index as i64,
                transaction_hash: tx.hash.to_hex(),
            })
        };

    let mut eh = EventHandler::new(blk);
    eh.filter_by_address(factories);

    eh.on::<TricryptoPoolDeployed, _>(&mut on_tri_crypto_pool_deployed);
    eh.handle_events();
}

fn find_pool_from_plain_pool_deployed_event(
    tx: &eth::TransactionTrace,
    log: &eth::Log,
) -> Option<String> {
    tx.calls
        .iter()
        .filter(|call| !call.state_reverted)
        .flat_map(|call| call.logs.iter())
        .find_map(|transfer_log| {
            if let Some(transfer) = Transfer::match_and_decode(transfer_log) {
                if transfer.from == NULL_ADDRESS
                    && (transfer.to == transfer_log.address || transfer.to == log.address)
                {
                    return Some(transfer_log.address.to_hex());
                }
            }
            None
        })
}

fn get_pool_from_plain_pool_deployed_events(
    blk: &eth::Block,
    factories: &StoreGetString,
    pools: &mut Vec<Pool>,
) {
    let mut on_plain_pool_deployed =
        |_event: PlainPoolDeployed, tx: &eth::TransactionTrace, log: &eth::Log| {
            let pool_address = match find_pool_from_plain_pool_deployed_event(tx, log) {
                Some(x) => x,
                None => return,
            };

            pools.push(Pool {
                address: pool_address,
                log_index: log.index as i64,
                block_number: blk.number as i64,
                block_timestmap: Some(blk.timestamp().to_owned()),
                transaction_index: tx.index as i64,
                transaction_hash: tx.hash.to_hex(),
            })
        };

    let mut eh = EventHandler::new(blk);
    eh.filter_by_address(factories);

    eh.on::<PlainPoolDeployed, _>(&mut on_plain_pool_deployed);
    eh.handle_events();
}

fn get_pool_from_plain_pool_deployed_with_pool_events(
    blk: &eth::Block,
    factories: &StoreGetString,
    pools: &mut Vec<Pool>,
) {
    let mut on_plain_pool_deployed =
        |event: PlainPoolDeployedWithPool, tx: &eth::TransactionTrace, log: &eth::Log| {
            pools.push(Pool {
                address: event.pool.to_hex(),
                log_index: log.index as i64,
                block_number: blk.number as i64,
                block_timestmap: Some(blk.timestamp().to_owned()),
                transaction_index: tx.index as i64,
                transaction_hash: tx.hash.to_hex(),
            })
        };

    let mut eh = EventHandler::new(blk);
    eh.filter_by_address(factories);

    eh.on::<PlainPoolDeployedWithPool, _>(&mut on_plain_pool_deployed);
    eh.handle_events();
}

fn get_pool_from_base_pool_added_events(
    blk: &eth::Block,
    factories: &StoreGetString,
    pools: &mut Vec<Pool>,
) {
    let mut on_base_pool_added =
        |event: BasePoolAdded, tx: &eth::TransactionTrace, log: &eth::Log| {
            pools.push(Pool {
                address: event.base_pool.to_hex(),
                log_index: log.index as i64,
                block_number: blk.number as i64,
                block_timestmap: Some(blk.timestamp().to_owned()),
                transaction_index: tx.index as i64,
                transaction_hash: tx.hash.to_hex(),
            })
        };

    let mut eh = EventHandler::new(blk);
    eh.filter_by_address(factories);

    eh.on::<BasePoolAdded, _>(&mut on_base_pool_added);
    eh.handle_events();
}

fn get_pool_from_base_pool_added_with_implementation_events(
    blk: &eth::Block,
    factories: &StoreGetString,
    pools: &mut Vec<Pool>,
) {
    let mut on_base_pool_added =
        |event: BasePoolAddedWithImplementation, tx: &eth::TransactionTrace, log: &eth::Log| {
            pools.push(Pool {
                address: event.base_pool.to_hex(),
                log_index: log.index as i64,
                block_number: blk.number as i64,
                block_timestmap: Some(blk.timestamp().to_owned()),
                transaction_index: tx.index as i64,
                transaction_hash: tx.hash.to_hex(),
            })
        };

    let mut eh = EventHandler::new(blk);
    eh.filter_by_address(factories);

    eh.on::<BasePoolAddedWithImplementation, _>(&mut on_base_pool_added);
    eh.handle_events();
}

fn get_pool_from_meta_pool_deployed_events(
    blk: &eth::Block,
    factories: &StoreGetString,
    pools: &mut Vec<Pool>,
) {
    let mut on_meta_pool_deployed =
        |_event: MetaPoolDeployed, tx: &eth::TransactionTrace, log: &eth::Log| {
            let pool_address = match find_pool_from_plain_pool_deployed_event(tx, log) {
                Some(x) => x,
                None => return,
            };

            pools.push(Pool {
                address: pool_address,
                log_index: log.index as i64,
                block_number: blk.number as i64,
                block_timestmap: Some(blk.timestamp().to_owned()),
                transaction_index: tx.index as i64,
                transaction_hash: tx.hash.to_hex(),
            })
        };

    let mut eh = EventHandler::new(blk);
    eh.filter_by_address(factories);

    eh.on::<MetaPoolDeployed, _>(&mut on_meta_pool_deployed);
    eh.handle_events();
}

fn get_pool_pool_addded_events(
    blk: &eth::Block,
    factories: &StoreGetString,
    pools: &mut Vec<Pool>,
) {
    let mut on_pool_added = |event: PoolAdded, tx: &eth::TransactionTrace, log: &eth::Log| {
        pools.push(Pool {
            address: event.pool.to_hex(),
            log_index: log.index as i64,
            block_number: blk.number as i64,
            block_timestmap: Some(blk.timestamp().to_owned()),
            transaction_index: tx.index as i64,
            transaction_hash: tx.hash.to_hex(),
        })
    };

    let mut eh = EventHandler::new(blk);
    eh.filter_by_address(factories);

    eh.on::<PoolAdded, _>(&mut on_pool_added);
    eh.handle_events();
}

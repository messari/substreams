use substreams::store::{StoreAdd, StoreAddBigInt, StoreGet, StoreGetProto, StoreNew};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::NULL_ADDRESS;

use substreams_helper::event_handler::EventHandler;
use substreams_helper::hex::Hexable;

use crate::abi::Pool::events::Transfer;
use crate::common::traits::PoolAddresser;
use crate::{pb::pancake::v2::Pool, store_key::StoreKey};

#[substreams::handlers::store]
pub fn store_output_token_supply(
    block: eth::Block,
    pool_store: StoreGetProto<Pool>,
    output_store: StoreAddBigInt,
) {
    let mut on_transfer = |event: Transfer, _tx: &eth::TransactionTrace, log: &eth::Log| {
        let is_burn = event.to == NULL_ADDRESS;
        let is_mint = event.from == NULL_ADDRESS;

        if !(is_mint || is_burn) || (is_mint && is_burn) {
            return;
        }

        let value = event.value;
        let pool_address = log.address.to_hex();

        if is_burn {
            output_store.add(
                log.ordinal,
                StoreKey::OutputTokenBalance.get_unique_pool_key(&pool_address),
                value.neg(),
            );
        } else {
            output_store.add(
                log.ordinal,
                StoreKey::OutputTokenBalance.get_unique_pool_key(&pool_address),
                value,
            );
        }
    };

    let mut eh = EventHandler::new(&block);
    eh.filter_by_address(PoolAddresser { store: &pool_store });
    eh.on::<Transfer, _>(&mut on_transfer);
    eh.handle_events();
}

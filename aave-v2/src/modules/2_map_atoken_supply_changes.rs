use std::collections::HashMap;

use ethabi::ethereum_types::Address;
use substreams::scalar::BigInt;
use substreams::store::StoreGet;
use substreams::store::StoreGetProto;
use substreams_ethereum::pb::eth as pbeth;

use substreams_helper::block::BlockHandler;
use substreams_helper::common::HasAddresser;
use substreams_helper::event_handler::EventHandler;
use substreams_helper::hex::Hexable;
use substreams_helper::storage;
use substreams_helper::storage::get_storage_changes_for_addresses;
use substreams_helper::storage::ABIEncodeable;

use crate::abi::ERC20::events::Transfer;
use crate::pb::aave_v2::v1::Contract;
use crate::pb::aave_v2::v1::{ATokenBalance, ATokenBalances, ATokenSupplies, ATokenSupply};

struct ATokenAddresser<'a> {
    store: &'a StoreGetProto<Contract>,
}
impl<'a> ATokenAddresser<'a> {
    fn has_address(&self, key: Address) -> bool {
        let contract = self.store.get_last(key.to_hex());
        if contract.is_none() {
            return false;
        }

        let token = contract.unwrap().token;
        if token.is_none() {
            return false;
        }

        return true;
    }
}
impl<'a> HasAddresser for ATokenAddresser<'a> {
    fn has_address(&self, key: Address) -> bool {
        return self.has_address(key);
    }
}

#[substreams::handlers::map]
fn map_atoken_supply_changes(
    block: pbeth::v2::Block,
    store: StoreGetProto<Contract>,
) -> Result<ATokenSupplies, substreams::errors::Error> {
    let addresser = ATokenAddresser { store: &store };
    let bh = BlockHandler::new(&block);
    let changes = get_storage_changes_for_addresses(&addresser, &block);

    // This is where AAVE V2 AToken contracts store the scaled_supply variable.
    let scaled_supply_storage_key = storage::Uint256 {
        slot: BigInt::from(54),
        value: BigInt::zero(),
    }
    .storage_key();

    let mut supplies = HashMap::<Vec<u8>, ATokenSupply>::new();
    for change in changes {
        if change.change.key != scaled_supply_storage_key {
            continue;
        }

        let scaled_supply = BigInt::from_unsigned_bytes_be(change.change.new_value.as_slice());
        let token = addresser
            .store
            .get_last(change.change.address.clone().to_hex())
            .unwrap();
        let supply = ATokenSupply {
            timestamp: bh.timestamp(),
            block_hash: block.hash.clone().to_hex(),
            a_token: Some(token.token.unwrap()),
            scaled_supply: Some(scaled_supply.into()),
        };
        supplies.insert(change.change.address.clone(), supply);
    }

    Ok(ATokenSupplies {
        supplies: supplies.values().cloned().collect(),
    })
}

#[substreams::handlers::map]
fn map_atoken_balances(
    block: pbeth::v2::Block,
    store: StoreGetProto<Contract>,
) -> Result<ATokenBalances, substreams::errors::Error> {
    let mut balances: Vec<ATokenBalance> = vec![];

    get_balances(&block, &mut balances, &store);
    Ok(ATokenBalances { balances })
}

fn get_balances(
    block: &pbeth::v2::Block,
    balances: &mut Vec<ATokenBalance>,
    store: &StoreGetProto<Contract>,
) {
    let addresser = ATokenAddresser { store: &store };

    let mut on_transfer = |ev: Transfer, tx: &pbeth::v2::TransactionTrace, log: &pbeth::v2::Log| {
        let from = Address::from_slice(ev.from.as_slice());
        let to = Address::from_slice(ev.to.as_slice());

        let balance_mapping = storage::Mapping {
            slot: BigInt::from(52),
        };
        let from_key = balance_mapping.storage_key(&from);
        let to_key = balance_mapping.storage_key(&to);

        for call in tx.calls.clone() {
            for change in call.storage_changes {
                if change.address != log.address {
                    continue;
                }

                if change.key != from_key && change.key != to_key {
                    continue;
                }

                let mut address = from.to_hex();
                if change.key == to_key {
                    address = to.to_hex();
                }

                balances.push(ATokenBalance {
                    a_token: store.get_last(log.address.to_hex()).unwrap().token,
                    address: address,
                    scaled_balance: Some(
                        BigInt::abi_decode(change.new_value.as_slice())
                            .unwrap()
                            .into(),
                    ),
                })
            }
        }
    };

    let mut eh = EventHandler::new(block);
    eh.filter_by_address(addresser);
    eh.on::<Transfer, _>(&mut on_transfer);
    eh.handle_events();
}

use std::str::FromStr;

use ethabi::ethereum_types::Address;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::pb::eth::{self as pbeth};

use substreams::scalar::BigInt;
use substreams_helper::hex::Hexable;
use substreams_helper::storage::get_keccak_preimages_for_addresses;
use substreams_helper::storage::ABIEncodeable;
use substreams_helper::storage::Array;
use substreams_helper::storage::Mapping;
use substreams_helper::storage::StorageLayout;
use substreams_helper::storage::Struct;
use substreams_helper::storage::Uint128;
use substreams_helper::storage::{get_storage_changes_for_addresses, StorageChange};

use crate::constants::{
    SDS_CONTRACT_BALANCE_SLOT, SDS_TOKEN_CONTRACT, SNX_TOKEN_STATE_BALANCE_SLOT,
    SNX_TOKEN_STATE_CONTRACT,
};
use crate::pb::synthetix::v1::{TokenBalance, TokenBalances};

#[substreams::handlers::map]
fn map_snx_balances(block: pbeth::v2::Block) -> Result<TokenBalances, substreams::errors::Error> {
    let address = Address::from_str(SNX_TOKEN_STATE_CONTRACT).unwrap();
    let sds_address = Address::from_str(SDS_TOKEN_CONTRACT).unwrap();
    let mut balances: Vec<TokenBalance> = vec![];

    let changes = get_storage_changes_for_addresses(&vec![address, sds_address], &block);

    for change in changes.clone() {
        let balance = snx_balance_from_storage_change(&change, &block);
        if balance.is_some() {
            balances.push(balance.unwrap());
        }

        let mut sds_balances = sds_balance_from_storage_change(
            changes.clone(),
            get_keccak_preimages_for_addresses(&sds_address, &block),
            &block,
        );

        balances.append(&mut sds_balances);
    }

    Ok(TokenBalances { balances })
}

fn snx_balance_from_storage_change(change: &StorageChange, block: &Block) -> Option<TokenBalance> {
    if change.preimage.is_none() {
        return None;
    }

    let balances_mapping = Mapping {
        slot: BigInt::from(SNX_TOKEN_STATE_BALANCE_SLOT),
    };

    let preimage = change.preimage.as_ref().unwrap().to_owned();
    if !balances_mapping.preimage_in_slot(preimage.clone()) {
        return None;
    }

    let holder = balances_mapping
        .key_from_preimage::<Address>(preimage)
        .unwrap();
    let amount = BigInt::abi_decode(change.change.new_value.clone()).unwrap();
    Some(TokenBalance {
        token: change.change.address.to_hex(),
        holder: holder.to_hex(),
        balance: Some(amount.into()),
        timestamp: Some(block.into()),
    })
}

fn sds_balance_from_storage_change(
    changes: Vec<StorageChange>,
    preimages: Vec<Vec<u8>>,
    block: &Block,
) -> Vec<TokenBalance> {
    if changes.len() == 0 {
        return vec![];
    }

    let token = changes[0].change.address.clone();
    let mut sds_balances = vec![];
    let balances_array_mapping = Mapping {
        slot: BigInt::from(SDS_CONTRACT_BALANCE_SLOT),
    };

    let preimages = balances_array_mapping.filter_keccak_preimages(preimages);
    for preimage in preimages.clone() {
        let holder = balances_array_mapping
            .key_from_preimage::<Address>(preimage.preimage.clone())
            .unwrap();

        let mut balance = Struct::new(BigInt::zero());
        balance.add_field("balance", Uint128::default());
        balance.add_field("timestamp", Uint128::default());
        let mut arr = Array::new(preimage.slot, balance);

        let mut last_index = BigInt::from(0);
        let mut most_recent_balance = BigInt::from(0);
        for change in arr.filter_array_changes(changes.clone(), BigInt::from(1000)) {
            let index = arr
                .infer_array_index_from_storage_key(change.change.key.clone())
                .unwrap();

            if index >= last_index {
                last_index = index;
                arr.decode(vec![change.change.new_value.clone()], None)
                    .unwrap();
                most_recent_balance = arr.item.get::<Uint128>("balance").value.clone();
            }
        }

        sds_balances.push(TokenBalance {
            token: token.to_hex(),
            holder: holder.to_hex(),
            balance: Some(most_recent_balance.into()),
            timestamp: Some(block.into()),
        });
    }
    sds_balances
}

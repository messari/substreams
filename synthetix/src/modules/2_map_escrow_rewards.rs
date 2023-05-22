use std::str::FromStr;

use ethabi::ethereum_types::Address;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::{self as pbeth};

use substreams_helper::hex::Hexable;
use substreams_helper::storage::{
    get_storage_changes_for_addresses, ABIEncodeable, Mapping, StorageChange,
};

use crate::constants::{
    ESCROW_REWARDS_CONTRACT_V1, ESCROW_REWARDS_CONTRACT_V2, ESCROW_REWARDS_ESCROWED_BALANCE_SLOT,
    ESCROW_REWARDS_VESTED_BALANCE_SLOT,
};
use crate::pb::synthetix::v1::escrow_reward::EscrowContractVersion;
use crate::pb::synthetix::v1::Timestamp;
use crate::pb::synthetix::v1::{escrow_reward::BalanceType, EscrowReward, EscrowRewards};

#[substreams::handlers::map]
fn map_escrow_rewards(block: pbeth::v2::Block) -> Result<EscrowRewards, substreams::errors::Error> {
    let mut v2_rewards = get_v2_escrow_rewards(&block);
    let mut v1_rewards = get_v1_escrow_rewards(&block);

    let mut rewards = vec![];
    rewards.append(&mut v2_rewards);
    rewards.append(&mut v1_rewards);
    Ok(EscrowRewards { rewards })
}

fn get_v1_escrow_rewards(block: &pbeth::v2::Block) -> Vec<EscrowReward> {
    let v1_contract = Address::from_str(ESCROW_REWARDS_CONTRACT_V1).unwrap();
    return get_escrow_rewards(block, v1_contract, EscrowContractVersion::V1);
}

fn get_v2_escrow_rewards(block: &pbeth::v2::Block) -> Vec<EscrowReward> {
    let v2_contract = Address::from_str(ESCROW_REWARDS_CONTRACT_V2).unwrap();
    return get_escrow_rewards(block, v2_contract, EscrowContractVersion::V2);
}

fn get_escrow_rewards(
    block: &pbeth::v2::Block,
    contract: Address,
    version: EscrowContractVersion,
) -> Vec<EscrowReward> {
    let changes = get_storage_changes_for_addresses(&contract, &block);

    let timestamp: Timestamp = block.into();
    let mut rewards = vec![];
    for change in changes.as_slice() {
        let vested_balance = get_vested_balance_from_change(change, &version);
        if let Some(mut balance) = vested_balance {
            balance.timestamp = Some(timestamp.clone());
            rewards.push(balance);
        }

        let escrowed_balance = get_escrowed_balance_from_change(change, &version);
        if let Some(mut balance) = escrowed_balance {
            balance.timestamp = Some(timestamp.clone());
            rewards.push(balance);
        }
    }
    return rewards;
}

fn get_escrowed_balance_from_change(
    change: &StorageChange,
    version: &EscrowContractVersion,
) -> Option<EscrowReward> {
    return get_balance_from_mapping_change(
        change,
        &Mapping {
            slot: BigInt::from(ESCROW_REWARDS_ESCROWED_BALANCE_SLOT),
        },
        BalanceType::Escrowed,
        version,
    );
}

fn get_vested_balance_from_change(
    change: &StorageChange,
    version: &EscrowContractVersion,
) -> Option<EscrowReward> {
    return get_balance_from_mapping_change(
        change,
        &Mapping {
            slot: BigInt::from(ESCROW_REWARDS_VESTED_BALANCE_SLOT),
        },
        BalanceType::Vested,
        version,
    );
}

fn get_balance_from_mapping_change(
    change: &StorageChange,
    mapping: &Mapping,
    balance_type: BalanceType,
    version: &EscrowContractVersion,
) -> Option<EscrowReward> {
    if let Some(preimage) = &change.preimage {
        let holder = mapping.key_from_preimage::<Address>(preimage.to_owned());
        if holder.is_none() {
            return None;
        }

        let balance = BigInt::abi_decode(change.change.new_value.clone()).unwrap();
        return Some(EscrowReward {
            holder: holder.unwrap().to_hex(),
            balance: Some(balance.into()),
            balance_type: balance_type.into(),
            escrow_contract_version: version.to_owned().into(),
            timestamp: None,
        });
    }
    None
}

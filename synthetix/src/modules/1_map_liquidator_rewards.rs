use std::str::FromStr;
use std::vec;

use ethabi::Address;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::{self as pbeth};

use substreams_helper::hex::Hexable;
use substreams_helper::storage::get_storage_changes_for_addresses;
use substreams_helper::storage::Mapping;
use substreams_helper::storage::StorageLayout;
use substreams_helper::storage::Struct;
use substreams_helper::storage::Uint128;
use substreams_helper::storage::Uint256;

use crate::constants::{
    LIQUIDATOR_REWARDS_ACCUMULATED_REWARDS_PER_SHARE_SLOT, LIQUIDATOR_REWARDS_CONTRACT,
    LIQUIDATOR_REWARDS_CONTRACT_ENTRIES_SLOT,
};
use crate::pb::synthetix::v1::BigInt as pbBigInt;
use crate::pb::synthetix::v1::LiquidatorReward;
use crate::pb::synthetix::v1::LiquidatorRewards;

#[substreams::handlers::map]
fn map_liquidation_rewards(
    block: pbeth::v2::Block,
) -> Result<LiquidatorRewards, substreams::errors::Error> {
    let changes = get_storage_changes_for_addresses(
        &Address::from_str(LIQUIDATOR_REWARDS_CONTRACT).unwrap(),
        &block,
    );

    let mut accumulated_rewards_per_share: Option<pbBigInt> = None;
    let mut account_rewards_entry = Struct::new(BigInt::from(0));
    account_rewards_entry.add_field("claimable", Uint128::default());
    account_rewards_entry.add_field("entryAccumulatedRewards", Uint128::default());
    let entries = Mapping {
        slot: BigInt::from(LIQUIDATOR_REWARDS_CONTRACT_ENTRIES_SLOT),
    };

    let mut rewards = vec![];
    for change in changes {
        if let Some(preimage) = change.preimage {
            if let Some(account) = entries.key_from_preimage::<Address>(preimage) {
                account_rewards_entry
                    .decode(vec![change.change.clone().new_value], None)
                    .unwrap();

                let claimable = account_rewards_entry
                    .get::<Uint128>("claimable")
                    .value
                    .to_owned();
                let entry_accumulated_rewards = account_rewards_entry
                    .get::<Uint128>("entryAccumulatedRewards")
                    .value
                    .to_owned();
                rewards.push(LiquidatorReward {
                    claimable: Some(claimable.into()),
                    entry_accumulated_rewards: Some(entry_accumulated_rewards.into()),
                    account: account.to_hex(),
                });
            }
        }

        let mut accumulated = Uint256::default();
        accumulated.set_slot(BigInt::from(
            LIQUIDATOR_REWARDS_ACCUMULATED_REWARDS_PER_SHARE_SLOT,
        ));
        if accumulated.storage_key() == change.change.key {
            accumulated
                .decode(vec![change.change.new_value], None)
                .unwrap();
            accumulated_rewards_per_share = Some(accumulated.value.into());
        }
    }

    Ok(LiquidatorRewards {
        rewards: rewards,
        accumulated_rewards_per_share: accumulated_rewards_per_share,
    })
}

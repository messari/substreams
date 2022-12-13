use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::v2::balance_change::Reason;
use substreams_ethereum::pb::eth::v2::{self as eth, BalanceChange};
use substreams_ethereum::scalar::BigIntSign;

use crate::utils::{BigIntDeserializeExt, get_latest_day, get_latest_hour};
use ethabi::ethereum_types::Address;

pub struct BlockHandler<'a>(&'a eth::Block);

impl<'a> BlockHandler<'a> {
    pub fn new(block: &'a eth::Block) -> Self {
        Self(block)
    }

    /// Returns the timestamp for the start of the most recent day
    pub fn timestamp(&self) -> i64 {
        if let Some(header) = self.0.header.as_ref() {
            if let Some(timestamp) = header.timestamp.as_ref() {
                timestamp.seconds
            } else {
                panic!("Unable to find timestamp in block header!\nBlock: {:?}", self.0);
            }
        } else {
            panic!("Unable to find header in block!\nBlock: {:?}", self.0);
        }
    }

    /// Returns the timestamp for the start of the most recent day
    pub fn days_timestamp(&self) -> i64 {
        get_latest_day(self.timestamp())
    }

    /// Returns the timestamp for the start of the most recent hour
    pub fn hours_timestamp(&self) -> i64 {
        get_latest_hour(self.timestamp())
    }

    pub fn author(&self) -> Option<String> {
        if let Some(header) = self.0.header.as_ref() {
            Some(Address::from_slice(&header.coinbase).to_string())
        } else {
            None
        }
    }

    pub fn block_size(&self) -> BigInt {
        BigInt::from(self.0.size)
    }

    /// Get the supply for the current block
    pub fn supply(&self) -> BigInt {
        self.0
            .balance_changes
            .iter()
            .map(|balance_change| {
                // Only need to consider positive movements of balance change as the negative movements
                // are just from the other side of the positive balance changes
                get_balance_gain(balance_change)
            })
            .fold(BigInt::zero(), |sum, val| sum + val)
    }

    /// Gets the number of transactions seen in the block
    pub fn transactions(&self) -> BigInt {
        BigInt::from(self.0.transaction_traces.len() as u64)
    }

    pub fn difficulty(&self) -> BigInt {
        if let Some(header) = self.0.header.as_ref() {
            if let Some(difficulty) = header.difficulty.as_ref() {
                return difficulty.deserialize();
            }
        }
        BigInt::zero()
    }

    pub fn gas_used(&self) -> BigInt {
        if let Some(header) = self.0.header.as_ref() {
            BigInt::from(header.gas_used)
        } else {
            BigInt::zero()
        }
    }

    pub fn chunks(&self) -> BigInt {
        if let Some(header) = self.0.header.as_ref() {
            BigInt::from(header.gas_used)
        } else {
            BigInt::zero()
        }
    }

    pub fn gas_limit(&self) -> BigInt {
        if let Some(header) = self.0.header.as_ref() {
            BigInt::from(header.gas_limit)
        } else {
            BigInt::zero()
        }
    }

    pub fn gas_price(&self) -> BigInt {
        if let Some(header) = self.0.header.as_ref() {
            if let Some(base_fee_per_gas) = header.base_fee_per_gas.as_ref() {
                return base_fee_per_gas.deserialize() * self.gas_used();
            }
        }

        BigInt::zero()
    }

    pub fn burnt_fees(&self) -> BigInt {
        self.0
            .transaction_traces
            .iter()
            .map(|transaction_trace| {
                if let Some(gas_price) = transaction_trace.gas_price.as_ref() {
                    gas_price.deserialize() * BigInt::from(transaction_trace.gas_used)
                } else {
                    BigInt::zero()
                }
            })
            .fold(BigInt::zero(), |sum, val| sum + val)
    }

    /// Returns the sum of all positive balance changes with reward reasons (check is_a_reward() fn to see which reasons qualify)
    pub fn rewards(&self) -> BigInt {
        self.0
            .balance_changes
            .iter()
            .map(|balance_change| {
                if is_a_reward(balance_change) {
                    // We should only be accounting for rewards from one side of the transfer so we will not count negative
                    // balance changes with the idea that there will be a counterpart balance change equivalent to this for
                    // the gain in balance for the rewardee, and we will be counting that one instead
                    get_balance_gain(balance_change)
                } else {
                    BigInt::zero()
                }
            })
            .fold(BigInt::zero(), |sum, val| sum + val)
    }
}

fn get_balance_gain(balance_change: &BalanceChange) -> BigInt {
    match (balance_change.old_value.as_ref(), balance_change.new_value.as_ref()) {
        (Some(old_value_raw), Some(new_value_raw)) => {
            let old_value = old_value_raw.deserialize();
            let new_value = new_value_raw.deserialize();
            if new_value > old_value {
                new_value - old_value
            } else {
                // If old_value > new_value money is flowing out this account to the rewardee.
                // TODO: log here to see what's going on..
                BigInt::zero()
            }
        }
        (Some(old_value), None) => old_value.deserialize(), // Maybe we should panic if this happens also..
        (None, Some(new_value)) => new_value.deserialize(),
        (None, None) => BigInt::zero(),
    }
}

fn is_a_reward(balance_change: &BalanceChange) -> bool {
    const REWARD_REASONS: [i32; 4] = [
        Reason::RewardFeeReset as i32,
        Reason::RewardMineBlock as i32,
        Reason::RewardMineUncle as i32,
        Reason::RewardTransactionFee as i32
    ];

    REWARD_REASONS.contains(&balance_change.reason)
}

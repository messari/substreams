use std::fmt::{Display, Formatter};

pub(crate) enum StoreKey {
    // Pre-aggregation keys
    Transactions,
    Supply,
    Blocks,
    UniqueAuthors,
    // Aggregation keys
    CumulativeUniqueAuthors,
    BlockHeight,
    CumulativeDifficulty,
    CumulativeGasUsed,
    CumulativeBurntFees,
    CumulativeRewards,
    CumulativeTransactions,
    CumulativeSize,
    TotalSupply,
    DailyBlocks,
    BlocksAcrossDay,
    BlocksAcrossHour,
    Difficulty,
    GasUsed,
    GasLimit,
    BurntFees,
    Rewards,
    BlockSize,
    BlockInterval,
    NumDays,
    DailyUniqueAuthors,
    DailySupply,
    DailyTransactions,
    NumHours,
    HourlyUniqueAuthors,
    HourlySupply,
    HourlyTransactions,
}

impl StoreKey {
    pub(crate) fn get_total_sum_key(&self) -> String {
        format!("1{}", self.get_unique_id())
    }

    pub(crate) fn get_total_sum_squares_key(&self) -> String {
        format!("2{}", self.get_unique_id())
    }

    pub(crate) fn get_day_sum_key(&self, day_timestamp: &String) -> String {
        format!("3{}{}", self.get_unique_id(), day_timestamp)
    }

    pub(crate) fn get_day_sum_squares_key(&self, day_timestamp: &String) -> String {
        format!("4{}{}", self.get_unique_id(), day_timestamp)
    }

    pub(crate) fn get_hour_sum_key(&self, hour_timestamp: &String) -> String {
        format!("5{}{}", self.get_unique_id(), hour_timestamp)
    }

    pub(crate) fn get_hour_sum_squares_key(&self, hour_timestamp: &String) -> String {
        format!("6{}{}", self.get_unique_id(), hour_timestamp)
    }

    pub(crate) fn get_unique_id(&self) -> String {
        // This is done "manually" in the match block instead of doing a '#[repr(u8)]' and casting to
        // string. It's done this way so that if there is an error with writing to the store it should
        // still be easy to understand what the key represents making it easier to debug the error.
        match self {
            StoreKey::Transactions => "A".to_string(),
            StoreKey::Supply => "B".to_string(),
            StoreKey::Blocks => "C".to_string(),
            StoreKey::UniqueAuthors => "D".to_string(),
            StoreKey::CumulativeUniqueAuthors => "E".to_string(),
            StoreKey::BlockHeight => "F".to_string(),
            StoreKey::CumulativeDifficulty => "G".to_string(),
            StoreKey::CumulativeGasUsed => "H".to_string(),
            StoreKey::CumulativeBurntFees => "I".to_string(),
            StoreKey::CumulativeRewards => "J".to_string(),
            StoreKey::CumulativeTransactions => "K".to_string(),
            StoreKey::CumulativeSize => "L".to_string(),
            StoreKey::TotalSupply => "M".to_string(),
            StoreKey::DailyBlocks => "N".to_string(),
            StoreKey::BlocksAcrossDay => "O".to_string(),
            StoreKey::BlocksAcrossHour => "P".to_string(),
            StoreKey::Difficulty => "Q".to_string(),
            StoreKey::GasUsed => "R".to_string(),
            StoreKey::GasLimit => "S".to_string(),
            StoreKey::BurntFees => "T".to_string(),
            StoreKey::Rewards => "U".to_string(),
            StoreKey::BlockSize => "V".to_string(),
            StoreKey::BlockInterval => "W".to_string(),
            StoreKey::NumDays => "X".to_string(),
            StoreKey::DailyUniqueAuthors => "Y".to_string(),
            StoreKey::DailySupply => "Z".to_string(),
            StoreKey::DailyTransactions => "a".to_string(),
            StoreKey::NumHours => "b".to_string(),
            StoreKey::HourlyUniqueAuthors => "c".to_string(),
            StoreKey::HourlySupply => "d".to_string(),
            StoreKey::HourlyTransactions => "e".to_string(),
        }
    }
}
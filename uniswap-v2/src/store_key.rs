#[derive(Clone)]
pub(crate) enum StoreKey {
    Pool,
    User,
    UserBalance,
    PoolTokenSupply,
    TokenWhitelist,
    InputTokenBalance,
    TokenPrice,
    PoolVolume,
    PoolTVL,

    // Usage Metrics keys
    PoolCount,
    ActiveUser,
    ActiveUserCount,
    TransactionCount,
    DepositCount,
    WithdrawCount,
    SwapCount,
}

impl StoreKey {
    pub(crate) fn get_unique_pool_key(&self, pool_address: &String) -> String {
        format!("1{}:{}", self.get_unique_id(), pool_address)
    }

    pub(crate) fn get_unique_token_key(&self, token_address: &String) -> String {
        format!("2{}:{}", self.get_unique_id(), token_address)
    }

    pub(crate) fn get_cumulative_field_key(&self, unique_key: &String) -> String {
        format!("c:{}:{}", self.get_unique_id(), unique_key)
    }

    pub(crate) fn get_cumulative_stats_key(&self) -> String {
        format!("c:{}", self.get_unique_id())
    }

    pub(crate) fn get_daily_stats_key(&self, day_timestamp: &String) -> String {
        format!("d:{}:{}", self.get_unique_id(), day_timestamp)
    }

    pub(crate) fn get_hourly_stats_key(&self, hour_timestamp: &String) -> String {
        format!("h:{}:{}", self.get_unique_id(), hour_timestamp)
    }

    pub(crate) fn get_daily_user_key(&self, user: &String, day_timestamp: &String) -> String {
        format!("d:{}:{}:{}", self.get_unique_id(), user, day_timestamp)
    }

    pub(crate) fn get_hourly_user_key(&self, user: &String, hour_timestamp: &String) -> String {
        format!("h:{}:{}:{}", self.get_unique_id(), user, hour_timestamp)
    }

    pub(crate) fn get_pool_token_balance_key(
        &self,
        pool_address: &String,
        token_address: &String,
    ) -> String {
        format!(
            "{}:{}:{}",
            self.get_unique_id(),
            pool_address,
            token_address
        )
    }

    pub(crate) fn get_pool_and_token_from_key(&self, key: &String) -> Option<(String, String)> {
        let chunks: Vec<&str> = key.split(":").collect();

        if chunks[0] != self.get_unique_id() {
            return None;
        }

        return Some((chunks[1].to_string(), chunks[2].to_string()));
    }

    pub(crate) fn get_user_balance_key(
        &self,
        pool_address: &String,
        user_address: &String,
    ) -> String {
        format!(
            "{}:{}::{}",
            self.get_unique_id(),
            pool_address,
            user_address
        )
    }

    pub(crate) fn get_unique_id(&self) -> String {
        match self {
            StoreKey::Pool => "Pool".to_string(),
            StoreKey::PoolCount => "PoolCount".to_string(),
            StoreKey::UserBalance => "Balance".to_string(),
            StoreKey::User => "User".to_string(),
            StoreKey::ActiveUser => "ActiveUser".to_string(),
            StoreKey::ActiveUserCount => "ActiveUserCount".to_string(),
            StoreKey::PoolTokenSupply => "PoolSupply".to_string(),
            StoreKey::DepositCount => "DepositCount".to_string(),
            StoreKey::WithdrawCount => "WithdrawCount".to_string(),
            StoreKey::SwapCount => "SwapCount".to_string(),
            StoreKey::TransactionCount => "TransactionCount".to_string(),
            StoreKey::TokenWhitelist => "TokenWhitelist".to_string(),
            StoreKey::InputTokenBalance => "InputTokenBalance".to_string(),
            StoreKey::TokenPrice => "TokenPrice".to_string(),
            StoreKey::PoolVolume => "PoolVolume".to_string(),
            StoreKey::PoolTVL => "PoolLevelTVL".to_string(),
        }
    }
}

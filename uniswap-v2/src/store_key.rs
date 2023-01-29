#[derive(Clone)]
pub(crate) enum StoreKey {
    Pool,
    User,
    UserBalance,
    TokenWhitelist,
    InputTokenBalance,
    TokenPrice,

    // Usage Metrics keys
    PoolCount,
    ActiveUser,
    ActiveUserCount,
    TransactionCount,
    DepositCount,
    WithdrawCount,
    SwapCount,

    // Liquidity Pool Keys
    PoolTVL,
    PoolVolume,
    PoolOutputTokenSupply,
    PoolSupplySideRevenue,
    PoolProtocolSideRevenue,
    PoolTotalRevenue,
}

impl StoreKey {
    pub(crate) fn get_unique_pool_key(&self, pool_address: &String) -> String {
        format!("1{}:{}", self.get_unique_id(), pool_address)
    }

    pub(crate) fn get_unique_token_key(&self, token_address: &String) -> String {
        format!("2{}:{}", self.get_unique_id(), token_address)
    }

    pub(crate) fn get_pool_specific_hourly_key(&self, timestamp: &i64, pool: &String) -> String {
        format!("h:{}:{}:{}", self.get_unique_id(), timestamp, pool)
    }

    pub(crate) fn get_pool_specific_daily_key(&self, timestamp: &i64, pool: &String) -> String {
        format!("d:{}:{}:{}", self.get_unique_id(), timestamp, pool)
    }

    pub(crate) fn get_pool_specific_cumulative_key(&self, pool: &String) -> String {
        format!("c:{}:{}", self.get_unique_id(), pool)
    }

    pub(crate) fn get_protocol_specific_hourly_key(&self, timestamp: &i64) -> String {
        format!("h:{}:{}", self.get_unique_id(), timestamp)
    }

    pub(crate) fn get_protocol_specific_daily_key(&self, timestamp: &i64) -> String {
        format!("d:{}:{}", self.get_unique_id(), timestamp)
    }

    pub(crate) fn get_protocol_specific_cumulative_key(&self) -> String {
        format!("c:{}", self.get_unique_id())
    }

    pub(crate) fn get_user_balance_key(&self, pool: &String, user: &String) -> String {
        format!("{}:{}::{}", self.get_unique_id(), pool, user)
    }

    pub(crate) fn get_pool_token_balance_key(&self, pool: &String, token: &String) -> String {
        format!("{}:{}:{}", self.get_unique_id(), pool, token)
    }

    pub(crate) fn get_pool_from_key(&self, key: &String) -> String {
        let chunks: Vec<&str> = key.split(":").collect();

        return chunks[1].to_string();
    }

    pub(crate) fn get_pool_and_token_from_key(&self, key: &String) -> Option<(String, String)> {
        let chunks: Vec<&str> = key.split(":").collect();

        if chunks[0] != self.get_unique_id() {
            return None;
        }

        return Some((chunks[1].to_string(), chunks[2].to_string()));
    }

    pub(crate) fn get_unique_id(&self) -> String {
        match self {
            StoreKey::Pool => "Pool".to_string(),
            StoreKey::PoolCount => "PoolCount".to_string(),
            StoreKey::UserBalance => "Balance".to_string(),
            StoreKey::User => "User".to_string(),
            StoreKey::ActiveUser => "ActiveUser".to_string(),
            StoreKey::ActiveUserCount => "ActiveUserCount".to_string(),
            StoreKey::PoolOutputTokenSupply => "PoolOutputTokenSupply".to_string(),
            StoreKey::DepositCount => "DepositCount".to_string(),
            StoreKey::WithdrawCount => "WithdrawCount".to_string(),
            StoreKey::SwapCount => "SwapCount".to_string(),
            StoreKey::TransactionCount => "TransactionCount".to_string(),
            StoreKey::TokenWhitelist => "TokenWhitelist".to_string(),
            StoreKey::InputTokenBalance => "InputTokenBalance".to_string(),
            StoreKey::TokenPrice => "TokenPrice".to_string(),
            StoreKey::PoolVolume => "PoolVolume".to_string(),
            StoreKey::PoolTVL => "PoolTVL".to_string(),
            StoreKey::PoolSupplySideRevenue => "PoolSupplySideRevenue".to_string(),
            StoreKey::PoolProtocolSideRevenue => "PoolProtocolSideRevenue".to_string(),
            StoreKey::PoolTotalRevenue => "PoolTotalRevenue".to_string(),
        }
    }
}

#[derive(Clone)]
pub(crate) enum StoreKey {
    Pool,
    PoolCount,
    UserBalance,
    PoolTokenSupply,
    DepositCount,
    WithdrawCount,
    SwapCount,
    TransactionCount,
    TokenWhitelist,
    InputTokenBalance,
    TokenPrice,
    PoolVolume
}

impl StoreKey {
    pub(crate) fn get_unique_pool_key(&self, pool_address: &String) -> String {
        format!("{}:{}", self.get_unique_id(), pool_address)
    }

    pub(crate) fn get_unique_token_key(&self, token_address: &String) -> String {
        format!("{}:{}", self.get_unique_id(), token_address)
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
            StoreKey::PoolTokenSupply => "PoolSupply".to_string(),
            StoreKey::DepositCount => "DepositCount".to_string(),
            StoreKey::WithdrawCount => "WithdrawCount".to_string(),
            StoreKey::SwapCount => "SwapCount".to_string(),
            StoreKey::TransactionCount => "TransactionCount".to_string(),
            StoreKey::TokenWhitelist => "TokenWhitelist".to_string(),
            StoreKey::InputTokenBalance => "InputTokenBalance".to_string(),
            StoreKey::TokenPrice => "TokenPrice".to_string(),
            StoreKey::PoolVolume => "PoolVolume".to_string(),
        }
    }
}

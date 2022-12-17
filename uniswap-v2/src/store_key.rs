#[derive(Clone)]
pub(crate) enum StoreKey {
    Pool,
    UserBalance,
    DepositEvent,
    WithdrawEvent,
    SwapEvent,
}

impl StoreKey {
    pub(crate) fn get_unique_pool_key(&self, pool_address: &String) -> String {
        format!("{}:{}", self.get_unique_id(), pool_address)
    }

    pub(crate) fn get_unique_event_key(&self, pool_address: &String, txn_hash: &String) -> String {
        format!("{}:{}::{}", self.get_unique_id(), pool_address, txn_hash)
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
            StoreKey::UserBalance => "Balance".to_string(),
            StoreKey::DepositEvent => "Deposit".to_string(),
            StoreKey::WithdrawEvent => "Withdraw".to_string(),
            StoreKey::SwapEvent => "Swap".to_string(),
        }
    }
}

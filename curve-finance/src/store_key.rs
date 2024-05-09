#[derive(Clone)]
pub enum StoreKey {
    Pool,
    AdminBalanceToken0,
    AdminBalanceToken1,
    AdminBalanceToken2,
    AdminBalanceToken3,
}

impl StoreKey {
    pub fn get_unique_key(&self, key: &str) -> String {
        format!("{}:{}", self.unique_id(), key)
    }

    pub fn unique_id(&self) -> String {
        match self {
            StoreKey::Pool => "Pool".to_string(),
            StoreKey::AdminBalanceToken0 => "ABT0".to_string(),
            StoreKey::AdminBalanceToken1 => "ABT1".to_string(),
            StoreKey::AdminBalanceToken2 => "ABT2".to_string(),
            StoreKey::AdminBalanceToken3 => "ABT3".to_string(),
        }
    }
}

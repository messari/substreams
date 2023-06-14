#[derive(Clone)]
pub enum StoreKey {
    Pool,
    TotalBalance,
    LatestTimestamp,
    LatestBlockNumber,
    TokenBalance,
    Token0Balance,
    Token1Balance,
    OutputTokenBalance,
    TokenPrice,
    TotalValueLockedUSD,
    Volume,
    DailySupplySideRevenueUSD,
    HourlySupplySideRevenueUSD,
    CumulativeSupplySideRevenueUSD,
    DailyProtocolSideRevenueUSD,
    HourlyProtocolSideRevenueUSD,
    CumulativeProtocolSideRevenueUSD,
    DailyTotalRevenueUSD,
    HourlyTotalRevenueUSD,
    CumulativeTotalRevenueUSD,
    DailyVolumeUSD,
    HourlyVolumeUSD,
    CumulativeVolumeUSD,
    VolumeByTokenUSD,
    DailyVolumeByTokenUSD,
    HourlyVolumeByTokenUSD,
    DailyVolumeByTokenAmount,
    HourlyVolumeByTokenAmount,
}

impl StoreKey {
    pub fn get_unique_pool_key(&self, key: &str) -> String {
        format!("{}:{}", self.unique_id(), key)
    }

    pub fn get_unique_pair_key(&self, key1: &str, key2: &str) -> String {
        format!("{}:{}:{}", self.unique_id(), key1, key2)
    }

    pub fn get_unique_protocol_key(&self) -> String {
        format!("[Protocol]:{}", self.unique_id())
    }

    pub fn get_unique_snapshot_key(&self, id: i64, keys: Vec<&str>) -> String {
        format!("{}:{}:{}", self.unique_id(), id, keys.join(":"))
    }

    pub fn get_unique_daily_protocol_key(&self, day_id: i64) -> String {
        format!("[Protocol]:{}:{}", self.unique_id(), day_id)
    }

    pub fn get_unique_snapshot_tracking_key(&self, key1: &str, key2: &str) -> String {
        format!("{}:{}:{}", self.unique_id(), key1, key2)
    }

    pub fn get_pool(&self, key: &str) -> Option<String> {
        let chunks: Vec<&str> = key.split(":").collect();

        if chunks[0] != self.unique_id() {
            return None;
        }
        return Some(chunks[1].to_string());
    }

    pub fn get_pool_and_token(&self, key: &str) -> Option<(String, String)> {
        let chunks: Vec<&str> = key.split(":").collect();

        if chunks[0] != self.unique_id() {
            return None;
        }
        return Some((chunks[1].to_string(), chunks[2].to_string()));
    }

    pub fn unique_id(&self) -> String {
        match self {
            StoreKey::Pool => "Pool".to_string(),
            StoreKey::TotalBalance => "TotalBalance".to_string(),
            StoreKey::LatestTimestamp => "LatestTimestamp".to_string(),
            StoreKey::LatestBlockNumber => "LatestBlockNumber".to_string(),
            StoreKey::TokenBalance => "TokenBalance".to_string(),
            StoreKey::Token0Balance => "Token0Balance".to_string(),
            StoreKey::Token1Balance => "Token1Balance".to_string(),
            StoreKey::OutputTokenBalance => "OutputTokenBalance".to_string(),
            StoreKey::TotalValueLockedUSD => "TotalValueLockedUSD".to_string(),
            StoreKey::TokenPrice => "TokenPrice".to_string(),
            StoreKey::Volume => "Volume".to_string(),
            StoreKey::DailySupplySideRevenueUSD => "d:SupplySideRevenueUSD".to_string(),
            StoreKey::HourlySupplySideRevenueUSD => "h:SupplySideRevenueUSD".to_string(),
            StoreKey::CumulativeSupplySideRevenueUSD => "c:SupplySideRevenueUSD".to_string(),
            StoreKey::DailyProtocolSideRevenueUSD => "d:ProtocolSideRevenueUSD".to_string(),
            StoreKey::HourlyProtocolSideRevenueUSD => "h:ProtocolSideRevenueUSD".to_string(),
            StoreKey::CumulativeProtocolSideRevenueUSD => "c:ProtocolSideRevenueUSD".to_string(),
            StoreKey::DailyTotalRevenueUSD => "d:TotalRevenueUSD".to_string(),
            StoreKey::HourlyTotalRevenueUSD => "h:TotalRevenueUSD".to_string(),
            StoreKey::CumulativeTotalRevenueUSD => "c:TotalRevenueUSD".to_string(),
            StoreKey::DailyVolumeUSD => "d:VolumeUSD".to_string(),
            StoreKey::HourlyVolumeUSD => "h:VolumeUSD".to_string(),
            StoreKey::CumulativeVolumeUSD => "c:VolumeUSD".to_string(),
            StoreKey::VolumeByTokenUSD => "VolumeByTokenUSD".to_string(),
            StoreKey::DailyVolumeByTokenUSD => "d:VolumeByTokenUSD".to_string(),
            StoreKey::HourlyVolumeByTokenUSD => "h:VolumeByTokenUSD".to_string(),
            StoreKey::DailyVolumeByTokenAmount => "d:VolumeByTokenAmount".to_string(),
            StoreKey::HourlyVolumeByTokenAmount => "h:VolumeByTokenAmount".to_string(),
        }
    }
}

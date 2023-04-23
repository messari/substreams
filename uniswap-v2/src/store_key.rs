use substreams::scalar::BigInt;

#[derive(Clone)]
pub(crate) enum StoreKey {
    Pool,
    TotalBalance,
    LatestTimestamp,
    LatestBlockNumber,
    Token0Balance,
    Token1Balance,
    OutputTokenBalance,
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
}

impl StoreKey {
    pub fn get_unique_pool_key(&self, key: &String) -> String {
        format!("{}:{}", self.unique_id(), key)
    }

    pub fn get_unique_protocol_key(&self) -> String {
        format!("[Protocol]:{}", self.unique_id())
    }

    pub fn get_unique_daily_pool_key(&self, day_id: BigInt, key: &String) -> String {
        format!("{}:{}:{}", self.unique_id(), day_id.to_string(), key)
    }

    pub fn get_unique_hourly_pool_key(&self, hour_id: BigInt, key: &String) -> String {
        format!("{}:{}:{}", self.unique_id(), hour_id.to_string(), key)
    }

    pub fn get_unique_daily_protocol_key(&self, day_id: BigInt) -> String {
        format!("[Protocol]:{}:{}", self.unique_id(), day_id.to_string())
    }

    pub fn get_unique_snapshot_tracking_key(&self, key1: &String, key2: &String) -> String {
        format!("{}:{}:{}", self.unique_id(), key1, key2)
    }

    pub fn get_pool(&self, key: &String) -> Option<String> {
        let chunks: Vec<&str> = key.split(":").collect();

        if chunks[0] != self.unique_id() {
            return None;
        }
        return Some(chunks[1].to_string());
    }

    pub fn unique_id(&self) -> String {
        match self {
            StoreKey::Pool => "Pool".to_string(),
            StoreKey::TotalBalance => "TotalBalance".to_string(),
            StoreKey::LatestTimestamp => "LatestTimestamp".to_string(),
            StoreKey::LatestBlockNumber => "LatestBlockNumber".to_string(),
            StoreKey::Token0Balance => "Token0Balance".to_string(),
            StoreKey::Token1Balance => "Token1Balance".to_string(),
            StoreKey::OutputTokenBalance => "OutputTokenBalance".to_string(),
            StoreKey::TotalValueLockedUSD => "TotalValueLockedUSD".to_string(),
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
        }
    }
}

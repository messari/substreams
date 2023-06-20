use crate::pb::synthetix::v1::EscrowContractVersion;

pub const ESCROW_REWARDS_CONTRACT_V1: &str = "0xb671f2210b1f6621a2607ea63e6b2dc3e2464d1f";
pub const ESCROW_REWARDS_CONTRACT_V2: &str = "0x182738bd9ee9810bc11f1c81b07ec6f3691110bb";
pub const ESCROW_REWARDS_CONTRACT_V2_FALLBACK: &str = "0xda4ef8520b1a57d7d63f1e249606d1a459698876";
pub const SNX_TOKEN_STATE_CONTRACT: &str = "0x5b1b5fea1b99d83ad479df0c222f0492385381dd";
pub const SDS_TOKEN_CONTRACT: &str = "0x89fcb32f29e509cc42d0c8b6f058c993013a843f";
pub const LIQUIDATOR_REWARDS_CONTRACT: &str = "0xf79603a71144e415730c1a6f57f366e4ea962c00";

pub struct EscrowContractStorageData {
    pub version: EscrowContractVersion,
    pub address: &'static str,
    pub escrowed_balance_slot: usize,
    pub vested_balance_slot: usize,
}

impl EscrowContractStorageData {
    pub const V1: Self = Self {
        version: EscrowContractVersion::V1,
        address: ESCROW_REWARDS_CONTRACT_V1,
        escrowed_balance_slot: 5,
        vested_balance_slot: 6,
    };
    pub const V2: Self = Self {
        version: EscrowContractVersion::V2,
        address: ESCROW_REWARDS_CONTRACT_V2,
        escrowed_balance_slot: 6,
        vested_balance_slot: 7,
    };
    pub const V2_FALLBACK: Self = Self {
        version: EscrowContractVersion::V2Fallback,
        address: ESCROW_REWARDS_CONTRACT_V2_FALLBACK,
        escrowed_balance_slot: 8,
        vested_balance_slot: 9,
    };
}

pub const SNX_TOKEN_STATE_BALANCE_SLOT: usize = 3;
pub const SDS_CONTRACT_BALANCE_SLOT: usize = 6;
pub const LIQUIDATOR_REWARDS_ACCUMULATED_REWARDS_PER_SHARE_SLOT: usize = 5;
pub const LIQUIDATOR_REWARDS_CONTRACT_ENTRIES_SLOT: usize = 6;

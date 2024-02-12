// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub factory_pair_createds: ::prost::alloc::vec::Vec<FactoryPairCreated>,
    #[prost(message, repeated, tag="2")]
    pub pools_approvals: ::prost::alloc::vec::Vec<PoolsApproval>,
    #[prost(message, repeated, tag="3")]
    pub pools_burns: ::prost::alloc::vec::Vec<PoolsBurn>,
    #[prost(message, repeated, tag="4")]
    pub pools_mints: ::prost::alloc::vec::Vec<PoolsMint>,
    #[prost(message, repeated, tag="5")]
    pub pools_swaps: ::prost::alloc::vec::Vec<PoolsSwap>,
    #[prost(message, repeated, tag="6")]
    pub pools_syncs: ::prost::alloc::vec::Vec<PoolsSync>,
    #[prost(message, repeated, tag="7")]
    pub pools_transfers: ::prost::alloc::vec::Vec<PoolsTransfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FactoryPairCreated {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub token0: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub token1: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub pair: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub param3: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsApproval {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub spender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsBurn {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub to: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsMint {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsSwap {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount0_in: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount1_in: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub amount0_out: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub amount1_out: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="11")]
    pub to: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsSync {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub reserve0: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub reserve1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsTransfer {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub value: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)

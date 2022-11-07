// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<TransferEvent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvent {
    #[prost(string, tag="1")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    #[prost(uint64, tag="3")]
    pub log_ordinal: u64,
    #[prost(string, tag="4")]
    pub token_address: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub to: ::prost::alloc::string::String,
    /// BigInt, in token's native amount
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthBalance {
    #[prost(string, tag="1")]
    pub token_address: ::prost::alloc::string::String,
    /// BigInt, in token's native amount
    #[prost(string, tag="2")]
    pub balance: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wallet {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub balances: ::prost::alloc::vec::Vec<EthBalance>,
}
// @@protoc_insertion_point(module)

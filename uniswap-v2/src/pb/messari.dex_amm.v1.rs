// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// bytes: Address
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub input_tokens: ::prost::alloc::vec::Vec<super::super::erc20::v1::Erc20Token>,
    /// Metrics
    ///
    /// string: BigDecimal
    #[prost(string, tag="100")]
    pub total_value_locked: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolToken {
    /// bytes: Address
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// Balance of input token in native amounts
    ///
    /// string: BigInt
    #[prost(string, tag="2")]
    pub balance: ::prost::alloc::string::String,
    /// Weights of input token in the liquidity pool in percentage values. For example, 0.5/0.5 for Uniswap pools, 0.482/0.518 for a Curve pool, 0.1/0.1/0.8 for a Balancer pool
    #[prost(double, tag="3")]
    pub weight: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pools {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Pool>,
}
/// Encoded file descriptor set for the `messari.dex_amm.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x80, 0x0a, 0x0a, 0x0d, 0x64, 0x65, 0x78, 0x5f, 0x61, 0x6d, 0x6d, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x12, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x72, 0x69, 0x2e, 0x64, 0x65, 0x78, 0x5f,
    0x61, 0x6d, 0x6d, 0x2e, 0x76, 0x31, 0x1a, 0x0c, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0b, 0x65, 0x72, 0x63, 0x32, 0x30, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0xa3, 0x01, 0x0a, 0x04, 0x50, 0x6f, 0x6f, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x18,
    0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x3f, 0x0a, 0x0c, 0x69, 0x6e, 0x70, 0x75,
    0x74, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c,
    0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x72, 0x69, 0x2e, 0x65, 0x72, 0x63, 0x32, 0x30, 0x2e, 0x76,
    0x31, 0x2e, 0x45, 0x52, 0x43, 0x32, 0x30, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x52, 0x0b, 0x69, 0x6e,
    0x70, 0x75, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x12, 0x2c, 0x0a, 0x12, 0x74, 0x6f, 0x74,
    0x61, 0x6c, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x5f, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x18,
    0x64, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x4c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x22, 0x57, 0x0a, 0x09, 0x50, 0x6f, 0x6f, 0x6c, 0x54,
    0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x18,
    0x0a, 0x07, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x07, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x77, 0x65, 0x69, 0x67,
    0x68, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x06, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74,
    0x22, 0x37, 0x0a, 0x05, 0x50, 0x6f, 0x6f, 0x6c, 0x73, 0x12, 0x2e, 0x0a, 0x05, 0x69, 0x74, 0x65,
    0x6d, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x72, 0x69, 0x2e, 0x64, 0x65, 0x78, 0x5f, 0x61, 0x6d, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x6f,
    0x6f, 0x6c, 0x52, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x4a, 0xff, 0x06, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x1a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02, 0x00, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x03, 0x00, 0x15, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x05, 0x00, 0x1b, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x07, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x08, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x09, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x10, 0x11, 0x0a, 0x1d, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x02, 0x15, 0x22, 0x10, 0x20, 0x62, 0x79, 0x74,
    0x65, 0x73, 0x3a, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x09, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x0a, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0a, 0x0b, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x27, 0x33, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x36, 0x37, 0x0a, 0x2c, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x02, 0x22, 0x1a, 0x09, 0x20, 0x4d, 0x65, 0x74, 0x72, 0x69,
    0x63, 0x73, 0x0a, 0x22, 0x14, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3a, 0x20, 0x42, 0x69,
    0x67, 0x44, 0x65, 0x63, 0x69, 0x6d, 0x61, 0x6c, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x0d, 0x09, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x0d, 0x1e, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x10, 0x00, 0x16, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x11, 0x0a, 0x1d, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x15, 0x22, 0x10, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73,
    0x3a, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x11, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x11, 0x13, 0x14, 0x0a, 0x49, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x13, 0x02,
    0x15, 0x1a, 0x2a, 0x20, 0x42, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x69,
    0x6e, 0x70, 0x75, 0x74, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x69, 0x6e, 0x20, 0x6e, 0x61,
    0x74, 0x69, 0x76, 0x65, 0x20, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x73, 0x0a, 0x22, 0x10, 0x20,
    0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3a, 0x20, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x13, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x13, 0x13, 0x14, 0x0a, 0xb8, 0x01, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x15, 0x02, 0x14, 0x1a, 0xaa, 0x01, 0x20, 0x57, 0x65, 0x69, 0x67, 0x68,
    0x74, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x74, 0x6f, 0x6b, 0x65,
    0x6e, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x71, 0x75, 0x69, 0x64, 0x69,
    0x74, 0x79, 0x20, 0x70, 0x6f, 0x6f, 0x6c, 0x20, 0x69, 0x6e, 0x20, 0x70, 0x65, 0x72, 0x63, 0x65,
    0x6e, 0x74, 0x61, 0x67, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x2e, 0x20, 0x46, 0x6f,
    0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2c, 0x20, 0x30, 0x2e, 0x35, 0x2f, 0x30,
    0x2e, 0x35, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x55, 0x6e, 0x69, 0x73, 0x77, 0x61, 0x70, 0x20, 0x70,
    0x6f, 0x6f, 0x6c, 0x73, 0x2c, 0x20, 0x30, 0x2e, 0x34, 0x38, 0x32, 0x2f, 0x30, 0x2e, 0x35, 0x31,
    0x38, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x43, 0x75, 0x72, 0x76, 0x65, 0x20, 0x70, 0x6f,
    0x6f, 0x6c, 0x2c, 0x20, 0x30, 0x2e, 0x31, 0x2f, 0x30, 0x2e, 0x31, 0x2f, 0x30, 0x2e, 0x38, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x42, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x72, 0x20, 0x70,
    0x6f, 0x6f, 0x6c, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x15,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x09, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x15, 0x12, 0x13, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x18, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x18, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x19, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x19, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x19, 0x0b, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x10, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x18, 0x19, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
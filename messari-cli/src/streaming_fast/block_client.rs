use std::env;
use prost::Message;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;

use crate::streaming_fast::block_client::fetch_client::FetchClient;
use crate::streaming_fast::block_client::single_block_request::BlockNumber;
use crate::streaming_fast::eth;
use crate::streaming_fast::streaming_config::Chain;

pub(crate) async fn get_latest_block_number(chain: &Chain) -> i64 {
    let streamingfast_token = env::var("SUBSTREAMS_API_TOKEN").unwrap();
    let token_metadata = MetadataValue::try_from(streamingfast_token.as_str()).unwrap();

    let mut client = FetchClient::with_interceptor(
        Channel::builder(chain.get_endpoint()).connect_lazy(),
        move |mut r: tonic::Request<()>| {
            r.metadata_mut().insert("authorization", token_metadata.clone());
            Ok(r)
        },
    );

    let req = SingleBlockRequest {
        transforms: [].to_vec(),
        reference: Some(single_block_request::Reference::BlockNumber(BlockNumber{num: u64::MAX})),
    };

    let response = client.block(req).await.unwrap();
    let response_block = eth::Block::decode(response.get_ref().block.as_ref().unwrap().value.as_ref()).unwrap();

    response_block.number as i64
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleBlockRequest {
    #[prost(message, repeated, tag = "6")]
    pub transforms: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(oneof = "single_block_request::Reference", tags = "3, 4, 5")]
    pub reference: ::core::option::Option<single_block_request::Reference>,
}
/// Nested message and enum types in `SingleBlockRequest`.
pub mod single_block_request {
    /// Get the current known canonical version of a block at with this number
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockNumber {
        #[prost(uint64, tag = "1")]
        pub num: u64,
    }
    /// Get the current block with specific hash and number
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockHashAndNumber {
        #[prost(uint64, tag = "1")]
        pub num: u64,
        #[prost(string, tag = "2")]
        pub hash: ::prost::alloc::string::String,
    }
    /// Get the block that generated a specific cursor
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Cursor {
        #[prost(string, tag = "1")]
        pub cursor: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Reference {
        #[prost(message, tag = "3")]
        BlockNumber(BlockNumber),
        #[prost(message, tag = "4")]
        BlockHashAndNumber(BlockHashAndNumber),
        #[prost(message, tag = "5")]
        Cursor(Cursor),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleBlockResponse {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    /// Controls where the stream of blocks will start.
    ///
    /// The stream will start **inclusively** at the requested block num.
    ///
    /// When not provided, starts at first streamable block of the chain. Not all
    /// chain starts at the same block number, so you might get an higher block than
    /// requested when using default value of 0.
    ///
    /// Can be negative, will be resolved relative to the chain head block, assuming
    /// a chain at head block #100, then using `-50` as the value will start at block
    /// #50. If it resolves before first streamable block of chain, we assume start
    /// of chain.
    ///
    /// If `start_cursor` is given, this value is ignored and the stream instead starts
    /// immediately after the Block pointed by the opaque `start_cursor` value.
    #[prost(int64, tag = "1")]
    pub start_block_num: i64,
    /// Controls where the stream of blocks will start which will be immediately after
    /// the Block pointed by this opaque cursor.
    ///
    /// Obtain this value from a previously received `Response.cursor`.
    ///
    /// This value takes precedence over `start_block_num`.
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
    /// When non-zero, controls where the stream of blocks will stop.
    ///
    /// The stream will close **after** that block has passed so the boundary is
    /// **inclusive**.
    #[prost(uint64, tag = "3")]
    pub stop_block_num: u64,
    /// With final_block_only, you only receive blocks with STEP_FINAL
    /// Default behavior will send blocks as STEP_NEW, with occasional STEP_UNDO
    #[prost(bool, tag = "4")]
    pub final_blocks_only: bool,
    #[prost(message, repeated, tag = "10")]
    pub transforms: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// Chain specific block payload, ex:
    ///    - sf.eosio.type.v1.Block
    ///    - sf.ethereum.type.v1.Block
    ///    - sf.near.type.v1.Block
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<::prost_types::Any>,
    #[prost(enumeration = "ForkStep", tag = "6")]
    pub step: i32,
    #[prost(string, tag = "10")]
    pub cursor: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ForkStep {
    StepUnset = 0,
    /// Incoming block
    StepNew = 1,
    /// A reorg caused this specific block to be excluded from the chain
    StepUndo = 2,
    /// Block is now final and can be committed (finality is chain specific,
    /// see chain documentation for more details)
    StepFinal = 3,
}
impl ForkStep {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ForkStep::StepUnset => "STEP_UNSET",
            ForkStep::StepNew => "STEP_NEW",
            ForkStep::StepUndo => "STEP_UNDO",
            ForkStep::StepFinal => "STEP_FINAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STEP_UNSET" => Some(Self::StepUnset),
            "STEP_NEW" => Some(Self::StepNew),
            "STEP_UNDO" => Some(Self::StepUndo),
            "STEP_FINAL" => Some(Self::StepFinal),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod fetch_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct FetchClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FetchClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: std::convert::TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> FetchClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::BoxBody>,
            T::Error: Into<StdError>,
            T::ResponseBody: Body<Data = Bytes> + Send + 'static,
            <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> FetchClient<InterceptedService<T, F>>
            where
                F: tonic::service::Interceptor,
                T::ResponseBody: Default,
                T: tonic::codegen::Service<
                    http::Request<tonic::body::BoxBody>,
                    Response = http::Response<
                        <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                    >,
                >,
                <T as tonic::codegen::Service<
                    http::Request<tonic::body::BoxBody>,
                >>::Error: Into<StdError> + Send + Sync,
        {
            FetchClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn block(
            &mut self,
            request: impl tonic::IntoRequest<super::SingleBlockRequest>,
        ) -> Result<tonic::Response<super::SingleBlockResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sf.firehose.v2.Fetch/Block",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
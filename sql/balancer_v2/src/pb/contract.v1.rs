// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub vault_authorizer_changeds: ::prost::alloc::vec::Vec<VaultAuthorizerChanged>,
    #[prost(message, repeated, tag="2")]
    pub vault_external_balance_transfers: ::prost::alloc::vec::Vec<VaultExternalBalanceTransfer>,
    #[prost(message, repeated, tag="3")]
    pub vault_flash_loans: ::prost::alloc::vec::Vec<VaultFlashLoan>,
    #[prost(message, repeated, tag="4")]
    pub vault_internal_balance_changeds: ::prost::alloc::vec::Vec<VaultInternalBalanceChanged>,
    #[prost(message, repeated, tag="5")]
    pub vault_paused_state_changeds: ::prost::alloc::vec::Vec<VaultPausedStateChanged>,
    #[prost(message, repeated, tag="6")]
    pub vault_pool_balance_changeds: ::prost::alloc::vec::Vec<VaultPoolBalanceChanged>,
    #[prost(message, repeated, tag="7")]
    pub vault_pool_balance_manageds: ::prost::alloc::vec::Vec<VaultPoolBalanceManaged>,
    #[prost(message, repeated, tag="8")]
    pub vault_pool_registereds: ::prost::alloc::vec::Vec<VaultPoolRegistered>,
    #[prost(message, repeated, tag="9")]
    pub vault_relayer_approval_changeds: ::prost::alloc::vec::Vec<VaultRelayerApprovalChanged>,
    #[prost(message, repeated, tag="10")]
    pub vault_swaps: ::prost::alloc::vec::Vec<VaultSwap>,
    #[prost(message, repeated, tag="11")]
    pub vault_tokens_deregistereds: ::prost::alloc::vec::Vec<VaultTokensDeregistered>,
    #[prost(message, repeated, tag="12")]
    pub vault_tokens_registereds: ::prost::alloc::vec::Vec<VaultTokensRegistered>,
    #[prost(message, repeated, tag="13")]
    pub pools_amp_update_starteds: ::prost::alloc::vec::Vec<PoolsAmpUpdateStarted>,
    #[prost(message, repeated, tag="14")]
    pub pools_amp_update_stoppeds: ::prost::alloc::vec::Vec<PoolsAmpUpdateStopped>,
    #[prost(message, repeated, tag="15")]
    pub pools_approvals: ::prost::alloc::vec::Vec<PoolsApproval>,
    #[prost(message, repeated, tag="16")]
    pub pools_paused_state_changeds: ::prost::alloc::vec::Vec<PoolsPausedStateChanged>,
    #[prost(message, repeated, tag="17")]
    pub pools_protocol_fee_percentage_cache_updateds: ::prost::alloc::vec::Vec<PoolsProtocolFeePercentageCacheUpdated>,
    #[prost(message, repeated, tag="18")]
    pub pools_recovery_mode_state_changeds: ::prost::alloc::vec::Vec<PoolsRecoveryModeStateChanged>,
    #[prost(message, repeated, tag="19")]
    pub pools_swap_fee_percentage_changeds: ::prost::alloc::vec::Vec<PoolsSwapFeePercentageChanged>,
    #[prost(message, repeated, tag="20")]
    pub pools_token_rate_cache_updateds: ::prost::alloc::vec::Vec<PoolsTokenRateCacheUpdated>,
    #[prost(message, repeated, tag="21")]
    pub pools_token_rate_provider_sets: ::prost::alloc::vec::Vec<PoolsTokenRateProviderSet>,
    #[prost(message, repeated, tag="22")]
    pub pools_transfers: ::prost::alloc::vec::Vec<PoolsTransfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Calls {
    #[prost(message, repeated, tag="1")]
    pub vault_call_batch_swaps: ::prost::alloc::vec::Vec<VaultBatchSwapCall>,
    #[prost(message, repeated, tag="2")]
    pub vault_call_deregister_tokens: ::prost::alloc::vec::Vec<VaultDeregisterTokensCall>,
    #[prost(message, repeated, tag="3")]
    pub vault_call_exit_pools: ::prost::alloc::vec::Vec<VaultExitPoolCall>,
    #[prost(message, repeated, tag="4")]
    pub vault_call_flash_loans: ::prost::alloc::vec::Vec<VaultFlashLoanCall>,
    #[prost(message, repeated, tag="5")]
    pub vault_call_join_pools: ::prost::alloc::vec::Vec<VaultJoinPoolCall>,
    #[prost(message, repeated, tag="6")]
    pub vault_call_manage_pool_balances: ::prost::alloc::vec::Vec<VaultManagePoolBalanceCall>,
    #[prost(message, repeated, tag="7")]
    pub vault_call_manage_user_balances: ::prost::alloc::vec::Vec<VaultManageUserBalanceCall>,
    #[prost(message, repeated, tag="8")]
    pub vault_call_query_batch_swaps: ::prost::alloc::vec::Vec<VaultQueryBatchSwapCall>,
    #[prost(message, repeated, tag="9")]
    pub vault_call_register_pools: ::prost::alloc::vec::Vec<VaultRegisterPoolCall>,
    #[prost(message, repeated, tag="10")]
    pub vault_call_register_tokens: ::prost::alloc::vec::Vec<VaultRegisterTokensCall>,
    #[prost(message, repeated, tag="11")]
    pub vault_call_set_authorizers: ::prost::alloc::vec::Vec<VaultSetAuthorizerCall>,
    #[prost(message, repeated, tag="12")]
    pub vault_call_set_pauseds: ::prost::alloc::vec::Vec<VaultSetPausedCall>,
    #[prost(message, repeated, tag="13")]
    pub vault_call_set_relayer_approvals: ::prost::alloc::vec::Vec<VaultSetRelayerApprovalCall>,
    #[prost(message, repeated, tag="14")]
    pub vault_call_swaps: ::prost::alloc::vec::Vec<VaultSwapCall>,
    #[prost(message, repeated, tag="15")]
    pub pools_call_approves: ::prost::alloc::vec::Vec<PoolsApproveCall>,
    #[prost(message, repeated, tag="16")]
    pub pools_call_decrease_allowances: ::prost::alloc::vec::Vec<PoolsDecreaseAllowanceCall>,
    #[prost(message, repeated, tag="17")]
    pub pools_call_disable_recovery_modes: ::prost::alloc::vec::Vec<PoolsDisableRecoveryModeCall>,
    #[prost(message, repeated, tag="18")]
    pub pools_call_enable_recovery_modes: ::prost::alloc::vec::Vec<PoolsEnableRecoveryModeCall>,
    #[prost(message, repeated, tag="19")]
    pub pools_call_increase_allowances: ::prost::alloc::vec::Vec<PoolsIncreaseAllowanceCall>,
    #[prost(message, repeated, tag="20")]
    pub pools_call_on_exit_pools: ::prost::alloc::vec::Vec<PoolsOnExitPoolCall>,
    #[prost(message, repeated, tag="21")]
    pub pools_call_on_join_pools: ::prost::alloc::vec::Vec<PoolsOnJoinPoolCall>,
    #[prost(message, repeated, tag="22")]
    pub pools_call_on_swaps: ::prost::alloc::vec::Vec<PoolsOnSwapCall>,
    #[prost(message, repeated, tag="23")]
    pub pools_call_pauses: ::prost::alloc::vec::Vec<PoolsPauseCall>,
    #[prost(message, repeated, tag="24")]
    pub pools_call_permits: ::prost::alloc::vec::Vec<PoolsPermitCall>,
    #[prost(message, repeated, tag="25")]
    pub pools_call_query_exits: ::prost::alloc::vec::Vec<PoolsQueryExitCall>,
    #[prost(message, repeated, tag="26")]
    pub pools_call_query_joins: ::prost::alloc::vec::Vec<PoolsQueryJoinCall>,
    #[prost(message, repeated, tag="27")]
    pub pools_call_set_asset_manager_pool_configs: ::prost::alloc::vec::Vec<PoolsSetAssetManagerPoolConfigCall>,
    #[prost(message, repeated, tag="28")]
    pub pools_call_set_swap_fee_percentages: ::prost::alloc::vec::Vec<PoolsSetSwapFeePercentageCall>,
    #[prost(message, repeated, tag="29")]
    pub pools_call_set_token_rate_cache_durations: ::prost::alloc::vec::Vec<PoolsSetTokenRateCacheDurationCall>,
    #[prost(message, repeated, tag="30")]
    pub pools_call_start_amplification_parameter_updates: ::prost::alloc::vec::Vec<PoolsStartAmplificationParameterUpdateCall>,
    #[prost(message, repeated, tag="31")]
    pub pools_call_stop_amplification_parameter_updates: ::prost::alloc::vec::Vec<PoolsStopAmplificationParameterUpdateCall>,
    #[prost(message, repeated, tag="32")]
    pub pools_call_transfers: ::prost::alloc::vec::Vec<PoolsTransferCall>,
    #[prost(message, repeated, tag="33")]
    pub pools_call_transfer_froms: ::prost::alloc::vec::Vec<PoolsTransferFromCall>,
    #[prost(message, repeated, tag="34")]
    pub pools_call_unpauses: ::prost::alloc::vec::Vec<PoolsUnpauseCall>,
    #[prost(message, repeated, tag="35")]
    pub pools_call_update_protocol_fee_percentage_caches: ::prost::alloc::vec::Vec<PoolsUpdateProtocolFeePercentageCacheCall>,
    #[prost(message, repeated, tag="36")]
    pub pools_call_update_token_rate_caches: ::prost::alloc::vec::Vec<PoolsUpdateTokenRateCacheCall>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultAuthorizerChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub new_authorizer: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultExternalBalanceTransfer {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultFlashLoan {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub fee_amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultInternalBalanceChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub delta: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultPausedStateChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bool, tag="5")]
    pub paused: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultPoolBalanceChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub liquidity_provider: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="7")]
    pub tokens: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag="8")]
    pub deltas: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="9")]
    pub protocol_fee_amounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultPoolBalanceManaged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub asset_manager: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub cash_delta: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub managed_delta: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultPoolRegistered {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub pool_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub specialization: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultRelayerApprovalChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub relayer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="7")]
    pub approved: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultSwap {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub token_in: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub token_out: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount_in: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub amount_out: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultTokensDeregistered {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="6")]
    pub tokens: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultTokensRegistered {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="6")]
    pub tokens: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="7")]
    pub asset_managers: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultBatchSwapCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(uint64, tag="6")]
    pub kind: u64,
    #[prost(bytes="vec", repeated, tag="7")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag="8")]
    pub limits: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="9")]
    pub deadline: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="10")]
    pub output_asset_deltas: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultDeregisterTokensCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="7")]
    pub tokens: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultExitPoolCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultFlashLoanCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="7")]
    pub tokens: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag="8")]
    pub amounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes="vec", tag="9")]
    pub user_data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultJoinPoolCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultManagePoolBalanceCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultManageUserBalanceCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultQueryBatchSwapCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(uint64, tag="6")]
    pub kind: u64,
    #[prost(bytes="vec", repeated, tag="7")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag="8")]
    pub output_param0: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultRegisterPoolCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(uint64, tag="6")]
    pub specialization: u64,
    #[prost(bytes="vec", tag="7")]
    pub output_param0: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultRegisterTokensCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="7")]
    pub tokens: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="8")]
    pub asset_managers: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultSetAuthorizerCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub new_authorizer: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultSetPausedCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bool, tag="6")]
    pub paused: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultSetRelayerApprovalCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub relayer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="8")]
    pub approved: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultSwapCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub limit: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub deadline: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub output_amount_calculated: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsAmpUpdateStarted {
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
    pub start_value: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub end_value: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub start_time: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub end_time: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsAmpUpdateStopped {
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
    pub current_value: ::prost::alloc::string::String,
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
pub struct PoolsPausedStateChanged {
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
    #[prost(bool, tag="6")]
    pub paused: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsProtocolFeePercentageCacheUpdated {
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
    pub fee_type: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub protocol_fee_percentage: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsRecoveryModeStateChanged {
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
    #[prost(bool, tag="6")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsSwapFeePercentageChanged {
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
    pub swap_fee_percentage: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsTokenRateCacheUpdated {
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
    pub token_index: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsTokenRateProviderSet {
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
    pub token_index: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub provider: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub cache_duration: ::prost::alloc::string::String,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsApproveCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub spender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub output_param0: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsDecreaseAllowanceCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub spender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub output_param0: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsDisableRecoveryModeCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsEnableRecoveryModeCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsIncreaseAllowanceCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub spender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub added_value: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub output_param0: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsOnExitPoolCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="10")]
    pub balances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="11")]
    pub last_change_block: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub protocol_swap_fee_percentage: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="13")]
    pub user_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="14")]
    pub output_param0: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="15")]
    pub output_param1: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsOnJoinPoolCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="10")]
    pub balances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="11")]
    pub last_change_block: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub protocol_swap_fee_percentage: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="13")]
    pub user_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="14")]
    pub output_param0: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="15")]
    pub output_param1: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsOnSwapCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="7")]
    pub balances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="8")]
    pub index_in: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub index_out: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub output_param0: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsPauseCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsPermitCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub spender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="9")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub deadline: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub v: u64,
    #[prost(bytes="vec", tag="12")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub s: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsQueryExitCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="10")]
    pub balances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="11")]
    pub last_change_block: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub protocol_swap_fee_percentage: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="13")]
    pub user_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="14")]
    pub output_bpt_in: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="15")]
    pub output_amounts_out: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsQueryJoinCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub pool_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="10")]
    pub balances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="11")]
    pub last_change_block: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub protocol_swap_fee_percentage: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="13")]
    pub user_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="14")]
    pub output_bpt_out: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="15")]
    pub output_amounts_in: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsSetAssetManagerPoolConfigCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub pool_config: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsSetSwapFeePercentageCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub swap_fee_percentage: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsSetTokenRateCacheDurationCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub duration: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsStartAmplificationParameterUpdateCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub raw_end_value: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub end_time: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsStopAmplificationParameterUpdateCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsTransferCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub output_param0: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsTransferFromCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="9")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bool, tag="10")]
    pub output_param0: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsUnpauseCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsUpdateProtocolFeePercentageCacheCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsUpdateTokenRateCacheCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub token: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)

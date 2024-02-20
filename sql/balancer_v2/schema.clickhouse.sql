CREATE TABLE IF NOT EXISTS vault_authorizer_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_authorizer" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS vault_external_balance_transfer (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "amount" UInt256,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    "token" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS vault_flash_loan (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "amount" UInt256,
    "fee_amount" UInt256,
    "recipient" VARCHAR(40),
    "token" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS vault_internal_balance_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "delta" Int256,
    "token" VARCHAR(40),
    "user" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS vault_paused_state_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "paused" BOOL
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS vault_pool_balance_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "deltas" Array(Int256),
    "liquidity_provider" VARCHAR(40),
    "pool_id" TEXT,
    "protocol_fee_amounts" Array(UInt256),
    "tokens" Array(VARCHAR(40))
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS vault_pool_balance_managed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "asset_manager" VARCHAR(40),
    "cash_delta" Int256,
    "managed_delta" Int256,
    "pool_id" TEXT,
    "token" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS vault_pool_registered (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "pool_address" VARCHAR(40),
    "pool_id" TEXT,
    "specialization" UInt8
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS vault_relayer_approval_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "approved" BOOL,
    "relayer" VARCHAR(40),
    "sender" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS vault_swap (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "amount_in" UInt256,
    "amount_out" UInt256,
    "pool_id" TEXT,
    "token_in" VARCHAR(40),
    "token_out" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS vault_tokens_deregistered (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "pool_id" TEXT,
    "tokens" Array(VARCHAR(40))
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS vault_tokens_registered (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "asset_managers" Array(VARCHAR(40)),
    "pool_id" TEXT,
    "tokens" Array(VARCHAR(40))
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");

CREATE TABLE IF NOT EXISTS vault_call_batch_swap (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "assets" Array(VARCHAR(40)),
    "deadline" UInt256,
    "kind" UInt8,
    "limits" Array(Int256),
    "output_asset_deltas" Array(Int256)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_deregister_tokens (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "pool_id" TEXT,
    "tokens" Array(VARCHAR(40))
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_exit_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "pool_id" TEXT,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_flash_loan (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "amounts" Array(UInt256),
    "recipient" VARCHAR(40),
    "tokens" Array(VARCHAR(40)),
    "user_data" TEXT
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_join_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "pool_id" TEXT,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_manage_pool_balance (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_manage_user_balance (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_query_batch_swap (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "assets" Array(VARCHAR(40)),
    "kind" UInt8,
    "output_param0" Array(Int256)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_register_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "output_param0" TEXT,
    "specialization" UInt8
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_register_tokens (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "asset_managers" Array(VARCHAR(40)),
    "pool_id" TEXT,
    "tokens" Array(VARCHAR(40))
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_set_authorizer (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "new_authorizer" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_set_paused (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "paused" BOOL
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_set_relayer_approval (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "approved" BOOL,
    "relayer" VARCHAR(40),
    "sender" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS vault_call_swap (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "deadline" UInt256,
    "limit" UInt256,
    "output_amount_calculated" UInt256
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_amp_update_started (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "end_time" UInt256,
    "end_value" UInt256,
    "start_time" UInt256,
    "start_value" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_amp_update_stopped (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "current_value" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_approval (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "owner" VARCHAR(40),
    "spender" VARCHAR(40),
    "value" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_paused_state_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "paused" BOOL
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_protocol_fee_percentage_cache_updated (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "fee_type" UInt256,
    "protocol_fee_percentage" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_recovery_mode_state_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "enabled" BOOL
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_swap_fee_percentage_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "swap_fee_percentage" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_token_rate_cache_updated (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "rate" UInt256,
    "token_index" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_token_rate_provider_set (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "cache_duration" UInt256,
    "provider" VARCHAR(40),
    "token_index" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_transfer (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "from" VARCHAR(40),
    "to" VARCHAR(40),
    "value" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_call_approve (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "amount" UInt256,
    "output_param0" BOOL,
    "spender" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_decrease_allowance (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "amount" UInt256,
    "output_param0" BOOL,
    "spender" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_disable_recovery_mode (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_enable_recovery_mode (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_increase_allowance (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "added_value" UInt256,
    "output_param0" BOOL,
    "spender" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_on_exit_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "balances" Array(UInt256),
    "last_change_block" UInt256,
    "output_param0" Array(UInt256),
    "output_param1" Array(UInt256),
    "pool_id" TEXT,
    "protocol_swap_fee_percentage" UInt256,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    "user_data" TEXT
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_on_join_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "balances" Array(UInt256),
    "last_change_block" UInt256,
    "output_param0" Array(UInt256),
    "output_param1" Array(UInt256),
    "pool_id" TEXT,
    "protocol_swap_fee_percentage" UInt256,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    "user_data" TEXT
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_on_swap (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "balances" Array(UInt256),
    "index_in" UInt256,
    "index_out" UInt256,
    "output_param0" UInt256
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_pause (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_permit (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "deadline" UInt256,
    "owner" VARCHAR(40),
    "r" TEXT,
    "s" TEXT,
    "spender" VARCHAR(40),
    "v" UInt8,
    "value" UInt256
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_query_exit (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "balances" Array(UInt256),
    "last_change_block" UInt256,
    "output_amounts_out" Array(UInt256),
    "output_bpt_in" UInt256,
    "pool_id" TEXT,
    "protocol_swap_fee_percentage" UInt256,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    "user_data" TEXT
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_query_join (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "balances" Array(UInt256),
    "last_change_block" UInt256,
    "output_amounts_in" Array(UInt256),
    "output_bpt_out" UInt256,
    "pool_id" TEXT,
    "protocol_swap_fee_percentage" UInt256,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    "user_data" TEXT
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_set_asset_manager_pool_config (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "pool_config" TEXT,
    "token" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_set_swap_fee_percentage (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "swap_fee_percentage" UInt256
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_set_token_rate_cache_duration (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "duration" UInt256,
    "token" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_start_amplification_parameter_update (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "end_time" UInt256,
    "raw_end_value" UInt256
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_stop_amplification_parameter_update (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_transfer (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "amount" UInt256,
    "output_param0" BOOL,
    "recipient" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_transfer_from (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "amount" UInt256,
    "output_param0" BOOL,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_unpause (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_update_protocol_fee_percentage_cache (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS pools_call_update_token_rate_cache (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "token" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");

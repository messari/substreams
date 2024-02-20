CREATE TABLE IF NOT EXISTS vault_authorizer_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_authorizer" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS vault_external_balance_transfer (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    "token" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS vault_flash_loan (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    "fee_amount" DECIMAL,
    "recipient" VARCHAR(40),
    "token" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS vault_internal_balance_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "delta" DECIMAL,
    "token" VARCHAR(40),
    "user" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS vault_paused_state_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "paused" BOOL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS vault_pool_balance_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "deltas" DECIMAL[],
    "liquidity_provider" VARCHAR(40),
    "pool_id" TEXT,
    "protocol_fee_amounts" DECIMAL[],
    "tokens" VARCHAR(40)[],
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS vault_pool_balance_managed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "asset_manager" VARCHAR(40),
    "cash_delta" DECIMAL,
    "managed_delta" DECIMAL,
    "pool_id" TEXT,
    "token" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS vault_pool_registered (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "pool_address" VARCHAR(40),
    "pool_id" TEXT,
    "specialization" INT,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS vault_relayer_approval_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "approved" BOOL,
    "relayer" VARCHAR(40),
    "sender" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS vault_swap (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount_in" DECIMAL,
    "amount_out" DECIMAL,
    "pool_id" TEXT,
    "token_in" VARCHAR(40),
    "token_out" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS vault_tokens_deregistered (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "pool_id" TEXT,
    "tokens" VARCHAR(40)[],
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS vault_tokens_registered (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "asset_managers" VARCHAR(40)[],
    "pool_id" TEXT,
    "tokens" VARCHAR(40)[],
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS vault_call_batch_swap (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "assets" VARCHAR(40)[],
    "deadline" DECIMAL,
    "kind" INT,
    "limits" DECIMAL[],
    "output_asset_deltas" DECIMAL[],
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_deregister_tokens (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "pool_id" TEXT,
    "tokens" VARCHAR(40)[],
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_exit_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "pool_id" TEXT,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_flash_loan (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "amounts" DECIMAL[],
    "recipient" VARCHAR(40),
    "tokens" VARCHAR(40)[],
    "user_data" TEXT,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_join_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "pool_id" TEXT,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_manage_pool_balance (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_manage_user_balance (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_query_batch_swap (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "assets" VARCHAR(40)[],
    "kind" INT,
    "output_param0" DECIMAL[],
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_register_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "output_param0" TEXT,
    "specialization" INT,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_register_tokens (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "asset_managers" VARCHAR(40)[],
    "pool_id" TEXT,
    "tokens" VARCHAR(40)[],
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_set_authorizer (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "new_authorizer" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_set_paused (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "paused" BOOL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_set_relayer_approval (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "approved" BOOL,
    "relayer" VARCHAR(40),
    "sender" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS vault_call_swap (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "deadline" DECIMAL,
    "limit" DECIMAL,
    "output_amount_calculated" DECIMAL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);


CREATE TABLE IF NOT EXISTS pools_amp_update_started (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "end_time" DECIMAL,
    "end_value" DECIMAL,
    "start_time" DECIMAL,
    "start_value" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_amp_update_stopped (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "current_value" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_approval (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "owner" VARCHAR(40),
    "spender" VARCHAR(40),
    "value" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_paused_state_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "paused" BOOL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_protocol_fee_percentage_cache_updated (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "fee_type" DECIMAL,
    "protocol_fee_percentage" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_recovery_mode_state_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "enabled" BOOL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_swap_fee_percentage_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "swap_fee_percentage" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_token_rate_cache_updated (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "rate" DECIMAL,
    "token_index" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_token_rate_provider_set (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "cache_duration" DECIMAL,
    "provider" VARCHAR(40),
    "token_index" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_transfer (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "from" VARCHAR(40),
    "to" VARCHAR(40),
    "value" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);CREATE TABLE IF NOT EXISTS pools_call_approve (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "amount" DECIMAL,
    "output_param0" BOOL,
    "spender" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_decrease_allowance (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "amount" DECIMAL,
    "output_param0" BOOL,
    "spender" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_disable_recovery_mode (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_enable_recovery_mode (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_increase_allowance (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "added_value" DECIMAL,
    "output_param0" BOOL,
    "spender" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_on_exit_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "balances" DECIMAL[],
    "last_change_block" DECIMAL,
    "output_param0" DECIMAL[],
    "output_param1" DECIMAL[],
    "pool_id" TEXT,
    "protocol_swap_fee_percentage" DECIMAL,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    "user_data" TEXT,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_on_join_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "balances" DECIMAL[],
    "last_change_block" DECIMAL,
    "output_param0" DECIMAL[],
    "output_param1" DECIMAL[],
    "pool_id" TEXT,
    "protocol_swap_fee_percentage" DECIMAL,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    "user_data" TEXT,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_on_swap (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "balances" DECIMAL[],
    "index_in" DECIMAL,
    "index_out" DECIMAL,
    "output_param0" DECIMAL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_pause (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_permit (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "deadline" DECIMAL,
    "owner" VARCHAR(40),
    "r" TEXT,
    "s" TEXT,
    "spender" VARCHAR(40),
    "v" INT,
    "value" DECIMAL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_query_exit (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "balances" DECIMAL[],
    "last_change_block" DECIMAL,
    "output_amounts_out" DECIMAL[],
    "output_bpt_in" DECIMAL,
    "pool_id" TEXT,
    "protocol_swap_fee_percentage" DECIMAL,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    "user_data" TEXT,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_query_join (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "balances" DECIMAL[],
    "last_change_block" DECIMAL,
    "output_amounts_in" DECIMAL[],
    "output_bpt_out" DECIMAL,
    "pool_id" TEXT,
    "protocol_swap_fee_percentage" DECIMAL,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    "user_data" TEXT,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_set_asset_manager_pool_config (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "pool_config" TEXT,
    "token" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_set_swap_fee_percentage (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "swap_fee_percentage" DECIMAL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_set_token_rate_cache_duration (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "duration" DECIMAL,
    "token" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_start_amplification_parameter_update (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "end_time" DECIMAL,
    "raw_end_value" DECIMAL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_stop_amplification_parameter_update (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_transfer (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "amount" DECIMAL,
    "output_param0" BOOL,
    "recipient" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_transfer_from (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "amount" DECIMAL,
    "output_param0" BOOL,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_unpause (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_update_protocol_fee_percentage_cache (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS pools_call_update_token_rate_cache (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "call_address" VARCHAR(40),
    "token" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);



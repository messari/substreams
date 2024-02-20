{{
    config(
        materialized = 'table',
        alias = 'BALANCER_V2_WITHDRAWS_FLATTENED'
    )
}}

WITH on_exit AS (
    SELECT 
        call_tx_hash AS transaction_hash
        , sender
        , recipient
        , pool_id
        , balances AS input_token_balances
    FROM pools_call_on_exit_pool
)

, raw_withdraws AS (
    SELECT
        ve.evt_tx_hash AS transaction_hash
        , ve.evt_index AS log_index
        , ve.evt_block_number AS block_number
        , ve.evt_block_time AS block_timestamp
        , ve.deltas
        , ve.liquidity_provider
        , ve.protocol_fee_amounts 
        , ve.tokens AS input_tokens
        , ve.pool_id AS pool__id_bytes
        , oe.recipient
        , oe.input_token_balances
    FROM vault_pool_balance_changed ve
    LEFT JOIN on_exit oe
        ON oe.transaction_hash = ve.evt_tx_hash
            AND oe.sender = ve.liquidity_provider
            AND oe.pool_id = ve.pool_id
    WHERE recipient IS NOT NULL
) 

, withdraws_flattened AS (
    SELECT 
        rw.transaction_hash
        , rw.block_number
        , rw.block_timestamp
        , rw.log_index
        , rw.liquidity_provider
        , rw.recipient
        -- , RW.PROTOCOL__ID
        , rw.deltas
        , rw.pool__id_bytes
        , SUBSTR(rw.pool__id_bytes, 1, 40) AS pool__id
        , token_idx
        , rw.deltas[token_idx] AS input_token_amount
        , rw.input_tokens[token_idx] AS input_token__id
        , rw.protocol_fee_amounts[token_idx] AS protocol_fee_amount
        , sum(delta) OVER (PARTITION BY rw.transaction_hash, rw.log_index) AS deltas_sum
        , rw.input_token_balances[token_idx] - abs(rw.deltas[token_idx]) AS input_token_balance
    FROM raw_withdraws rw
    CROSS JOIN LATERAL unnest(deltas) WITH ORDINALITY AS F(delta, token_idx)
)

, burnt_amount AS (
    SELECT
        wf.transaction_hash 
        , wf.log_index
        , wf.deltas
        , pt.from
        , pt.value AS output_token_amount
        , pt.evt_address AS output_token__id
    FROM withdraws_flattened wf
    LEFT JOIN pools_transfer pt
        ON pt.evt_tx_hash = wf.transaction_hash
            AND pt.to = '0000000000000000000000000000000000000000'
            AND pt.from IN (wf.recipient, wf.liquidity_provider)
            AND pt.evt_address = wf.pool__id
    WHERE wf.token_idx = 1
)

, final AS (
    SELECT
        wf.transaction_hash
        , wf.block_number
        , wf.block_timestamp
        , wf.log_index
        , wf.token_idx
        , wf.recipient
        , wf.pool__id AS "to"
        , wf.liquidity_provider AS "from"
        , wf.pool__id
        , wf.input_token__id
        , wf.input_token_amount
        , wf.input_token_balance
        , ba.output_token__id
        , ba.output_token_amount
        -- , wf.PROTOCOL__ID
        , wf.protocol_fee_amount
        , null AS input_token_amount_usd
        , null AS protocol_fee_amount_usd
    FROM withdraws_flattened wf
    LEFT JOIN burnt_amount ba
        ON ba.transaction_hash = wf.transaction_hash 
            AND ba.from IN (wf.recipient, wf.liquidity_provider)
            AND ba.deltas = wf.deltas
)

SELECT * FROM final

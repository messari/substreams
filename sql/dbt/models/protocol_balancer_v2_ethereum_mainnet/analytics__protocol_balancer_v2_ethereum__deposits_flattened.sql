{{
    config(
        materialized = 'table',
        alias = 'BALANCER_V2_DEPOSITS_FLATTENED'
    )
}}

WITH on_join AS (
    SELECT 
        call_tx_hash AS transaction_hash
        , sender
        , recipient
        , pool_id
        , output_param0 AS deltas
        , balances AS input_token_balances
    FROM pools_call_on_join_pool
)

, raw_deposits AS (
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
        , oj.recipient
        , oj.input_token_balances
    FROM vault_pool_balance_changed ve
    LEFT JOIN on_join oj
        ON oj.transaction_hash = ve.evt_tx_hash
            AND oj.deltas = ve.deltas
            AND oj.pool_id = ve.pool_id
    WHERE recipient IS NOT NULL
) 

, deposits_flattened AS (
    SELECT 
        rd.transaction_hash
        , rd.block_number
        , rd.block_timestamp
        , rd.log_index
        , rd.liquidity_provider
        , rd.recipient
        -- , RW.PROTOCOL__ID
        , rd.deltas
        , rd.pool__id_bytes
        , SUBSTR(rd.pool__id_bytes, 1, 40) AS pool__id
        , token_idx
        , rd.deltas[token_idx] AS input_token_amount
        , rd.input_tokens[token_idx] AS input_token__id
        , rd.protocol_fee_amounts[token_idx] AS protocol_fee_amount
        , sum(delta) OVER (PARTITION BY rd.transaction_hash, rd.log_index) AS deltas_sum
        , rd.input_token_balances[token_idx] + abs(rd.deltas[token_idx]) AS input_token_balance
    FROM raw_deposits rd
    CROSS JOIN LATERAL unnest(deltas) WITH ORDINALITY AS F(delta, token_idx)
)

, mint_amount AS (
    SELECT
        df.transaction_hash 
        , df.log_index
        , df.deltas
        , pt.to
        , pt.value AS output_token_amount
        , pt.evt_address AS output_token__id
    FROM deposits_flattened df
    LEFT JOIN pools_transfer pt
        ON pt.evt_tx_hash = df.transaction_hash
            AND pt.from = '0000000000000000000000000000000000000000'
            AND pt.to IN (df.recipient, df.liquidity_provider)
            AND pt.evt_address = df.pool__id
    WHERE df.token_idx = 1
)

, final AS (
    SELECT
        df.transaction_hash
        , df.block_number
        , df.block_timestamp
        , df.log_index
        , df.token_idx
        , df.recipient
        , df.pool__id AS "to"
        , df.liquidity_provider AS "from"
        , df.pool__id
        , df.input_token__id
        , df.input_token_amount
        , df.input_token_balance
        , ma.output_token__id
        , ma.output_token_amount
        -- , wf.PROTOCOL__ID
        , df.protocol_fee_amount
        , null AS input_token_amount_usd
        , null AS protocol_fee_amount_usd
    FROM deposits_flattened df
    LEFT JOIN mint_amount ma
        ON ma.transaction_hash = df.transaction_hash 
            AND ma.to IN (df.recipient, df.liquidity_provider)
            AND ma.deltas = df.deltas
)

SELECT * FROM final

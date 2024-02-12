{{
    config(
        materialized = 'incremental',
        alias = 'WITHDRAWS',
        unique_key='ID',
    )
}}

WITH pools as (
    SELECT 
        pair AS id
        , '0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f' AS protocol__id 
        , token0
        , token1
    FROM factory_pair_created
)

, burn_evts AS (
    SELECT 
        evt_tx_hash as transaction_hash
        , evt_index as log_index
        , evt_address as contract_address
        , pools.protocol__id
        , pools.token0
        , pools.token1
        , sender
        , amount0
        , amount1
        , evt_block_number as block_number
        , evt_block_time as block_timestamp
        , DATE_TRUNC('hour', evt_block_time) AS HOUR
    FROM pools_burn burn
        INNER JOIN pools ON burn.evt_address = pools.id
)

, sync AS (
    SELECT
        evt_tx_hash as transaction_hash
        , evt_index as log_index
        , reserve0
        , reserve1
    FROM pools_sync sync
)

, transfers AS (
    SELECT
        evt_tx_hash as transaction_hash
        , evt_index as log_index
        , evt_address as contract_address
        , "from"
        , value as liquidity
    FROM pools_transfer t
    WHERE "to" = '0000000000000000000000000000000000000000'
)

, burn_transfer AS (
    SELECT
        b.transaction_hash,
        b.log_index,
        t.liquidity
    FROM burn_evts b
        INNER JOIN transfers t
            ON b.transaction_hash = t.transaction_hash
                AND b.log_index > t.log_index
                AND b.contract_address = t.contract_address
)

, most_cols AS (
    SELECT
        'WITHDRAW-' || b.transaction_hash || '-' || b.log_index AS id
        , b.transaction_hash as hash
        , b.log_index
        , p.protocol__id
        , b.sender AS "to"
        , b.contract_address as "from"
        , b.block_number
        , b.block_timestamp AS timestamp
        , b.contract_address AS pool__id
        , ARRAY[p.token0, p.token1] AS input_tokens
        , b.contract_address as output_token__id
        , ARRAY[amount0, amount1] AS input_token_amounts
        , t.liquidity AS output_token_amount
        , ARRAY[s.reserve0, s.reserve1] as reserve_amounts
        
        -- TODO:
        , 0 AS _amount0_usd
        , 0 AS _amount1_usd 
        , 0 AS amount_usd
    FROM burn_evts b
        INNER JOIN pools p ON b.contract_address = p.id
        LEFT JOIN sync s ON b.transaction_hash = s.transaction_hash AND b.log_index = s.log_index + 1
        LEFT JOIN burn_transfer t ON b.transaction_hash = t.transaction_hash AND b.log_index = t.log_index
)

, final AS (
    SELECT
        id
        , hash
        , log_index
        , protocol__id
        , "to"
        , "from"
        , block_number
        , timestamp
        , input_tokens
        , output_token__id
        , input_token_amounts
        , output_token_amount
        , reserve_amounts
        , amount_usd
        , pool__id
    FROM most_cols
)

SELECT * FROM final

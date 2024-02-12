{{
    config(
        materialized = 'incremental',
        alias = 'SWAPS',
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

, swap_evts AS (
    SELECT 
        evt_tx_hash as transaction_hash
        , evt_index as log_index
        , evt_address as contract_address
        , pools.protocol__id
        , pools.token0
        , pools.token1
        , "to"
        , sender
        , amount0_in
        , amount0_out
        , amount1_in
        , amount1_out
        , evt_block_number as block_number
        , evt_block_time as block_timestamp
        , DATE_TRUNC('hour', evt_block_time) AS HOUR
    FROM pools_swap swap
        INNER JOIN pools ON swap.evt_address = pools.id
)

, sync AS (
    SELECT
        evt_tx_hash as transaction_hash
        , evt_index as log_index
        , reserve0
        , reserve1
    FROM pools_sync sync
)

, final AS (
    SELECT
        'SWAP-' || s.transaction_hash || '-' || s.log_index AS id
        , s.transaction_hash as hash
        , s.log_index
        , p.protocol__id
        , s."to" AS "to"
        , sender AS "from"
        , s.block_number
        , s.block_timestamp AS timestamp
        
        , CASE 
            WHEN amount0_in > 0 THEN p.token0
            ELSE p.token1
        END AS token_in__id
        , CASE 
            WHEN amount0_in > 0 THEN amount0_in
            ELSE amount1_in
        END AS amount_in
        , 0 AS amount_in_usd

        , CASE 
            WHEN amount0_in > 0 THEN p.token1
            ELSE p.token0
        END AS token_out__id
        , CASE 
            WHEN amount0_in > 0 THEN amount1_out
            ELSE amount0_out
        END AS amount_out
        , 0 AS amount_out_usd

        , ARRAY[sync.reserve0, sync.reserve1] as reserve_amounts
        , s.contract_address AS pool__id
    FROM swap_evts s
        INNER JOIN pools p ON s.contract_address = p.id
        LEFT JOIN sync ON s.transaction_hash = sync.transaction_hash AND s.log_index = sync.log_index + 1
)

SELECT * FROM final

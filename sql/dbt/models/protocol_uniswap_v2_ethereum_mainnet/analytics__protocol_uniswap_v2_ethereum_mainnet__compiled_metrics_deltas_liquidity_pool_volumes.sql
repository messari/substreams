{{
    config(
        materialized = 'view',
        alias = 'COMPILED_METRICS_DELTAS_LIQUIDITY_POOL_VOLUMES',
        unique_key='id',
    )
}}

{% set deltas_volumes = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_deltas_volumes') %}

WITH
    incremental_deltas AS (SELECT * FROM {{ deltas_volumes }})

    , prepared_incremental_deltas AS (
        SELECT
            id.*
            , id.delta_in AS delta
            , token_in__id AS token__id
            , DATE_TRUNC('hour', id.block_timestamp) AS hour
            , 0 AS delta_usd
        FROM incremental_deltas id
    )

    , final AS (
        SELECT
            pid.id
            , pid.pool__id
            , pid.token__id
            , pid.hour

            -- Delta Calculations
            , pid.delta
            , pid.delta_usd
            
            -- Cumulative Sum Delta Calculations
            , SUM(pid.delta)     OVER (PARTITION BY pid.pool__id, pid.token__id ORDER BY pid.block_number, pid.transaction_index, pid.log_index) AS cumulative_sum
            , SUM(pid.delta_usd) OVER (PARTITION BY pid.pool__id, pid.token__id ORDER BY pid.block_number, pid.transaction_index, pid.log_index) AS cumulative_sum_usd
            
            -- Hourly Delta Calculations
            , SUM(pid.delta)      OVER (PARTITION BY pid.pool__id, pid.token__id, pid.hour ORDER BY pid.block_number, pid.transaction_index, pid.log_index) AS hour_sum
            , SUM(pid.delta_usd)  OVER (PARTITION BY pid.pool__id, pid.token__id, pid.hour ORDER BY pid.block_number, pid.transaction_index, pid.log_index) AS hour_sum_usd

            , pid.transaction_hash
            , pid.block_number
            , pid.block_timestamp
            , pid.transaction_index
            , pid.log_index
        FROM prepared_incremental_deltas pid
        ORDER BY 
            pool__id
            , pid.token__id
            , block_number
            , transaction_index
            , log_index
    )

SELECT * FROM final

{{
    config(
        materialized = 'view',
        alias = 'COMPILED_METRICS_DELTAS_LIQUIDITY_POOL_INPUT_TOKEN_BALANCES',
        unique_key='id',
    )
}}

{% set deltas_input_token_balances = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_deltas_input_token_balances') %}

WITH
    incremental_deltas AS (SELECT * FROM {{ deltas_input_token_balances }})

    , final AS (
        SELECT
            id.id
            , id.pool__id
            , id.token__id
            , DATE_TRUNC('hour', id.block_timestamp) AS hour

            -- Delta Calculations
            , id.delta
            , COALESCE(id.delta / POWER(10, token.decimals) * token.price_usd, 0) AS delta_usd
            
            -- Cumulative Sum Delta Calculations
            , SUM(id.delta)  OVER (PARTITION BY id.pool__id, id.token__id ORDER BY id.block_number, id.transaction_index, id.log_index) AS cumulative_sum

            -- Cumulative Sum Delta Calculations (Current Timestamp Price)
            , COALESCE(cumulative_sum / POWER(10, token.decimals) * token.price_usd, 0) AS cumulative_sum_usd_current_timestamp_price

            , id.transaction_hash
            , id.block_number
            , id.block_timestamp
            , id.transaction_index
            , id.log_index
        FROM incremental_deltas id
        LEFT JOIN {{ token_prices_usd_hour }} token 
            ON id.token__id = token.contract_address 
                AND DATE_TRUNC('hour', id.block_timestamp) = token.time
        ORDER BY 
            pool__id
            , token__id
            , block_number
            , transaction_index
            , log_index
    )

SELECT * FROM final

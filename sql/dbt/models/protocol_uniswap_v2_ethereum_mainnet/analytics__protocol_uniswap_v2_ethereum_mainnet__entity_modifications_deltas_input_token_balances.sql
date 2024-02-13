{{
    config(
        materialized = 'view',
        alias = 'ENTITY_MODIFICATIONS_DELTAS_INPUT_TOKEN_BALANCES',
        unique_key='id',
    )
}}

{% set initialized_dex_amm_protocols    = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_dex_amm_protocols') %}
{% set initialized_liquidity_pools      = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_liquidity_pools') %}
{% set initialized_swaps                = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_swaps') %}
{% set initialized_deposits             = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_deposits') %}
{% set initialized_withdraws            = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_withdraws') %}

WITH
    incremental_initialized_swaps       AS (SELECT * FROM {{ initialized_swaps }})
    , incremental_initialized_deposits  AS (SELECT * FROM {{ initialized_deposits }})
    , incremental_initialized_withdraws AS (SELECT * FROM {{ initialized_withdraws }})

    , deposit_withdraws AS (
        SELECT
            id
            
            , protocol__id
            , pool__id

            , input_token_amounts
            , block_number
            , transaction_index
            , transaction_hash
            , log_index
            , block_timestamp
        FROM incremental_initialized_deposits

        UNION ALL 

        SELECT
            iiw.id
            
            , iiw.protocol__id
            , iiw.pool__id

            , JSON_BUILD_OBJECT(
                ilp.input_tokens[1]::VARCHAR, iiw.input_token_amounts -> ilp.input_tokens[1]::VARCHAR, 
                ilp.input_tokens[2]::VARCHAR, iiw.input_token_amounts -> ilp.input_tokens[2]::VARCHAR
            ) AS input_token_amounts

            , iiw.block_number
            , iiw.transaction_index
            , iiw.transaction_hash
            , iiw.log_index
            , iiw.block_timestamp
        FROM incremental_initialized_withdraws iiw
        LEFT JOIN {{ initialized_liquidity_pools }} ilp ON iiw.pool__id = ilp.id
    )

    , flattened_deposit_withdraw_deltas AS (
        SELECT
            dw.id

            , dw.protocol__id
            , dw.pool__id
            , f.key::VARCHAR AS token__id

            , (f.value)::TEXT::FLOAT AS delta

            , dw.block_number
            , dw.transaction_index
            , dw.transaction_hash
            , dw.log_index
            , dw.block_timestamp
        FROM deposit_withdraws dw,
        LATERAL JSON_EACH(input_token_amounts) AS f
    )

    , swap_in_deltas AS (
        SELECT
            id

            , protocol__id
            , pool__id
            , token_in__id AS token__id

            , amount_in AS delta

            , block_number
            , transaction_index
            , transaction_hash
            , log_index
            , block_timestamp
        FROM incremental_initialized_swaps
    )
    
    , swap_out_deltas AS (
        SELECT
            id

            , protocol__id
            , pool__id
            , token_out__id AS token__id

            , -amount_out AS delta

            , block_number
            , transaction_index
            , transaction_hash
            , log_index
            , block_timestamp
        FROM incremental_initialized_swaps
    )

    , final AS (
        SELECT * FROM flattened_deposit_withdraw_deltas
        UNION ALL
        SELECT * FROM swap_in_deltas
        UNION ALL
        SELECT * FROM swap_out_deltas
    )

SELECT * FROM final

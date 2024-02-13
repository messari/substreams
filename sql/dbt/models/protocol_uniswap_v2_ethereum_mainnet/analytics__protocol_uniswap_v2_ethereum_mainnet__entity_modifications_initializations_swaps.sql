{{
    config(
        materialized = 'view',
        alias = 'ENTITY_MODIFICATIONS_INITIALIZATIONS_SWAPS',
        unique_key='id',
    )
}}

{% set initialized_liquidity_pools      = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_liquidity_pools') %}
{% set initialized_dex_amm_protocols    = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_dex_amm_protocols') %}

WITH
    incremental_swap_events AS (
        SELECT 
            evt_tx_hash AS transaction_hash
            , null AS transaction_index
            , evt_index AS log_index
            , evt_address AS contract_address
            , sender AS param_sender
            , amount0_in AS param_amount0_in
            , amount0_out AS param_amount0_out
            , amount1_in AS param_amount1_in
            , amount1_out AS param_amount1_out
            , evt_block_number AS block_number
            , evt_block_time AS block_timestamp
        FROM pools_swap
    )

    , final AS (
        SELECT
            'SWAP-' || ise.transaction_hash || '-' || ise.log_index                                 AS id           
            
            , idap.id                                                                               AS protocol__id
            , ise.contract_address                                                                  AS pool__id
            , ise.param_sender                                                                      AS user__id
            
            , CASE
                WHEN ise.param_amount0_in > 0 THEN ise.param_amount0_in
                ELSE ise.param_amount1_in
            END AS amount_in
            , CASE
                WHEN ise.param_amount0_in > 0 THEN ABS(ise.param_amount1_out)
                ELSE ABS(ise.param_amount0_out)
            END AS amount_out

            , CASE
                WHEN ise.param_amount0_in > 0 THEN ilp.input_tokens[1]::VARCHAR
                ELSE ilp.input_tokens[2]::VARCHAR
            END AS token_in__id
            , CASE
                WHEN ise.param_amount0_in > 0 THEN ilp.input_tokens[2]::VARCHAR
                ELSE ilp.input_tokens[1]::VARCHAR
            END AS token_out__id

            , ise.block_number
            , ise.transaction_index
            , ise.transaction_hash
            , ise.log_index
            , ise.block_timestamp
        FROM {{ initialized_dex_amm_protocols }} idap, incremental_swap_events ise
        LEFT JOIN {{ initialized_liquidity_pools }} ilp ON ise.contract_address = ilp.id
    )

SELECT * FROM final

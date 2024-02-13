{{
    config(
        materialized = 'view',
        alias = 'ENTITY_MODIFICATIONS_INITIALIZATIONS_WITHDRAWS',
        unique_key='id',
    )
}}

{% set initialized_dex_amm_protocols    = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_dex_amm_protocols') %}
{% set initialized_liquidity_pools      = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_liquidity_pools') %}

WITH
    incremental_burn_events AS (
        SELECT 
            evt_tx_hash AS transaction_hash
            , null AS transaction_index
            , evt_index AS log_index
            , evt_address AS contract_address
            , sender AS param_sender
            , amount0 AS param_amount0
            , amount1 AS param_amount1
            , evt_block_number AS block_number
            , evt_block_time AS block_timestamp
        FROM pools_burn
    ), 
    
    incremental_transfer_events AS (
        SELECT 
            evt_tx_hash AS transaction_hash
            , evt_index AS log_index
            , evt_address AS contract_address
            , "from" AS param_from
            , "to" AS param_to
            , value AS param_value
        FROM pools_transfer
    )

    , final AS (
        SELECT
            'WITHDRAW-' || ibe.transaction_hash || '-' || ibe.log_index AS id

            , idap.id                                               AS protocol__id
            , ibe.contract_address                                  AS pool__id
            , ibe.param_sender                                      AS user__id

            , JSON_BUILD_OBJECT(
                ilp.input_tokens[1]::VARCHAR, ibe.param_amount0, 
                ilp.input_tokens[2]::VARCHAR, ibe.param_amount1
            ) AS input_token_amounts

            , t.param_value                                         AS output_token_amount

            , ibe.block_number
            , ibe.transaction_index
            , ibe.transaction_hash
            , ibe.log_index
            , ibe.block_timestamp
        FROM {{ initialized_dex_amm_protocols }} idap, incremental_burn_events ibe
        LEFT JOIN {{ initialized_liquidity_pools }} ilp ON ibe.contract_address = ilp.id
        INNER JOIN incremental_transfer_events t
            ON ibe.transaction_hash = t.transaction_hash
                AND ibe.log_index > t.log_index
                AND ibe.contract_address = t.contract_address
                AND t.param_to = '0000000000000000000000000000000000000000'
    )

SELECT * FROM final

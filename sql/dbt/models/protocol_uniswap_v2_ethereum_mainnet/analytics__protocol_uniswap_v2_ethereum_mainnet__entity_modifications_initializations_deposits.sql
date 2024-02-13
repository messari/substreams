{{
    config(
        materialized = 'view',
        alias = 'ENTITY_MODIFICATIONS_INITIALIZATIONS_DEPOSITS',
        unique_key='id',
    )
}}

{% set initialized_dex_amm_protocols    = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_dex_amm_protocols') %}
{% set initialized_liquidity_pools      = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_liquidity_pools') %}

WITH
    incremental_mint_events AS (
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
        FROM pools_mint
    ), 

    incremental_transfer_events AS (
        SELECT 
            evt_tx_hash AS transaction_hash
            , evt_index AS log_index
            , "from" AS param_from
            , "to" AS param_to
            , value AS param_value
        FROM pools_transfer
    )

    , final AS (
        SELECT
            'DEPOSIT-' || ime.transaction_hash || '-' || ime.log_index AS id
            
            , idap.id                                               AS protocol__id
            , ime.contract_address                                  AS pool__id
            , ime.param_sender                                      AS user__id

            , JSON_BUILD_OBJECT(
                ilp.input_tokens[1]::VARCHAR, ime.param_amount0, 
                ilp.input_tokens[2]::VARCHAR, ime.param_amount1
            ) AS input_token_amounts

            , t.param_value                                         AS output_token_amount
            
            , ime.block_number
            , ime.transaction_index
            , ime.transaction_hash
            , ime.log_index
            , ime.block_timestamp
        FROM {{ initialized_dex_amm_protocols }} idap, incremental_mint_events ime
        LEFT JOIN {{ initialized_liquidity_pools }} ilp ON ime.contract_address = ilp.id
        INNER JOIN incremental_transfer_events t
            ON ime.transaction_hash = t.transaction_hash
                AND ime.log_index = t.log_index + 2
                AND t.param_from = '0000000000000000000000000000000000000000'
    )

SELECT * FROM final

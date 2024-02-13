{{
    config(
        materialized = 'view',
        alias = 'ENTITY_MODIFICATIONS_INITIALIZATIONS_LIQUIDITY_POOLS',
        unique_key='id',
    )
}}

{% set initialized_dex_amm_protocols    = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_dex_amm_protocols') %}

WITH
    incremental_pool_created_events AS (
        SELECT * FROM factory_pair_created
    )
    
    , name_and_symbol AS (
        SELECT
            ipce.pair                                                                           AS pool
            , 'token0' || '/' || 'token1'                                                       AS _symbol
            , 'Uniswap V2 Pool: null'                                                           AS name
        FROM incremental_pool_created_events ipce
    )

    , final AS (
        SELECT 
            ipce.pair AS id
            
            , idap.id                                       AS protocol__id

            , nas.name
            , nas._symbol                                   AS symbol
            , ARRAY[token0, token1]                         AS input_tokens

            , evt_block_number as block_number
            , evt_tx_hash as transaction_hash
            , null as transaction_index
            , evt_index as log_index
            , evt_block_time as block_timestamp
        FROM {{ initialized_dex_amm_protocols }} idap, incremental_pool_created_events ipce
        LEFT JOIN name_and_symbol nas ON ipce.pair = nas.pool
    )

SELECT * FROM final

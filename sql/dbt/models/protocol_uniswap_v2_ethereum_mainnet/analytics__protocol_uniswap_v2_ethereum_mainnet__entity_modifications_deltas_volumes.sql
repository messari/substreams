{{
    config(
        materialized = 'view',
        alias = 'ENTITY_MODIFICATIONS_DELTAS_VOLUMES',
        unique_key='id',
    )
}}

{% set initialized_swaps = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_swaps') %}

WITH
    final AS (
        SELECT
            id

            , amount_in     AS delta_in
            , amount_out    AS delta_out

            , protocol__id
            , pool__id
            , token_in__id
            , token_out__id
            
            , block_number
            , transaction_index
            , transaction_hash
            , log_index
            , block_timestamp
        FROM {{ initialized_swaps }} iis
    )

SELECT * FROM final

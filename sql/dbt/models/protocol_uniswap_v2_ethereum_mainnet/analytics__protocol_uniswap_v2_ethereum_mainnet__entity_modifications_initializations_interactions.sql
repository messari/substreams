{# {{
    config(
        materialized = 'view',
        alias = 'ENTITY_MODIFICATIONS_INITIALIZATIONS_INTERACTIONS',
        unique_key='id',
    )
}}

{% set initialized_swaps                = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_swaps') %}
{% set initialized_deposits             = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_deposits') %}
{% set initialized_withdraws            = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_withdraws') %}
{% set initialized_dex_amm_protocols    = ref('analytics__protocol_uniswap_v2_ethereum_mainnet__entity_modifications_initializations_dex_amm_protocols') %}

WITH 
    user_interactions AS (
        SELECT
            id
            , user__id
            , pool__id
            , protocol__id
            , 'DEX_DEPOSIT' AS type
            , block_number
            , transaction_hash
            , transaction_index
            , log_index
            , block_timestamp
            , _load_timestamp_utc
            , _last_run_timestamp_utc
        FROM {{ initialized_deposits }}
        UNION ALL
        SELECT
            id
            , user__id
            , pool__id
            , protocol__id
            , 'DEX_WITHDRAW' AS type
            , block_number
            , transaction_hash
            , transaction_index
            , log_index
            , block_timestamp
            , _load_timestamp_utc
            , _last_run_timestamp_utc
        FROM {{ initialized_withdraws }}   
        UNION ALL
        SELECT
            id
            , user__id
            , pool__id
            , protocol__id
            , 'DEX_SWAP' AS type
            , block_number
            , transaction_hash
            , transaction_index
            , log_index
            , block_timestamp
            , _load_timestamp_utc
            , _last_run_timestamp_utc
        FROM {{ initialized_swaps }}
    )

    , final AS (
        SELECT
            ui.id

            , ui.user__id
            , ui.pool__id
            , ui.protocol__id

            , ui.type

            , ui.block_number
            , ui.transaction_hash
            , ui.transaction_index
            , ui.log_index
            , ui.block_timestamp
            , ui._load_timestamp_utc
            , ui._last_run_timestamp_utc
        FROM user_interactions ui
    )

SELECT * FROM final #}
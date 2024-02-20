{{
    config(
        materialized = 'table',
        alias = 'INPUT_TOKEN_BALANCES_FLATTENED'
    )
}}

{% set TOKEN_PRICES = source('ADHOC', 'PRICE_USD_1H') %}
{% set SWAPS = ref('analytics__protocol_balancer_v2_ethereum__swaps') %}
{% set ACTIONS = ref('analytics__protocol_balancer_v2_ethereum__actions') %}
{% set INPUT_TOKENS = ref('analytics__protocol_balnacer_v2_ethereum__input_tokens') %}
{% set ERC20_TOKENS = ref('analytics__protocol_balancer_v2_ethereum__erc20_tokens') %}
{% set DEPOSITS_FLATTENED = ref('analytics__protocol_balancer_v2_ethereum__deposits_flattened') %}
{% set WITHDRAWS_FLATTENED = ref('analytics__protocol_balancer_v2_ethereum__withdraws_flattened') %}

WITH BALANCES_FLATTENED AS (
    SELECT 
        TRANSACTION_HASH
        , BLOCK_NUMBER
        , LOG_INDEX
        , POOL__ID
        , INPUT_TOKEN__ID AS TOKEN__ID
        , INPUT_TOKEN_BALANCE
    FROM {{ DEPOSITS_FLATTENED }}
    UNION 
    SELECT 
        TRANSACTION_HASH
        , BLOCK_NUMBER
        , LOG_INDEX
        , POOL__ID
        , INPUT_TOKEN__ID AS TOKEN__ID
        , INPUT_TOKEN_BALANCE
    FROM {{ WITHDRAWS_FLATTENED }}
    UNION
    SELECT 
        HASH AS TRANSACTION_HASH
        , BLOCK_NUMBER
        , LOG_INDEX
        , POOL__ID
        , TOKEN_IN__ID AS TOKEN__ID
        , TOKEN_IN_BALANCE AS INPUT_TOKEN_BALANCE
    FROM {{ SWAPS }}
    UNION
    SELECT 
        HASH AS TRANSACTION_HASH
        , BLOCK_NUMBER
        , LOG_INDEX
        , POOL__ID
        , TOKEN_OUT__ID AS TOKEN__ID
        , TOKEN_OUT_BALANCE AS INPUT_TOKEN_BALANCE
    FROM {{ SWAPS }}
)

, RANKED_BALANCES AS (
    SELECT 
        TRANSACTION_HASH
        , BLOCK_NUMBER
        , LOG_INDEX
        , POOL__ID
        , TOKEN__ID
        , INPUT_TOKEN_BALANCE
        , ROW_NUMBER() OVER (PARTITION BY POOL__ID, TOKEN__ID ORDER BY BLOCK_NUMBER, LOG_INDEX ASC) AS RANK
    FROM BALANCES_FLATTENED
)

, ALL_TXNS AS (
    SELECT 
        T.POOL__ID
        , T.LOG_INDEX
        , T.TIMESTAMP
        , T.BLOCK_NUMBER
        , T.HASH AS TRANSACTION_HASH
    FROM {{ ACTIONS }} T
)

, FLATTENED_TXNS AS (
    SELECT
        T.TRANSACTION_HASH
        , T.BLOCK_NUMBER
        , T.LOG_INDEX
        , T.TIMESTAMP
        , T.POOL__ID
        , IT.TOKEN_IDX
        , IT.INPUT_TOKEN AS TOKEN__ID
        , ROW_NUMBER() OVER (PARTITION BY T.POOL__ID, TOKEN__ID ORDER BY T.BLOCK_NUMBER, T.LOG_INDEX ASC) AS RANK
    FROM ALL_TXNS T
    LEFT JOIN {{ INPUT_TOKENS }} IT
        ON IT.POOL__ID = T.POOL__ID
)

, FINAL AS (
    SELECT 
        T.POOL__ID
        , T.TOKEN__ID
        , T.TOKEN_IDX
        , DATE_TRUNC('hour', T.TIMESTAMP) AS HOUR
        , T.TIMESTAMP
        , T.BLOCK_NUMBER
        , COALESCE(
            B.INPUT_TOKEN_BALANCE, 
            LAG(B.INPUT_TOKEN_BALANCE) IGNORE NULLS OVER (PARTITION BY T.TOKEN_IDX, T.POOL__ID ORDER BY T.RANK)
        ) AS INPUT_TOKEN_BALANCE
        , INPUT_TOKEN_BALANCE / POWER(10, ET.DECIMALS) * PR.PRICE_USD AS INPUT_TOKEN_BALANCE_USD
    FROM FLATTENED_TXNS T
    LEFT JOIN RANKED_BALANCES B
        ON T.POOL__ID = B.POOL__ID
            AND T.TOKEN__ID = B.TOKEN__ID
            AND B.RANK = (
                SELECT
                    MAX(Z.RANK)
                FROM RANKED_BALANCES Z
                WHERE Z.TOKEN__ID = T.TOKEN__ID
                    AND Z.POOL__ID = T.POOL__ID
                    AND Z.BLOCK_NUMBER = T.BLOCK_NUMBER
            )
    LEFT JOIN {{ ERC20_TOKENS }} ET
        ON ET.ADDRESS = T.TOKEN__ID 
    LEFT JOIN {{ TOKEN_PRICES }} PR
        ON PR.CONTRACT_ADDRESS = T.TOKEN__ID
            AND PR.TIME = DATE_TRUNC('hour', T.TIMESTAMP)
            AND PR.NETWORK = 'Ethereum'
    
    QUALIFY ROW_NUMBER() OVER (PARTITION BY T.POOL__ID, T.TOKEN__ID, HOUR ORDER BY T.BLOCK_NUMBER DESC, T.LOG_INDEX DESC) = 1
)

SELECT * FROM FINAL
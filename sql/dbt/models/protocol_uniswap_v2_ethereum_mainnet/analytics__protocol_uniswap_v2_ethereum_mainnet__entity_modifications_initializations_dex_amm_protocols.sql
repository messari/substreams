{{
    config(
        materialized = 'view',
        alias = 'ENTITY_MODIFICATIONS_INITIALIZATIONS_DEX_AMM_PROTOCOLS',
        unique_key='id',
    )
}}

WITH
    final as (
        SELECT 
            '0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f'    AS id
            , 'Uniswap V2'                                  AS name
            , 'uniswap_v2'                                  AS slug
            , '1.3.2'                                       AS schema_version
            , '1.0.0'                                       AS subgraph_version
            , '1.0.0'                                       AS methodology_version
            , 'MAINNET'                                     AS network
            , 'EXCHANGE'                                    AS type
    )

SELECT * FROM final

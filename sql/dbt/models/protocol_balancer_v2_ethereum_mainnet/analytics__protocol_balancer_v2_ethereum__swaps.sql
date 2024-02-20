{{
    config(
        materialized = 'table',
        alias = 'BALANCER_V2_SWAPS'
    )
}}

WITH raw_swaps AS (
    SELECT
        ve.evt_tx_hash AS transaction_hash
        , ve.evt_index AS log_index
        , ve.evt_block_number AS block_number
        , ve.evt_block_time AS block_timestamp
        , ve.pool_id AS pool__id_bytes
        , SUBSTR(ve.pool_id, 1, 40) AS pool__id
        , ve.amount_in
        , ve.amount_out
        , ve.token_in AS token__in_id
        , ve.token_out AS token__out_id
    FROM vault_swap ve
)

, final AS (
    SELECT
        rs.transaction_hash as HASH
        , rs.block_number
        , rs.block_timestamp
        , rs.log_index
        , 'SWAP' || '-' || rs.transaction_hash || '-' || rs.log_index AS id
        , rs.pool__id
        , rs.pool__id AS "to"
        , null AS "from"
        , rs.token__in_id
        , rs.token__out_id
        , rs.amount_in
        , rs.amount_out
        , array_position(vr.tokens, rs.token__in_id) AS token_in_idx
        , array_position(vr.tokens, rs.token__out_id) AS token_out_idx

        , null AS amount_in_usd
        , null AS amount_out_usd
        , null AS volume_usd
    FROM raw_swaps rs
    LEFT JOIN vault_tokens_registered vr
        ON vr.pool_id = rs.pool__id_bytes
)

SELECT * FROM final

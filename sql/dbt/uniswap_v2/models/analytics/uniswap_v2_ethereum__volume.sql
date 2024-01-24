{% set swaps = ref("uniswap_v2_ethereum__swaps") %}
{% set prices = ref("uniswap_v2_ethereum__token_prices") %}

with swap_volume as (
    select 
        s.transaction_hash
        , s.log_index
        , s.block_number
        , s.block_time
        , s.token_in__id
        , (s.amount_in * p.token_in__price) / pow(10, p.token_in__decimals) as amount_in_usd
        , s.token_out__id
        , (s.amount_out * p.token_out__price) / pow(10, p.token_out__decimals) as amount_out_usd
    from {{ swaps }} s
    left join {{ prices }} p
    on s.transaction_hash = p.transaction_hash
        and s.log_index = p.log_index
)

, final as (
    select 
        transaction_hash
        , log_index
        , block_number
        , block_time
        , amount_in_usd
        , amount_out_usd
        , (amount_in_usd + amount_out_usd) / 2 as volume
    from swap_volume
)

select * from final
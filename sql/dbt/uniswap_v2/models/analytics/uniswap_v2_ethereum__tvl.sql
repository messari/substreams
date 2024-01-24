{% set sync = ref("uniswap_v2_ethereum__sync") %}
{% set prices = ref("uniswap_v2_ethereum__token_prices") %}

with tvl as (
    select 
        s.transaction_hash
        , s.log_index
        , s.block_number
        , s.block_time
        , case 
            when sw.token_in__id = '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2' then (s.reserve0 / pow(10, 18)) * sw.token_in__price
            when sw.token_out__id = '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2' then (s.reserve0 / pow(10, 18)) * sw.token_out__price
        end as token0_balance_usd
        , s.reserve1 / pow(10, 6) as token1_balance_usd
    from {{ sync }} s
    left join {{ prices }} sw
    on s.transaction_hash = sw.transaction_hash
        and s.log_index = sw.log_index - 1
)

, final as (
    select 
        transaction_hash
        , log_index
        , block_number
        , block_time
        , token0_balance_usd
        , token1_balance_usd
        , (token0_balance_usd + token1_balance_usd) as tvl
    from tvl
)

select * from final
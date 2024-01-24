{% set pool = ref("uniswap_v2_ethereum__pool") %}

with final as (
    select 
        evt_tx_hash as transaction_hash
        , evt_index as log_index
        , evt_block_time as block_time
        , evt_block_number as block_number
        , case 
            when amount0_in > 0 then p.token0__id else p.token1__id
        end as token_in__id
        , case 
            when amount0_in > 0 then amount0_in else amount1_in
        end as amount_in
        , case 
            when amount0_in > 0 then p.token1__id else p.token0__id
        end as token_out__id
        , case 
            when amount0_in > 0 then amount1_out else amount0_out
        end as amount_out
    from swap
    cross join {{ pool }} p
)

select * from final

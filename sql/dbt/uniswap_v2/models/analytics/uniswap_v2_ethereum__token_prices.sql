{% set swaps = ref("uniswap_v2_ethereum__swaps") %}

with final as (
    select 
        transaction_hash
        , log_index
        , amount_in
        , amount_out
        , token_in__id
        , case 
            when token_in__id = '0xdAC17F958D2ee523a2206206994597C13D831ec7' then 6 else 18
        end as token_in__decimals
        , case 
            when token_in__id = '0xdAC17F958D2ee523a2206206994597C13D831ec7' then 1 else (amount_out / pow(10, 6)) / (amount_in / pow(10, 18))
        end as token_in__price
        , token_out__id
        , case 
            when token_out__id = '0xdAC17F958D2ee523a2206206994597C13D831ec7' then 6 else 18
        end as token_out__decimals
        , case 
            when token_out__id = '0xdAC17F958D2ee523a2206206994597C13D831ec7' then 1 else (amount_in / pow(10, 6)) / (amount_out / pow(10, 18))
        end as token_out__price
    from {{ swaps }}
)

select * from final
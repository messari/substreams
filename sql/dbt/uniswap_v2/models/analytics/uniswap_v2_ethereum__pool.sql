
with final as (
    select 
        '0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852' as pool_address 
        , '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2' as token0__id 
        , 'Wrapped ETH' as token0__name
        , 'WETH' as token0__symbol
        , '18' as token0__decimals
        , '0xdAC17F958D2ee523a2206206994597C13D831ec7' as token1__id 
        , 'Tether USD' as token1__name
        , 'USDT' as token1__symbol
        , '6' as token1__decimals
)

select * from final

with final as (
    select 
        evt_tx_hash as transaction_hash
        , evt_index as log_index
        , evt_block_time as block_time
        , evt_block_number as block_number
        , amount0
        , amount1
    from mint
)

select * from final
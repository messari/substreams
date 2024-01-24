
with final as (
    select 
        evt_tx_hash as transaction_hash
        , evt_index as log_index
        , evt_block_time as block_time
        , evt_block_number as block_number
        , amount0_in - amount0_out as amount0
        , amount1_in - amount1_out as amount1
    from swap
)

select * from final

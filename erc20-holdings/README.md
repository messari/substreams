# ERC20 Holdings

This substream will find all of the ERC20 tokens on ethereum with metadata. In addition it will map transfers. Balance can be derived from this. Then the `store_balance_usd` module combines chainlink data to store the prices as well.

## Notes

- The `map_block_to_erc20_contracts` module should be a `store` module since multiple other modules will want to use it as input to get ERC20 token metadata.
- `map_block_to_erc20_contracts` gets ERC20 metadata some of the time. It should grab all ERC20 contracts, however the metadata is not always populated. I am not quite sure why this is.

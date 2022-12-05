# Messari Substreams

## Pre-requisites

### Getting started with Rust

- [Half hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Tiago's Rust first steps](https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/)
- [Rust By Example](https://github.com/rust-lang/rust-by-example)
- [Rust By Practice](https://github.com/sunface/rust-by-practice)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)

### Getting started with Substreams

- [YouTube: Introducing Substreams](https://www.youtube.com/watch?v=qWxffTKpciU)
- [Developer Docs](https://substreams.streamingfast.io/)

### Sample Substreams

- [Subtreams Template (NFT)](https://github.com/streamingfast/substreams-template)
- [Uniswap v3](https://github.com/streamingfast/substreams-uniswap-v3)
- [Compound v2](https://github.com/0xbe1/compoundv2-substreams)

### Helpers

- [Keccak 256 Encoder](https://emn178.github.io/online-tools/keccak_256.html)

## Development Status

🔨 = In progress.  
🛠 = Feature complete. Additional testing required.  
✅ = Production-ready.

| Substream        |  Status | Description                                |
|------------------|  :------: |--------------------------------------------|
| Ethereum Network | 🔨 | Network-level metrics and statistics       |
| ETH Balance      | 🛠 | ETH balance for every Ethereum address     |
| ERC20 Holdings   | 🛠 | ERC20 balance for every Ethereum address   |
| ERC20 Price      | 🛠 | Pricing module for ERC20 tokens            |
| SPL Holdings     |  | SPL token balance for every Solana address |
| ENS Look Up      | 🔨 | ENS records for lookup and reverse lookup  |
| Uniswap v2       | 🔨 | Substreams for Uniswap v2                  |
| Compound v2      | 🔨 | Substreams for Compound v2                 |

## Workflow

### Messari Command line interface

- To install run: `make install-cli`
- Two commands are currently available:
    - `messari init`
    -  `messari add abi`
- Use `--help` flag for details around providing args in the command line
- If any args are left blank the CLI will ask you for the necessary information when needed

###  Logging

- [Rust Logging](https://docs.rs/log/0.4.14/log/)
- Logging can be done using the standard `log` crate, or using the `slog` crate.
- **Note** that `substreams` does not cache the logs. If you want to see logs, you may need to make a change to your substreams code, so that a new binary will be generated - otherwise, this substream will just stream the cached data from the first execution without the logs.


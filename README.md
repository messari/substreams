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

| Substream        | Status | Description                                |
| ---------------- | :----: | ------------------------------------------ |
| Ethereum Network |   🔨   | Network-level metrics and statistics       |
| ETH Balance      |   🛠    | ETH balance for every Ethereum address     |
| ERC20 Holdings   |   🛠    | ERC20 balance for every Ethereum address   |
| ERC20 Price      |   🛠    | Pricing module for ERC20 tokens            |
| SPL Holdings     |        | SPL token balance for every Solana address |
| ENS Look Up      |   🔨   | ENS records for lookup and reverse lookup  |
| Uniswap v2       |   🔨   | Substreams for Uniswap v2                  |
| Compound v2      |   🔨   | Substreams for Compound v2                 |

## Workflow

### Messari Command line interface

- To install run: `make install-cli`
- Two commands are currently available:
  - `messari init`
  - `messari add abi`
- Use `--help` flag for details around providing args in the command line
- If any args are left blank the CLI will ask you for the necessary information when needed

### Logging

- [Rust Logging](https://docs.rs/log/0.4.14/log/)
- Logging can be done using the standard `log` crate, or using the `slog` crate.
- **Note** that `substreams` does not cache the logs. If you want to see logs, you may need to make a change to your substreams code, so that a new binary will be generated - otherwise, this substream will just stream the cached data from the first execution without the logs.

### Contributing to this repo

#### Versioning

> TLDR: Every substream modified by a PR needs to get its manifest version updated. Otherwise the CI will fail. You can do this with automatically with `npm run versions:update:git`

In this repository, multiple different substreams coexist at the same time, together with some utility libraries. Any given substream might depend on another one inside the repo, and a change to a dependency will likely result in a change in the output of all dependants.

When a substream gets updated, we want to update its version in the manifest, so that we know which substreams need to be redeployed as a result of some change. Since this is cumbersome to track manually, specially the more substreams that get added to the repo, we have a set of utility scripts to do this for you. They are located in `./scripts`. These same ones are used by the CI, to validate that all necessary versions have been updated. When you are done with your changes and are ready to open a Pull Request, you can run:

```
# (make sure to have your local master up to date)
$ npm run versions:update:git
```

This will look at all your file changes comparing them to `master`, and based on these files will determine which versions need to be updated. It will updated all `substreams.yaml`

#### Substreams Config

The same substream might be reused for different protocols and networks. Same functionality, but different initial parameters (start blocks, params, tracked addresses, network ...).

For standalone substreams, that are intended to be run as a whole (not only as a dependency of another), we define all the possible configurations we want to run them with.
These live in `./config/params.json`. The schema for this config file is defined in `./config/schemas/params.schema.json`. It is something like this:

```
[
  {
    "name": "aave-v2",       # name to uniquely identify a substream
    "path": "../aave-v2",    # path, relative to the config file, where the substrean lives (since the repo is not enforcing any particular org structure)
    "outputModules": ["map_output"],  # list of modules we want to output, to a sink. As of now we only support 1, but can easily be expanded to more.
    "subgraphModule": "map_entity_changes", # (optional) if this substream powers a subgraph, what is the module which should be used for that
    "deployments": [         # list deployment specific parameters
      {
        "name": "aave-v2-ethereum", # name to uniquely identify a deployment
        "network": "mainnet",
        "params": {                 # map of {moduleName} -> {paramValue}, for modules that have params inputs
          "store_observed_contracts": "0x357d51124f59836ded84c8a1730d72b749d8bc23;0x8dff5e27ea6b7ac08ebfdf9eb090f32ee9a30fcf;0x26db2b833021583566323e3b8985999981b9f1f3"
        },
        "startBlocks": {            # map of {moduleName} -> {starBlockNumber}, since each deployment might have different start blocks
          "store_observed_contracts": 12486774,
          "map_atoken_supply_changes": 12486774,
          "map_atoken_balances": 12486774,
          "map_raw_events": 12486774,
          "map_output": 12486774
        }
      }
    ]
  },
  ...
]
```

> **NOTE:** In a way, substreams present in this file might be considered production ready.

#### Subgraphs

In a similar way as standalone config, subgraphs powered by substreams in this repo are defined in `./config/subgraphs.json`.

As of now, this acts as a registry. Manually maintained. But eventually it could be automated so that changes to substreams powering the subgraphs defined in the repo are re-deployed.

The schema for this file can be found in `./config/schemas/subgraphs.schema.json`.

```{
  "eth-supply/eth-supply-ethereum": { # this should be of the form {substreamName}/{deploymentName}, matching the names in `params.json` for the substream powering this subgraph.
    "services": {  # this shares structure with https://github.com/messari/subgraphs/blob/master/deployment/deployment.json
      "hosted-service": {
        "slug": "substream-eth-supply-ethereum",
        "query-id": "substream-eth-supply-ethereum"
      }
    }
  },
  ...
}
```

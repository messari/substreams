# CompoundV2 Substreams

Ongoing effort to index Compound V2 using substreams.

## Architecture

```mermaid
graph TD;
  map_accrue_interest[map: map_accrue_interest]
  sf.ethereum.type.v1.Block[source: sf.ethereum.type.v1.Block] --> map_accrue_interest
  map_mint[map: map_mint]
  sf.ethereum.type.v1.Block[source: sf.ethereum.type.v1.Block] --> map_mint
  store_token --> map_mint
  store_price --> map_mint
  map_market_listed[map: map_market_listed]
  sf.ethereum.type.v1.Block[source: sf.ethereum.type.v1.Block] --> map_market_listed
  map_market_totals[map: map_market_totals]
  map_accrue_interest --> map_market_totals
  store_token --> map_market_totals
  store_price --> map_market_totals
  map_market_revenue_delta[map: map_market_revenue_delta]
  map_accrue_interest --> map_market_revenue_delta
  store_market_reserve_factor --> map_market_revenue_delta
  store_price --> map_market_revenue_delta
  store_token --> map_market_revenue_delta
  store_token[store: store_token]
  map_market_listed --> store_token
  store_market_reserve_factor[store: store_market_reserve_factor]
  sf.ethereum.type.v1.Block[source: sf.ethereum.type.v1.Block] --> store_market_reserve_factor
  store_market_count[store: store_market_count]
  map_market_listed --> store_market_count
  store_mint_count[store: store_mint_count]
  map_mint --> store_mint_count
  store_oracle[store: store_oracle]
  sf.ethereum.type.v1.Block[source: sf.ethereum.type.v1.Block] --> store_oracle
  store_price[store: store_price]
  map_accrue_interest --> store_price
  store_oracle --> store_price
  store_token --> store_price
  store_market_listed[store: store_market_listed]
  map_market_listed --> store_market_listed
  store_mint[store: store_mint]
  map_mint --> store_mint
  store_market_totals[store: store_market_totals]
  map_market_totals --> store_market_totals
  store_protocol_totals[store: store_protocol_totals]
  map_market_totals --> store_protocol_totals
  store_market_listed --> store_protocol_totals
  store_market_totals --> store_protocol_totals
  store_revenue[store: store_revenue]
  map_market_revenue_delta --> store_revenue
```

## Quick Start

### Install

Run `go install ./cmd/substreams` under `substreams` repo `develop` branch to get the latest `substreams` cli.

Run `brew install bufbuild/buf/buf` to install `buf`.

### Build

Generate src/pb

```bash
make codegen
```

Build

```bash
make build
```

### Run

```bash
sftoken
substreams run -e api-dev.streamingfast.io:443 substreams.yaml map_market_listed,store_market --start-block 7710778 --stop-block +10
```

## Implemented Schema

LendingProtocol
- oracle
- totalPoolCount
- totalValueLockedUSD
- totalBorrowBalanceUSD
- cumulativeTotalRevenueUSD
- cumulativeProtocolSideRevenueUSD
- cumulativeSupplySideRevenueUSD

Market
- inputToken
- outputToken
- reserveFactor
- totalValueLockedUSD
- totalBorrowBalanceUSD
- cumulativeTotalRevenueUSD
- cumulativeProtocolSideRevenueUSD
- cumulativeSupplySideRevenueUSD

Token
- address
- name
- symbol
- decimals
- lastPriceUSD

UsageMetricsDailySnapshot
- dailyDepositCount

MarketDailySnapshot
- dailyTotalRevenueUSD
- dailyProtocolSideRevenueUSD
- dailySupplySideRevenueUSD

FinancialsDailySnapshot
- dailyTotalRevenueUSD
- dailyProtocolSideRevenueUSD
- dailySupplySideRevenueUSD

## Troubleshooting

Running `store_*` module could produce the below error. Just ignore it and retry.

```
Error: rpc error: code = Unknown desc = error building pipeline: synchronizing stores: from worker: calling back scheduler: squashing: merging partials: initializing next partial store "store_transfers": storage file 0012289000-0012288000.partial: not found
```

#[rustfmt::skip]
mod abi;
#[rustfmt::skip]
mod pb;
mod rpc;
mod utils;

use crate::utils::{exponent_to_big_decimal, MANTISSA_FACTOR};
use bigdecimal::{BigDecimal, Zero};
use pb::compound;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use substreams::{proto, store, Hex};
use substreams_ethereum::NULL_ADDRESS;
use substreams_ethereum::{pb::eth as ethpb, Event as EventTrait};

#[substreams::handlers::map]
fn map_accrue_interest(
    blk: ethpb::v1::Block,
) -> Result<compound::AccrueInterestList, substreams::errors::Error> {
    let mut accrue_interest_list: Vec<compound::AccrueInterest> = vec![];
    for log in blk.logs() {
        // TODO: there are 2 versions of AccrueInterest events, 3-args (early blocks) vs 4-args
        if let Some(accrue_interest) = abi::ctoken::events::AccrueInterest::match_and_decode(log) {
            accrue_interest_list.push(compound::AccrueInterest {
                interest_accumulated: accrue_interest.interest_accumulated.to_string(),
                borrow_index: accrue_interest.borrow_index.to_string(),
                total_borrows: accrue_interest.total_borrows.to_string(),
                address: log.log.address.clone(),
                block_number: blk.number,
                timestamp: blk
                    .header
                    .as_ref()
                    .unwrap()
                    .timestamp
                    .as_ref()
                    .unwrap()
                    .seconds,
            })
        }
    }

    Ok(compound::AccrueInterestList {
        accrue_interest_list,
    })
}

#[substreams::handlers::map]
fn map_mint(
    blk: ethpb::v1::Block,
    store_token: store::StoreGet,
    store_price: store::StoreGet,
) -> Result<compound::MintList, substreams::errors::Error> {
    let mut mint_list = compound::MintList { mint_list: vec![] };
    for trx in blk.transaction_traces {
        for log in trx.receipt.unwrap().logs.iter() {
            let market_address = &log.address;
            let mint_event_res = abi::ctoken::events::Mint::match_and_decode(log);
            let underlying_price_res = store_price.get_last(&format!(
                "market:{}:underlying:price",
                Hex::encode(market_address)
            ));
            let underlying_res = store_token.get_last(&format!(
                "market:{}:underlying",
                Hex::encode(market_address)
            ));
            if let (Some(mint_event), Some(underlying_token), Some(underlying_price)) =
                (mint_event_res, underlying_res, underlying_price_res)
            {
                let price = utils::string_to_bigdecimal(underlying_price.as_ref());
                let underlying_token: compound::Token = proto::decode(&underlying_token).unwrap();
                let mint = compound::Mint {
                    id: format!("{}-{}", Hex::encode(&trx.hash), log.index).into_bytes(),
                    timestamp: blk
                        .header
                        .as_ref()
                        .unwrap()
                        .timestamp
                        .as_ref()
                        .unwrap()
                        .seconds,
                    minter: mint_event.minter,
                    mint_amount: mint_event.mint_amount.to_string(),
                    mint_tokens: mint_event.mint_tokens.to_string(),
                    mint_amount_usd: BigDecimal::from_str(
                        mint_event.mint_amount.to_string().as_str(),
                    )
                    .unwrap()
                    .div(utils::exponent_to_big_decimal(underlying_token.decimals))
                    .mul(price)
                    .to_string(),
                };
                mint_list.mint_list.push(mint);
            }
        }
    }
    Ok(mint_list)
}

#[substreams::handlers::map]
fn map_market_listed(
    blk: ethpb::v1::Block,
) -> Result<compound::MarketListedList, substreams::errors::Error> {
    let mut market_listed_list = compound::MarketListedList {
        market_listed_list: vec![],
    };
    for log in blk.logs() {
        if let Some(market_listed) = abi::comptroller::events::MarketListed::match_and_decode(log) {
            market_listed_list
                .market_listed_list
                .push(compound::MarketListed {
                    ctoken: market_listed.c_token,
                });
        }
    }
    Ok(market_listed_list)
}

#[substreams::handlers::map]
fn map_market_totals(
    accrue_interest_list: compound::AccrueInterestList,
    store_token: store::StoreGet,
    store_price: store::StoreGet,
) -> Result<compound::MarketTotalsList, substreams::errors::Error> {
    let mut market_totals_list = compound::MarketTotalsList {
        market_totals_list: vec![],
    };
    for accrue_interest in accrue_interest_list.accrue_interest_list {
        let market_address = accrue_interest.address;
        let underlying_res: Option<compound::Token> = store_token
            .get_last(&format!(
                "market:{}:underlying",
                Hex::encode(&market_address)
            ))
            .map(|x| proto::decode(&x).unwrap());
        let underlying_price_res = store_price
            .get_last(&format!(
                "market:{}:underlying:price",
                Hex::encode(&market_address)
            ))
            .map(|x| utils::string_to_bigdecimal(x.as_ref()));
        let ctoken_supply_res = rpc::fetch(rpc::RpcCallParams {
            to: market_address.clone(),
            method: "totalSupply()".to_string(),
            args: vec![],
        })
        .map(|x| utils::bytes_to_bigdecimal(x.as_ref()));
        let ctoken_exchange_rate_res = rpc::fetch(rpc::RpcCallParams {
            to: market_address.clone(),
            method: "exchangeRateStored()".to_string(),
            args: vec![],
        })
        .map(|x| utils::bytes_to_bigdecimal(x.as_ref()));
        let total_borrows_mantissa =
            BigDecimal::from_str(accrue_interest.total_borrows.as_str()).unwrap();
        if let (
            Some(underlying),
            Some(underlying_price),
            Ok(ctoken_supply),
            Ok(ctoken_exchange_rate),
        ) = (
            underlying_res,
            underlying_price_res,
            ctoken_supply_res,
            ctoken_exchange_rate_res,
        ) {
            let total_value_locked = ctoken_supply
                .mul(ctoken_exchange_rate)
                .div(exponent_to_big_decimal(
                    utils::MANTISSA_FACTOR + underlying.decimals,
                ))
                .mul(underlying_price.clone());
            let total_borrows = total_borrows_mantissa
                .div(exponent_to_big_decimal(underlying.decimals))
                .mul(underlying_price);
            // TODO: add exchange rate, input token balance, output token supply
            let market_totals = compound::MarketTotals {
                market: market_address,
                total_value_locked: total_value_locked.to_string(),
                total_borrows: total_borrows.to_string(),
            };
            market_totals_list.market_totals_list.push(market_totals);
        }
    }
    Ok(market_totals_list)
}

#[substreams::handlers::map]
fn map_market_revenue_delta(
    accrue_interest_list: compound::AccrueInterestList,
    store_reserve_factor: store::StoreGet,
    store_price: store::StoreGet,
    store_token: store::StoreGet,
) -> Result<compound::MarketRevenueDeltaList, substreams::errors::Error> {
    let mut market_revenue_delta_list = compound::MarketRevenueDeltaList {
        market_revenue_delta_list: vec![],
    };
    for accrue_interest in accrue_interest_list.accrue_interest_list {
        let interest_accumulated_mantissa =
            BigDecimal::from_str(accrue_interest.interest_accumulated.as_str()).unwrap();
        let market_address = Hex::encode(accrue_interest.address.clone());
        let reserve_factor_res =
            store_reserve_factor.get_last(&format!("market:{}:reserve_factor", market_address));
        let underlying_price_res =
            store_price.get_last(&format!("market:{}:underlying:price", market_address));
        let underlying_res = store_token.get_last(&format!("market:{}:underlying", market_address));
        if let (Some(b_reserve_factor), Some(b_underlying_price), Some(b_underlying)) =
            (reserve_factor_res, underlying_price_res, underlying_res)
        {
            let reserve_factor = utils::string_to_bigdecimal(b_reserve_factor.as_ref());
            let underlying_price = utils::string_to_bigdecimal(b_underlying_price.as_ref());
            let underlying_token: compound::Token = proto::decode(&b_underlying).unwrap();
            let total_revenue = interest_accumulated_mantissa
                .div(exponent_to_big_decimal(underlying_token.decimals))
                .mul(underlying_price);
            let protocol_revenue = total_revenue.clone().mul(reserve_factor);
            let supply_revenue = total_revenue.clone().sub(protocol_revenue.clone());
            let revenue_delta = compound::MarketRevenueDelta {
                market: accrue_interest.address,
                total_revenue: total_revenue.to_string(),
                protocol_revenue: protocol_revenue.to_string(),
                supply_revenue: supply_revenue.to_string(),
                timestamp: accrue_interest.timestamp,
            };
            market_revenue_delta_list
                .market_revenue_delta_list
                .push(revenue_delta)
        }
    }
    Ok(market_revenue_delta_list)
}

#[substreams::handlers::store]
fn store_market_reserve_factor(blk: ethpb::v1::Block, output: store::StoreSet) {
    for log in blk.logs() {
        if let Some(new_reserve_factor) =
            abi::ctoken::events::NewReserveFactor::match_and_decode(log)
        {
            output.set(
                0,
                format!("market:{}:reserve_factor", Hex::encode(&log.log.address)),
                &Vec::from(
                    BigDecimal::from_str(
                        new_reserve_factor
                            .new_reserve_factor_mantissa
                            .to_string()
                            .as_str(),
                    )
                    .unwrap()
                    .div(exponent_to_big_decimal(MANTISSA_FACTOR))
                    .to_string(),
                ),
            )
        }
    }
}

// TODO: use append_bytes
#[substreams::handlers::store]
fn store_market_listed(market_listed_list: compound::MarketListedList, output: store::StoreAppend) {
    for market_listed in market_listed_list.market_listed_list {
        output.append(
            0,
            "protocol:market_listed".to_string(),
            &Hex::encode(&market_listed.ctoken),
        )
    }
}

#[substreams::handlers::store]
fn store_mint(mint_list: compound::MintList, output: store::StoreSet) {
    for mint in mint_list.mint_list {
        output.set(
            0,
            String::from_utf8(mint.id.clone()).unwrap(),
            &proto::encode(&mint).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_token(market_listed_list: compound::MarketListedList, output: store::StoreSet) {
    for market_listed in market_listed_list.market_listed_list {
        let ctoken_id = market_listed.ctoken;
        // handle eth and sai differently
        // because eth and sai (89d24a6b4ccb1b6faa2625fe562bdd9a23260359) are NOT ERC20 tokens
        let is_ceth = ctoken_id == Hex::decode("4ddc2d193948926d02f9b1fe9e1daa0718270ed5").unwrap();
        let is_csai = ctoken_id == Hex::decode("f5dce57282a584d2746faf1593d3121fcac444dc").unwrap();

        let ctoken_res = rpc::fetch_token(ctoken_id.clone());
        if ctoken_res.is_err() {
            continue;
        }
        let ctoken = ctoken_res.unwrap();
        let underlying_token_res = if is_ceth {
            Ok(compound::Token {
                id: NULL_ADDRESS.to_vec(),
                name: "Ether".to_string(),
                symbol: "ETH".to_string(),
                decimals: 18,
            })
        } else if is_csai {
            Ok(compound::Token {
                id: Hex::decode("89d24a6b4ccb1b6faa2625fe562bdd9a23260359").unwrap(),
                name: "Sai Stablecoin v1.0 (SAI)".to_string(),
                symbol: "SAI".to_string(),
                decimals: 18,
            })
        } else {
            rpc::fetch(rpc::RpcCallParams {
                to: ctoken_id.clone(),
                method: "underlying()".to_string(),
                args: vec![],
            })
            .map(|x| x[12..32].to_vec())
            .and_then(rpc::fetch_token)
        };
        if underlying_token_res.is_err() {
            continue;
        }
        let underlying_token = underlying_token_res.unwrap();
        output.set(
            0,
            format!("market:{}:ctoken", Hex::encode(ctoken_id.clone())),
            &proto::encode(&ctoken).unwrap(),
        );
        output.set(
            0,
            format!("market:{}:underlying", Hex::encode(ctoken_id.clone())),
            &proto::encode(&underlying_token).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_mint_count(mint_list: compound::MintList, output: store::StoreAddInt64) {
    for mint in mint_list.mint_list {
        output.add(
            0,
            format!("mint:count:{}", mint.timestamp / (24 * 60 * 60)),
            1,
        )
    }
}

#[substreams::handlers::store]
fn store_market_count(
    market_listed_list: compound::MarketListedList,
    output: store::StoreAddInt64,
) {
    for _ in market_listed_list.market_listed_list {
        output.add(0, "market:count".to_string(), 1)
    }
}

#[substreams::handlers::store]
fn store_oracle(blk: ethpb::v1::Block, output: store::StoreSet) {
    for log in blk.logs() {
        if let Some(new_price_oracle) =
            abi::comptroller::events::NewPriceOracle::match_and_decode(log)
        {
            output.set(
                0,
                "protocol:oracle".to_string(),
                &new_price_oracle.new_price_oracle,
            );
        }
    }
}

#[substreams::handlers::store]
fn store_price(
    accrue_interest_list: compound::AccrueInterestList,
    store_oracle: store::StoreGet,
    store_token: store::StoreGet,
    output: store::StoreSet,
) {
    for accrue_interest in accrue_interest_list.accrue_interest_list {
        let market_address = accrue_interest.address;
        let oracle_res = store_oracle.get_last(&"protocol:oracle".to_string());
        let underlying_res = store_token.get_last(&format!(
            "market:{}:underlying",
            Hex::encode(&market_address)
        ));
        if let (Some(oracle), Some(underlying)) = (oracle_res, underlying_res) {
            let underlying_token: compound::Token = proto::decode(&underlying).unwrap();
            let price_usd_res = utils::get_underlying_price_usd(
                market_address.clone(),
                underlying_token.id,
                oracle,
                accrue_interest.block_number,
                underlying_token.decimals,
            );
            if price_usd_res.is_err() {
                continue;
            }
            output.set(
                0,
                format!("market:{}:underlying:price", Hex::encode(&market_address)),
                &Vec::from(price_usd_res.unwrap().to_string()),
            )
        }
    }
}

#[substreams::handlers::store]
fn store_market_totals(market_totals_list: compound::MarketTotalsList, output: store::StoreSet) {
    for market_totals in market_totals_list.market_totals_list {
        output.set(
            0,
            format!("market:{}:tvl", Hex::encode(market_totals.market.clone())),
            &Vec::from(market_totals.total_value_locked),
        );
        output.set(
            0,
            format!("market:{}:total_borrows", Hex::encode(market_totals.market)),
            &Vec::from(market_totals.total_borrows),
        );
    }
}

#[substreams::handlers::store]
fn store_protocol_totals(
    market_totals_list: compound::MarketTotalsList,
    store_market_listed: store::StoreGet,
    store_market_totals: store::StoreGet,
    output: store::StoreSet,
) {
    for market_totals in market_totals_list.market_totals_list {
        let market_address = market_totals.market;
        let market_listed_res: Option<Vec<Vec<u8>>> = store_market_listed
            .get_last(&"protocol:market_listed".to_string())
            .map(|x| x.chunks(20).map(|s| s.into()).collect());
        if let Some(market_listed) = market_listed_res {
            let mut protocol_tvl = BigDecimal::zero();
            let mut protocol_total_borrows = BigDecimal::zero();
            for market in market_listed {
                if market == market_address {
                    protocol_tvl = protocol_tvl.add(
                        BigDecimal::from_str(market_totals.total_value_locked.as_str()).unwrap(),
                    );
                    protocol_total_borrows = protocol_total_borrows
                        .add(BigDecimal::from_str(market_totals.total_borrows.as_str()).unwrap());
                    continue;
                }
                let other_market_tvl_res: Option<BigDecimal> = store_market_totals
                    .get_last(&format!("market:{}:tvl", Hex::encode(&market_address)))
                    .map(|x| utils::string_to_bigdecimal(x.as_ref()));
                if let Some(other_market_tvl) = other_market_tvl_res {
                    protocol_tvl = protocol_tvl.add(other_market_tvl)
                }
                let other_market_total_borrows_res: Option<BigDecimal> = store_market_totals
                    .get_last(&format!(
                        "market:{}:total_borrows",
                        Hex::encode(&market_address)
                    ))
                    .map(|x| utils::string_to_bigdecimal(x.as_ref()));
                if let Some(other_market_total_borrows) = other_market_total_borrows_res {
                    protocol_total_borrows = protocol_total_borrows.add(other_market_total_borrows)
                }
            }

            output.set(
                0,
                "protocol:tvl".to_string(),
                &Vec::from(protocol_tvl.to_string()),
            );
            output.set(
                0,
                "protocol:total_borrows".to_string(),
                &Vec::from(protocol_total_borrows.to_string()),
            );
        }
    }
}

#[substreams::handlers::store]
fn store_revenue(
    market_revenue_delta_list: compound::MarketRevenueDeltaList,
    output: store::StoreAddBigFloat,
) {
    for market_revenue_delta in market_revenue_delta_list.market_revenue_delta_list {
        let market_address = Hex::encode(market_revenue_delta.market);
        let total_revenue_delta =
            BigDecimal::from_str(market_revenue_delta.total_revenue.as_str()).unwrap();
        let protocol_revenue_delta =
            BigDecimal::from_str(market_revenue_delta.protocol_revenue.as_str()).unwrap();
        let supply_revenue_delta =
            BigDecimal::from_str(market_revenue_delta.supply_revenue.as_str()).unwrap();

        // spot revenue
        output.add(
            0,
            format!("market:{}:revenue:total", market_address),
            &total_revenue_delta,
        );
        output.add(
            0,
            format!("market:{}:revenue:protocol", market_address),
            &BigDecimal::from_str(market_revenue_delta.protocol_revenue.as_str()).unwrap(),
        );
        output.add(
            0,
            format!("market:{}:revenue:supply", market_address),
            &BigDecimal::from_str(market_revenue_delta.supply_revenue.as_str()).unwrap(),
        );
        output.add(
            0,
            "protocol:revenue:total".to_string(),
            &total_revenue_delta,
        );
        output.add(
            0,
            "protocol:revenue:protocol".to_string(),
            &protocol_revenue_delta,
        );
        output.add(
            0,
            "protocol:revenue:supply".to_string(),
            &supply_revenue_delta,
        );

        // snapshot revenue
        output.add(
            0,
            format!(
                "market:{}:revenue:total:{}",
                market_address,
                market_revenue_delta.timestamp / (24 * 60 * 60)
            ),
            &total_revenue_delta,
        );
        output.add(
            0,
            format!(
                "market:{}:revenue:protocol:{}",
                market_address,
                market_revenue_delta.timestamp / (24 * 60 * 60)
            ),
            &BigDecimal::from_str(market_revenue_delta.protocol_revenue.as_str()).unwrap(),
        );
        output.add(
            0,
            format!(
                "market:{}:revenue:supply:{}",
                market_address,
                market_revenue_delta.timestamp / (24 * 60 * 60)
            ),
            &BigDecimal::from_str(market_revenue_delta.supply_revenue.as_str()).unwrap(),
        );
        output.add(
            0,
            format!(
                "protocol:revenue:total:{}",
                market_revenue_delta.timestamp / (24 * 60 * 60)
            ),
            &total_revenue_delta,
        );
        output.add(
            0,
            format!(
                "protocol:revenue:protocol:{}",
                market_revenue_delta.timestamp / (24 * 60 * 60)
            ),
            &protocol_revenue_delta,
        );
        output.add(
            0,
            format!(
                "protocol:revenue:supply:{}",
                market_revenue_delta.timestamp / (24 * 60 * 60)
            ),
            &supply_revenue_delta,
        );
    }
}

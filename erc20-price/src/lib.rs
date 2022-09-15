#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

use std::collections::HashSet;

use substreams::{log, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};

use pb::erc20_price::{Erc20Price, Erc20Prices};
use substreams_helper::price;
use substreams_helper::types::Network;

#[substreams::handlers::map]
fn map_price(block: eth::Block) -> Result<Erc20Prices, substreams::errors::Error> {
    let mut erc20_tokens = HashSet::new();
    for log in block.logs() {
        if let Some(_) = abi::erc20::events::Transfer::match_and_decode(log) {
            let erc20_token = log.log.clone().address;
            erc20_tokens.insert(Hex::encode(erc20_token));
        }
    }
    let mut prices = Erc20Prices { items: vec![] };
    for erc20_token in erc20_tokens {
        let erc20_token = Hex::decode(erc20_token).map_err(|e| {
            substreams::errors::Error::Unexpected(format!("Failed to decode erc20 token: {}", e))
        })?;
        let token_price =
            price::get_price(Network::Ethereum, erc20_token.clone()).map_err(|e| {
                substreams::errors::Error::Unexpected(format!("Failed to get price: {}", e))
            })?;
        prices.items.push(Erc20Price {
            block_number: block.number,
            price_usd: token_price.to_string(),
            token_address: erc20_token.clone(),
        });
        log::info!("token {} price {}", Hex(erc20_token), token_price);
    }
    Ok(prices)
}

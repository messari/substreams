#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

use hex_literal::hex;
use pb::erc20_price::v1::{Erc20Price, Erc20Prices};
use substreams::{log, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};
use substreams_helper::price;
use substreams_helper::types::Network;

#[substreams::handlers::map]
fn map_price(block: eth::Block) -> Result<Erc20Prices, substreams::errors::Error> {
    map_price_for_tokens(
        block.number,
        vec![hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").to_vec()],
    )
}

fn map_price_for_tokens(
    block_number: u64,
    erc20_tokens: Vec<Vec<u8>>,
) -> Result<Erc20Prices, substreams::errors::Error> {
    let mut prices = Erc20Prices { items: vec![] };

    for erc20_token in erc20_tokens {
        let token_price =
            price::get_price(Network::Ethereum, erc20_token.clone()).map_err(|e| {
                substreams::errors::Error::Unexpected(format!("Failed to get price: {}", e))
            })?;
        prices.items.push(Erc20Price {
            block_number,
            price_usd: token_price.to_string(),
            token_address: Hex(erc20_token.clone()).to_string(),
        });
        log::info!("token {} price {}", Hex(erc20_token), token_price);
    }

    Ok(prices)
}

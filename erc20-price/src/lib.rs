use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;

use pb::erc20_price::Erc20Price;
use substreams_helper::price;
use substreams_helper::types::Network;

mod pb;

const WETH_ADDRESS: &str = "c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";

#[substreams::handlers::map]
fn eth_price(block: eth::Block) -> Result<Erc20Price, substreams::errors::Error> {
    let eth_price = price::get_price(Network::Ethereum, Hex::decode(WETH_ADDRESS).unwrap()).unwrap();
    Ok(Erc20Price {
        block_number: block.number,
        price_usd: eth_price.to_string(),
    })
}

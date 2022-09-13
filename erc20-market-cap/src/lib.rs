mod pb;

use substreams_ethereum::pb::eth::v2 as eth;

use pb::erc20_market_cap::{Erc20MarketCap, Erc20MarketCaps};
use pb::erc20_price::{Erc20Price, Erc20Prices};
use substreams_helper::erc20::{self, Erc20Token};
use substreams_helper::types::Network;

#[substreams::handlers::map]
fn map_market_cap(prices: Erc20Prices) -> Result<Erc20MarketCaps, substreams::errors::Error> {
    let mut market_caps = vec![];

    for Erc20Price {
        price_usd,
        token_address,
        ..
    } in prices.items.iter()
    {
        let price = price_usd
            .parse::<f64>()
            .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?;

        let Erc20Token { total_supply, .. } = erc20::get_erc20_token(token_address.to_vec())
            .ok_or(substreams::errors::Error::Unexpected(String::from(
                "Failed to get token info",
            )))?;

        let market_cap = price * (total_supply as f64);

        market_caps.push(Erc20MarketCap {
            price,
            total_supply,
            market_cap,
        });
    }

    Ok(Erc20MarketCaps { items: market_caps })
}

#[rustfmt::skip]
pub mod pb;

use substreams::{log, Hex};

use pb::erc20_market_cap::v1::{Erc20MarketCap, Erc20MarketCaps};
use pb::erc20_price::v1::{Erc20Price, Erc20Prices};
use substreams_helper::erc20::{self, Erc20Token};
use substreams_helper::math;

#[substreams::handlers::map]
fn map_market_cap(prices: Erc20Prices) -> Result<Erc20MarketCaps, substreams::errors::Error> {
    let mut items = vec![];

    for Erc20Price {
        price_usd,
        token_address,
        ..
    } in prices.items.iter()
    {
        let price = math::decimal_from_str(price_usd)
            .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?;
        let token_address_hex = format!("0x{}", Hex::encode(token_address));

        let Erc20Token { total_supply, .. } = erc20::get_erc20_token(token_address_hex.clone())
            .ok_or(substreams::errors::Error::Unexpected(format!(
                "Failed to get token info for address: {}",
                token_address_hex
            )))?;

        let market_cap = price.clone()
            * math::decimal_from_str(&total_supply.to_string())
                .map_err(|e| substreams::errors::Error::Unexpected(e.to_string()))?;

        let item = Erc20MarketCap {
            price: format!("{:.7}", price),
            total_supply: total_supply.to_string(),
            market_cap: format!("{:.7}", market_cap),
            token_address: token_address_hex,
        };

        items.push(item);
    }

    log::info!("Market Caps: {:?}", items);

    Ok(Erc20MarketCaps { items })
}

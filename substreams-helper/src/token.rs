

pub fn get_eth_token() -> Option<Token> {
    let eth_token = Token {
        address: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE".to_string(),
        name: "Ethereum".to_string(),
        symbol: "ETH".to_string(),
        decimals: 18 as u64,
        total_supply: None,
    };

    Some(eth_token)
}

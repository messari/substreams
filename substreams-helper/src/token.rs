use crate::pb::sol_token::v1::TokenAccount;

pub fn get_sol_token() -> Option<TokenAccount> {
    let sol_token = TokenAccount {
        address: "TODO".to_string(),
        name: "Solana".to_string(),
        symbol: "SOL".to_string(),
        decimals: 18_u64,
        owner: "TODO".to_string(),
        mint: "TODO".to_string()
    };

    Some(sol_token)
}
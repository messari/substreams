use crate::pb::sol_token::v1::TokenAccount;

pub fn get_sol_token() -> Option<TokenAccount> {
    let sol_token = TokenAccount {
        address: "So11111111111111111111111111111111111111111".to_string(),
        name: "Solana".to_string(),
        symbol: "SOL".to_string(),
        decimals: 9_u64,
        freeze_authority: "NA".to_string(),
        mint_authority: "NA".to_string(),
    };

    Some(sol_token)
}

use crate::pb::sol_token::v1::TokenAccount;

pub fn get_sol_token() -> Option<TokenAccount> {
    let sol_token = TokenAccount {
        address: "So11111111111111111111111111111111111111111".to_string(),
        name: "Solana".to_string(),
        symbol: "SOL".to_string(),
        decimals: 9_u64,
        freeze_authority: "".to_string(),
        mint_authority: "".to_string(),
        tx_created: "".to_string(),
        block_created: 0_u64,
    };

    Some(sol_token)
}

use substreams::store;
use substreams::store::{StoreNew, StoreSet, StoreSetProto};

use crate::pb::synthetix::v1::{TokenBalance, TokenBalances};

#[substreams::handlers::store]
fn store_balances(balances: TokenBalances, store: store::StoreSetProto<TokenBalance>) {
    for balance in balances.balances {
        store.set(0, token_balance_key(&balance), &balance);
    }
}

fn token_balance_key(balance: &TokenBalance) -> String {
    format!("{}-{}", balance.token, balance.holder)
}

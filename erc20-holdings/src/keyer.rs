pub fn account_balance_key(account_address: &String) -> String {
    format!("account_balance:{}", account_address)
}

pub fn account_balance_usd_key(account_address: &String) -> String {
    format!("account_balance_usd:{}", account_address)
}

pub fn account_from_balance_key(key: &String) -> String {
    key.split(":").collect::<Vec<&str>>()[1].to_string()
}

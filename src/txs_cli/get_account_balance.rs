use crate::txs_core::{client::Client, types::account_address::AccountAddress};
use anyhow::Result;

pub async fn run(account_address: &str) -> Result<String> {
    let client = Client::default();
    let account_address = AccountAddress::from_hex_literal(account_address)?;

    client
        .get_account_balance(&account_address)
        .await
        .map(|balance| format!("Account balance: {balance} coins"))
}

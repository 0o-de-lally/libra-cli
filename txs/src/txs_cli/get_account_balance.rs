use anyhow::Result;
use txs::{
    coin_client::CoinClient, extension::client_ext::ClientExt, rest_client::Client,
    types::account_address::AccountAddress,
};

pub async fn run(account_address: &str) -> Result<String> {
    let client = Client::default();
    let coin_client = CoinClient::new(&client);
    let account_address = AccountAddress::from_hex_literal(account_address)?;

    coin_client
        .get_account_balance(&account_address)
        .await
        .map(|balance| format!("Account balance: {balance} coins"))
}

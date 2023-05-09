use anyhow::Result;
use aptos_sdk::{rest_client::Client, types::account_address::AccountAddress};
use txs::extension::client_ext::ClientExt;

pub async fn run(account_address: &str, resource_type: Option<String>) -> Result<String> {
    let client = Client::default();
    let account_address = AccountAddress::from_hex_literal(account_address)?;

    client
        .get_account_resource_ext(account_address, resource_type)
        .await
}

use crate::txs_core::{
    client::{Client, TransferOptions},
    crypto::{ed25519::Ed25519PrivateKey, ValidCryptoMaterialStringExt},
    types::{account_address::AccountAddress, AccountKey, LocalAccount},
};
use anyhow::{Context, Result};

pub async fn run(
    to_account: &str,
    amount: u64,
    private_key: &str,
    gas_unit_price: Option<u64>,
) -> Result<()> {
    let client = Client::default();
    let private_key = Ed25519PrivateKey::from_encoded_string(private_key)
        .context(format!("Failed to parse the private key {private_key}"))?;
    let account_key = AccountKey::from_private_key(private_key);
    let account_address = account_key.authentication_key().derived_address();
    let sequence_number = client.get_sequence_number(&account_address).await?;
    let mut from_account = LocalAccount::new(account_address, account_key, sequence_number);
    let to_account = AccountAddress::from_hex_literal(to_account).context(format!(
        "Failed to parse the recipient address {to_account}"
    ))?;
    let transfer_options = gas_unit_price.map(TransferOptions::from_gas_unit_price);

    client
        .wait_for_transaction(
            &client
                .transfer(&mut from_account, to_account, amount, transfer_options)
                .await?,
        )
        .await?;

    println!("Success!");
    Ok(())
}

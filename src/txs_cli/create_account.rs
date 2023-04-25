use crate::txs_core::{client::Client, types::account_address::AccountAddress};
use anyhow::{Context, Result};

pub async fn run(account_address: &str, coins: u64) -> Result<()> {
    let client = Client::default();
    let account_address = AccountAddress::from_hex_literal(account_address)?;

    if coins == 0 {
        client
            .create_account_by_faucet(account_address)
            .await
            .context(format!(
                "Failed to create account {}",
                account_address.to_hex_literal()
            ))?;
    } else {
        client
            .fund_by_faucet(account_address, coins)
            .await
            .context(format!("Failed to create account {}", account_address))?;
    }

    println!("Success!");
    Ok(())
}

use crate::txs_core::{
    client::Client,
    constant::{
        DEFAULT_COIN_TYPE, DEFAULT_GAS_UNIT_PRICE, DEFAULT_MAX_GAS_AMOUNT, DEFAULT_TIMEOUT_SECS,
    },
    types::{account_address::AccountAddress, LocalAccount, TransferOptions},
};
use anyhow::{Context, Result};

pub async fn run(
    to_account: &str,
    amount: u64,
    private_key: &str,
    max_gas: Option<u64>,
    gas_unit_price: Option<u64>,
) -> Result<()> {
    let client = Client::default();
    let mut from_account = LocalAccount::from_private_key(private_key, None).await?;
    let to_account = AccountAddress::from_hex_literal(to_account).context(format!(
        "Failed to parse the recipient address {to_account}"
    ))?;
    let transfer_options = TransferOptions {
        max_gas_amount: max_gas.unwrap_or(DEFAULT_MAX_GAS_AMOUNT),
        gas_unit_price: gas_unit_price.unwrap_or(DEFAULT_GAS_UNIT_PRICE),
        timeout_secs: DEFAULT_TIMEOUT_SECS,
        coin_type: DEFAULT_COIN_TYPE,
    };

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

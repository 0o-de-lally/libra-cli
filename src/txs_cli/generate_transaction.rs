use crate::txs_core::{
    client::Client,
    constant::{DEFAULT_GAS_UNIT_PRICE, DEFAULT_MAX_GAS_AMOUNT, DEFAULT_TIMEOUT_SECS},
    types::{transaction::SignedTransaction, LocalAccount, TransactionOptions},
};
use anyhow::Result;
use aptos_crypto::{ed25519::Ed25519PrivateKey, ValidCryptoMaterialStringExt};

pub async fn run(
    function_id: &str,
    private_key: &str,
    type_args: Option<String>,
    args: Option<String>,
    max_gas: Option<u64>,
    gas_unit_price: Option<u64>,
) -> Result<SignedTransaction> {
    let client = Client::default();
    let private_key = Ed25519PrivateKey::from_encoded_string(private_key)?;
    let mut account = LocalAccount::from_private_key(private_key, None).await?;
    let options = TransactionOptions {
        max_gas_amount: max_gas.unwrap_or(DEFAULT_MAX_GAS_AMOUNT),
        gas_unit_price: gas_unit_price.unwrap_or(DEFAULT_GAS_UNIT_PRICE),
        timeout_secs: DEFAULT_TIMEOUT_SECS,
    };

    client
        .generate_transaction(&mut account, function_id, type_args, args, options)
        .await
}

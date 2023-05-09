use anyhow::Result;
use aptos_sdk::{rest_client::Client, types::transaction::SignedTransaction};
use txs::extension::client_ext::ClientExt;

pub async fn run(signed_trans: &SignedTransaction) -> Result<()> {
    let client = Client::default();
    let pending_trans = client.submit(signed_trans).await?.into_inner();
    client.wait_for_transaction(&pending_trans).await?;
    Ok(())
}

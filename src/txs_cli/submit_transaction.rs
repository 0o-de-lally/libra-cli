use anyhow::Result;

use crate::txs_core::{client::Client, types::transaction::SignedTransaction};

pub async fn run(signed_trans: &SignedTransaction) -> Result<()> {
    let client = Client::default();
    let pending_trans = client.submit_transaction(signed_trans).await?;
    client.wait_for_transaction(&pending_trans).await?;
    Ok(())
}

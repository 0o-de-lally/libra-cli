use anyhow::Result;
use clap::Parser;
use txs::txs_args::TxsArgs;

#[tokio::main]
async fn main() -> Result<()> {
    TxsArgs::parse().run().await
}

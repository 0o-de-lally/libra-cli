use crate::transfer_coin::transfer_coin;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(name = "Txs", author, version)]
pub struct TxsArgs {
    /// Demo transfer coin transaction for local testnet
    #[clap(long)]
    demo_tx: bool,

    /// Display information
    #[clap(long)]
    info: bool,
}

impl TxsArgs {
    /// Runs Txs based on the given command line arguments
    pub async fn run(self) -> Result<()> {
        if self.demo_tx {
            println!("running demo tx ...");
            return transfer_coin()
                .await
                .context("Failed to run transfer_coin tx");
        }

        if self.info {
            println!("todo --- info");
        }

        Ok(())
    }
}

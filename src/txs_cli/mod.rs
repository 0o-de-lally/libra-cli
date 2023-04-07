use anyhow::{Context, Result};
use clap::Parser;

mod transfer_coin;

#[derive(Clone, Debug, Parser)]
#[clap(name = "Txs", author, version)]
pub struct TxsCli {
    /// Demo transfer coin transaction for local testnet
    #[clap(long)]
    demo_tx: bool,

    /// Display information
    #[clap(long)]
    info: bool,
}

impl TxsCli {
    pub async fn run(&self) -> Result<()> {
        if self.demo_tx {
            println!("running demo tx ...");
            return transfer_coin::run()
                .await
                .context("Failed to run transfer_coin tx");
        }

        if self.info {
            println!("todo --- info");
        }

        Ok(())
    }
}

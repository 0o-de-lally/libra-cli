use crate::transfer_coin::transfer_coin;
use anyhow::{Context};
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
    pub async fn run(self) {
        if self.demo_tx {
            println!("running demo tx ...");

            let _result = transfer_coin().await.context("Failed to run transfer_coin tx");
            return;
        }

        if self.info {
            println!("todo --- info");
            return;
        }
    }
}
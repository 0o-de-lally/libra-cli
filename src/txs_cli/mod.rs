use anyhow::{Context, Result};
use clap::Parser;

mod generate_local_account;
mod transfer_coin;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), author, version, about, long_about = None, arg_required_else_help = true)]
pub struct TxsCli {
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    #[clap(about = "Demo transfer coin transaction for local testnet")]
    Demo,
    #[clap(about = "Generate keys and account address locally")]
    GenerateLocalAccount {
        #[arg(
            short,
            long,
            value_name = "Private Key",
            help = "Generate account from the given private key"
        )]
        private_key: Option<String>,
    },
}

impl TxsCli {
    pub async fn run(&self) -> Result<()> {
        match &self.subcommand {
            Some(Subcommand::Demo) => transfer_coin::run()
                .await
                .context("Failed to run transfer_coin tx"),
            Some(Subcommand::GenerateLocalAccount { private_key }) => {
                println!(
                    "{}",
                    generate_local_account::run(&private_key.clone().unwrap_or_default())?
                );
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

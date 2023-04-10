use anyhow::Result;
use clap::Parser;

mod create_account;
mod demo;
mod generate_local_account;
mod get_account_balance;
mod get_account_resource;
mod transfer_coin;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), author, version, about, long_about = None, arg_required_else_help = true)]
pub struct TxsCli {
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    #[clap(about = "Demo transfer coin example for local testnet")]
    Demo,

    #[clap(about = "Generate keys and account address locally")]
    GenerateLocalAccount {
        #[arg(
            short,
            long,
            value_name = "private key",
            help = "Generate account from the given private key"
        )]
        private_key: Option<String>,
    },

    #[clap(about = "Create onchain account by using Aptos faucet")]
    CreateAccount {
        #[arg(
            short,
            long,
            value_name = "account address",
            help = "Create onchain account with the given address"
        )]
        account_address: String,

        #[arg(
            short,
            long,
            value_name = "some amount of coins",
            help = "The amount of coins to fund the new account"
        )]
        coins: Option<u64>,
    },

    #[clap(about = "Get account balance")]
    GetAccountBalance {
        #[arg(
            short,
            long,
            value_name = "account address",
            help = "Address of the onchain account to get balance from"
        )]
        account_address: String,
    },

    #[clap(about = "Get account resource")]
    GetAccountResource {
        #[arg(
            short,
            long,
            value_name = "account address",
            help = "Address of the onchain account to get resource from"
        )]
        account_address: String,

        #[arg(
            short,
            long,
            value_name = "resource type",
            help = "Type of the resource to get from account"
        )]
        resource_type: Option<String>,
    },

    #[clap(about = "Transfer coins between accounts")]
    TransferCoins {
        #[arg(
            short,
            long,
            value_name = "account address",
            help = "Address of the recipient"
        )]
        to_account: String,

        #[arg(
            short,
            long,
            value_name = "some amount of coins",
            help = "The amount of coins to transfer"
        )]
        amount: u64,

        #[arg(
            short,
            long,
            value_name = "private key",
            help = "Private key of the account to withdraw money from"
        )]
        private_key: String,

        #[arg(
            short,
            long,
            value_name = "gas unit price",
            help = "The amount of coins to pay for 1 gas unit. The higher the price is, the higher priority your transaction will be executed with"
        )]
        gas_unit_price: Option<u64>,
    },
}

impl TxsCli {
    pub async fn run(&self) -> Result<()> {
        match &self.subcommand {
            Some(Subcommand::Demo) => demo::run().await,
            Some(Subcommand::GenerateLocalAccount { private_key }) => {
                println!(
                    "{}",
                    generate_local_account::run(&private_key.clone().unwrap_or_default())?
                );
                Ok(())
            }
            Some(Subcommand::CreateAccount {
                account_address,
                coins,
            }) => create_account::run(account_address, coins.unwrap_or_default()).await,
            Some(Subcommand::GetAccountBalance { account_address }) => {
                println!("{}", get_account_balance::run(account_address).await?);
                Ok(())
            }
            Some(Subcommand::GetAccountResource {
                account_address,
                resource_type,
            }) => {
                println!(
                    "{}",
                    get_account_resource::run(account_address, resource_type.to_owned()).await?
                );
                Ok(())
            }
            Some(Subcommand::TransferCoins {
                to_account,
                amount,
                private_key,
                gas_unit_price,
            }) => {
                transfer_coin::run(
                    to_account,
                    amount.to_owned(),
                    private_key,
                    gas_unit_price.to_owned(),
                )
                .await
            }
            _ => Ok(()),
        }
    }
}

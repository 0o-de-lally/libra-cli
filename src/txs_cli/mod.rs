use std::path::PathBuf;

use crate::txs::util::format_signed_transaction;
use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use indoc::indoc;

mod create_account;
mod demo;
mod generate_local_account;
mod generate_transaction;
mod get_account_balance;
mod get_account_resource;
mod submit_transaction;
mod transfer_coin;
mod view;

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
            value_name = "PRIVATE_KEY",
            help = "Generate account from the given private key"
        )]
        private_key: Option<String>,

        #[arg(
            short,
            long,
            value_name = "OUTPUT_DIR",
            help = "Path of the directory to store yaml files"
        )]
        output_dir: Option<String>,
    },

    #[clap(about = "Create onchain account by using Aptos faucet")]
    CreateAccount {
        #[arg(
            short,
            long,
            value_name = "ACCOUNT_ADDRESS",
            help = "Create onchain account with the given address"
        )]
        account_address: String,

        #[arg(
            short,
            long,
            value_name = "COINS",
            help = "The amount of coins to fund the new account"
        )]
        coins: Option<u64>,
    },

    #[clap(about = "Get account balance")]
    GetAccountBalance {
        #[arg(
            short,
            long,
            value_name = "ACCOUNT_ADDRESS",
            help = "Address of the onchain account to get balance from"
        )]
        account_address: String,
    },

    #[clap(about = "Get account resource")]
    GetAccountResource {
        #[arg(
            short,
            long,
            value_name = "ACCOUNT_ADDRESS",
            help = "Address of the onchain account to get resource from"
        )]
        account_address: String,

        #[arg(
            short,
            long,
            value_name = "RESOURCE_TYPE",
            help = "Type of the resource to get from account"
        )]
        resource_type: Option<String>,
    },

    #[clap(about = "Transfer coins between accounts")]
    TransferCoins {
        #[arg(short, long, value_name = "ADDR", help = "Address of the recipient")]
        to_account: String,

        #[arg(
            short,
            long,
            value_name = "AMOUNT",
            help = "The amount of coins to transfer"
        )]
        amount: u64,

        #[arg(
            short,
            long,
            value_name = "PRIVATE_KEY",
            help = "Private key of the account to withdraw money from"
        )]
        private_key: String,

        #[arg(
            short,
            long,
            value_name = "MAX_GAS",
            help = "Maximum number of gas units to be used to send this transaction"
        )]
        max_gas: Option<u64>,

        #[arg(
            short,
            long,
            value_name = "GAS_UNIT_PRICE",
            help = "The amount of coins to pay for 1 gas unit. The higher the price is, the higher priority your transaction will be executed with"
        )]
        gas_unit_price: Option<u64>,
    },

    #[clap(about = "Generate a transaction that executes an Entry function on-chain")]
    GenerateTransaction {
        #[arg(
            short,
            long,
            value_name = "FUNCTION_ID",
            help = indoc!{r#"
                Function identifier has the form <ADDRESS>::<MODULE_ID>::<FUNCTION_NAME>

                Example:
                0x1::coin::transfer
            "#}
        )]
        function_id: String,

        #[arg(
            short,
            long,
            value_name = "TYPE_ARGS",
            help = indoc!{ r#"
                Type arguments separated by commas

                Example: 
                'u8, u16, u32, u64, u128, u256, bool, address, vector<u8>, signer'
                '0x1::aptos_coin::AptosCoin'
            "#}
        )]
        type_args: Option<String>,

        #[arg(
            short,
            long,
            value_name = "ARGS",
            help = indoc!{ r#"
                Function arguments separated by commas

                Example:
                '0x1, true, 12, 24_u8, x"123456"'
            "#}
        )]
        args: Option<String>,

        #[arg(
            short,
            long,
            value_name = "MAX_GAS",
            help = "Maximum amount of gas units to be used to send this transaction"
        )]
        max_gas: Option<u64>,

        #[arg(
            short,
            long,
            value_name = "GAS_UNIT_PRICE",
            help = "The amount of coins to pay for 1 gas unit. The higher the price is, the higher priority your transaction will be executed with"
        )]
        gas_unit_price: Option<u64>,

        #[arg(
            short,
            long,
            value_name = "PRIVATE_KEY",
            help = "Private key to sign the transaction"
        )]
        private_key: String,

        #[arg(
            short,
            long,
            help = "Submit the generated transaction to the blockchain"
        )]
        submit: bool,
    },

    #[clap(about = "Execute a View function on-chain")]
    View {
        #[arg(
            short,
            long,
            value_name = "FUNCTION_ID",
            help = indoc!{r#"
                Function identifier has the form <ADDRESS>::<MODULE_ID>::<FUNCTION_NAME>

                Example:
                0x1::coin::balance
            "#}
        )]
        function_id: String,

        #[arg(
            short,
            long,
            value_name = "TYPE_ARGS",
            help = indoc!{ r#"
                Type arguments separated by commas

                Example: 
                'u8, u16, u32, u64, u128, u256, bool, address, vector<u8>, signer'
                '0x1::aptos_coin::AptosCoin'
            "#}
        )]
        type_args: Option<String>,

        #[arg(
            short,
            long,
            value_name = "ARGS",
            help = indoc!{ r#"
                Function arguments separated by commas

                Example:
                '0x1, true, 12, 24_u8, x"123456"'
            "#}
        )]
        args: Option<String>,
    },
}

impl TxsCli {
    pub async fn run(&self) -> Result<()> {
        match &self.subcommand {
            Some(Subcommand::Demo) => demo::run().await,
            Some(Subcommand::GenerateLocalAccount {
                private_key,
                output_dir,
            }) => {
                println!(
                    "{}",
                    generate_local_account::run(
                        &private_key.clone().unwrap_or_default(),
                        output_dir.as_ref().map(PathBuf::from)
                    )
                    .await?
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
                max_gas,
                gas_unit_price,
            }) => {
                transfer_coin::run(
                    to_account,
                    amount.to_owned(),
                    private_key,
                    max_gas.to_owned(),
                    gas_unit_price.to_owned(),
                )
                .await
            }
            Some(Subcommand::GenerateTransaction {
                function_id,
                type_args,
                args,
                max_gas,
                gas_unit_price,
                private_key,
                submit,
            }) => {
                println!("====================");
                let signed_trans = generate_transaction::run(
                    function_id,
                    private_key,
                    type_args.to_owned(),
                    args.to_owned(),
                    max_gas.to_owned(),
                    gas_unit_price.to_owned(),
                )
                .await?;

                println!("{}", format_signed_transaction(&signed_trans));

                if *submit {
                    println!("{}", "Submitting transaction...".green().bold());
                    submit_transaction::run(&signed_trans).await?;
                    println!("Success!");
                }
                Ok(())
            }
            Some(Subcommand::View {
                function_id,
                type_args,
                args,
            }) => {
                println!("====================");
                println!(
                    "{}",
                    view::run(function_id, type_args.to_owned(), args.to_owned()).await?
                );
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

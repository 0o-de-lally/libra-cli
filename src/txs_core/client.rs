// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::{
    config::Config,
    constant::DEFAULT_ACCOUNT_RESOURCE_TYPE,
    transaction_builder::TransactionBuilder,
    types::{
        account_address::AccountAddress,
        chain_id::ChainId,
        transaction::{EntryFunction, SignedTransaction, TransactionArgument, TransactionPayload},
        LocalAccount, TransactionOptions, TransferOptions,
    },
};
use crate::txs_core::util::{format_args, format_type_args, parse_function_id};
use anyhow::{anyhow, Context, Result};
use aptos_rest_client::{
    aptos_api_types::{EntryFunctionId, IndexResponse, MoveType, ViewRequest},
    Account, Client as ApiClient, FaucetClient, PendingTransaction,
};
use bcs;
use move_core_types::{
    identifier::Identifier,
    language_storage::{ModuleId, TypeTag},
    parser::{parse_transaction_arguments, parse_type_tags},
    transaction_argument::convert_txn_args,
};
use once_cell::sync::Lazy;
use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};
use url::Url;

pub struct Client {
    api_client: ApiClient,
    faucet_client: FaucetClient,
}

static FAUCET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_FAUCET_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("http://0.0.0.0:8081"),
    )
    .unwrap()
});

impl Client {
    pub fn new(url: &Url) -> Self {
        Self {
            api_client: ApiClient::new(url.clone()),
            faucet_client: FaucetClient::new(FAUCET_URL.clone(), url.clone()),
        }
    }

    pub async fn transfer(
        &self,
        from_account: &mut LocalAccount,
        to_account: AccountAddress,
        amount: u64,
        options: TransferOptions<'_>,
    ) -> Result<PendingTransaction> {
        let chain_id = self.get_index().await?.chain_id;
        let transaction_builder = TransactionBuilder::new(
            TransactionPayload::EntryFunction(EntryFunction::new(
                ModuleId::new(AccountAddress::ONE, Identifier::new("coin").unwrap()),
                Identifier::new("transfer").unwrap(),
                vec![TypeTag::from_str(options.coin_type).unwrap()],
                vec![
                    bcs::to_bytes(&to_account).unwrap(),
                    bcs::to_bytes(&amount).unwrap(),
                ],
            )),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + options.timeout_secs,
            ChainId::new(chain_id),
        )
        .max_gas_amount(options.max_gas_amount)
        .gas_unit_price(options.gas_unit_price);
        let signed_txn = from_account.sign_with_transaction_builder(transaction_builder);
        self.submit_transaction(&signed_txn).await
    }

    pub async fn generate_transaction(
        &self,
        from_account: &mut LocalAccount,
        function_id: &str,
        ty_args: Option<String>,
        args: Option<String>,
        options: TransactionOptions,
    ) -> Result<SignedTransaction> {
        let chain_id = self.get_index().await?.chain_id;
        let (module_address, module_name, function_name) = parse_function_id(function_id)?;
        let module = ModuleId::new(module_address, module_name);
        let ty_args: Vec<TypeTag> = if let Some(ty_args) = ty_args {
            parse_type_tags(&ty_args)
                .context(format!("Unable to parse the type argument(s): {ty_args}"))?
        } else {
            vec![]
        };
        let args: Vec<TransactionArgument> = if let Some(args) = args {
            parse_transaction_arguments(&args)
                .context(format!("Unable to parse argument(s): {args}"))?
        } else {
            vec![]
        };

        println!("{}", format_type_args(&ty_args));
        println!("{}", format_args(&args));

        let expiration_timestamp_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + options.timeout_secs;

        let transaction_builder = TransactionBuilder::new(
            TransactionPayload::EntryFunction(EntryFunction::new(
                module,
                function_name,
                ty_args,
                convert_txn_args(&args),
            )),
            expiration_timestamp_secs,
            ChainId::new(chain_id),
        )
        .max_gas_amount(options.max_gas_amount)
        .gas_unit_price(options.gas_unit_price);

        Ok(from_account.sign_with_transaction_builder(transaction_builder))
    }

    async fn get_index(&self) -> Result<IndexResponse> {
        self.api_client
            .get_index()
            .await
            .context("Failed to Index")
            .map(|res| res.inner().clone())
    }

    pub async fn get_account_balance(&self, account: &AccountAddress) -> Result<u64> {
        let response = self
            .api_client
            .get_account_balance(*account)
            .await
            .context("Failed to get account balance")?;
        Ok(response.inner().get())
    }

    pub async fn get_sequence_number(&self, account: &AccountAddress) -> Result<u64> {
        let response = self
            .api_client
            .get_account_resource(*account, DEFAULT_ACCOUNT_RESOURCE_TYPE)
            .await
            .context("Failed to get account resource")?;
        if let Some(res) = response.inner() {
            Ok(serde_json::from_value::<Account>(res.data.to_owned())?.sequence_number)
        } else {
            Err(anyhow!("No data returned for the sequence number"))
        }
    }

    pub async fn get_account_resource(
        &self,
        account: &AccountAddress,
        resource_type: Option<String>,
    ) -> Result<String> {
        let response = self
            .api_client
            .get_account_resource(
                *account,
                resource_type
                    .as_deref()
                    .unwrap_or(DEFAULT_ACCOUNT_RESOURCE_TYPE),
            )
            .await
            .context("Failed to get account resource")?;
        if let Some(res) = response.inner() {
            Ok(format!("{:#}", res.data))
        } else {
            Err(anyhow!("No data returned for the account resource"))
        }
    }

    pub async fn view(
        &self,
        function_id: &str,
        ty_args: Option<String>,
        args: Option<String>,
    ) -> Result<Vec<serde_json::Value>> {
        let entry_fuction_id = EntryFunctionId::from_str(function_id)
            .context(format!("Invalid function id: {function_id}"))?;
        let ty_args: Vec<MoveType> = if let Some(ty_args) = ty_args {
            parse_type_tags(&ty_args)
                .context(format!("Unable to parse the type argument(s): {ty_args}"))?
                .iter()
                .map(|t| t.into())
                .collect()
        } else {
            vec![]
        };
        let args: Vec<serde_json::Value> = if let Some(args) = args {
            let mut output = vec![];
            for arg in args.split(',') {
                let arg = serde_json::Value::try_from(arg.trim())
                    .context(format!("Failed to parse argument: {arg}"))?;
                output.push(arg);
            }
            output
        } else {
            vec![]
        };

        println!("{}", format_type_args(&ty_args));
        println!("{}", format_args(&args));

        let request = ViewRequest {
            function: entry_fuction_id,
            type_arguments: ty_args,
            arguments: args,
        };

        self.api_client
            .view(&request, None)
            .await
            .context("Failed to execute View request")
            .map(|res| res.inner().to_owned())
    }

    pub async fn submit_transaction(
        &self,
        signed_trans: &SignedTransaction,
    ) -> Result<PendingTransaction> {
        Ok(self
            .api_client
            .submit(signed_trans)
            .await
            .context("Transaction failed")?
            .into_inner())
    }

    pub async fn wait_for_transaction(
        &self,
        pending_transaction: &PendingTransaction,
    ) -> Result<()> {
        self.api_client
            .wait_for_transaction(pending_transaction)
            .await
            .map_err(|e| anyhow!(e))
            .map(|_| ())
    }

    pub async fn create_account_by_faucet(&self, address: AccountAddress) -> Result<()> {
        self.faucet_client.create_account(address).await
    }

    pub async fn fund_by_faucet(&self, address: AccountAddress, amount: u64) -> Result<()> {
        self.faucet_client.fund(address, amount).await
    }
}

impl Default for Client {
    fn default() -> Self {
        let config = Config::default();
        let node_url = Url::from_str(&config.node_url).unwrap();
        Client::new(&node_url)
    }
}

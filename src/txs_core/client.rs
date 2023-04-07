// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::{
    transaction_builder::TransactionBuilder,
    types::{
        account_address::AccountAddress,
        chain_id::ChainId,
        transaction::{EntryFunction, TransactionPayload},
        LocalAccount,
    },
};
use anyhow::{Context, Result};
use aptos_rest_client::{Client as ApiClient, FaucetClient, PendingTransaction};
use bcs;
use move_core_types::{
    identifier::Identifier,
    language_storage::{ModuleId, TypeTag},
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
        options: Option<TransferOptions<'_>>,
    ) -> Result<PendingTransaction> {
        let options = options.unwrap_or_default();

        let chain_id = self
            .api_client
            .get_index()
            .await
            .context("Failed to get chain ID")?
            .inner()
            .chain_id;
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
        .sender(from_account.address())
        .sequence_number(from_account.sequence_number())
        .max_gas_amount(options.max_gas_amount)
        .gas_unit_price(options.gas_unit_price);
        let signed_txn = from_account.sign_with_transaction_builder(transaction_builder);
        Ok(self
            .api_client
            .submit(&signed_txn)
            .await
            .context("Failed to submit transfer transaction")?
            .into_inner())
    }

    pub async fn get_account_balance(&self, account: &AccountAddress) -> Result<u64> {
        let response = self
            .api_client
            .get_account_balance(*account)
            .await
            .context("Failed to get account balance")?;
        Ok(response.inner().get())
    }

    pub async fn wait_for_transaction(
        &self,
        pending_transaction: &PendingTransaction,
    ) -> Result<()> {
        self.api_client
            .wait_for_transaction(pending_transaction)
            .await
            .context("Failed when waiting for the transaction")
            .map(|_| ())
    }

    pub async fn create_account_by_faucet(&self, address: AccountAddress) -> Result<()> {
        self.faucet_client.create_account(address).await
    }

    pub async fn fund_by_faucet(&self, address: AccountAddress, amount: u64) -> Result<()> {
        self.faucet_client.fund(address, amount).await
    }
}

pub struct TransferOptions<'a> {
    pub max_gas_amount: u64,

    pub gas_unit_price: u64,

    /// This is the number of seconds from now you're willing to wait for the
    /// transaction to be committed.
    pub timeout_secs: u64,

    /// This is the coin type to transfer.
    pub coin_type: &'a str,
}

impl<'a> Default for TransferOptions<'a> {
    fn default() -> Self {
        Self {
            max_gas_amount: 5_000,
            gas_unit_price: 100,
            timeout_secs: 10,
            coin_type: "0x1::aptos_coin::AptosCoin",
        }
    }
}

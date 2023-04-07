use crate::{
    coin_client::CoinClient,
    config::Config,
    crypto::ed25519::Ed25519PrivateKey,
    rest_client::{Client, FaucetClient},
    types::{AccountKey, LocalAccount},
};
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use std::str::FromStr;
use url::Url;

static FAUCET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_FAUCET_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("http://0.0.0.0:8081"),
    )
    .unwrap()
});

pub async fn transfer_coin() -> Result<()> {
    let config = Config::new();
    let node_url = Url::from_str(&config.node_url).unwrap();
    let rest_client = Client::new(node_url.clone());
    let faucet_client = FaucetClient::new(FAUCET_URL.clone(), node_url.clone());
    let coin_client = CoinClient::new(&rest_client);

    // let mut alice = LocalAccount::generate(&mut rand::rngs::OsRng); // Aptos Alice
    // 0L Alice - from pri key
    let arr: [u8; 32] = [
        196, 63, 87, 153, 70, 68, 235, 218, 30, 171, 254, 191, 132, 222, 247, 63, 189, 29, 60, 228,
        66, 169, 210, 178, 244, 203, 159, 77, 167, 185, 144, 140,
    ];
    let pri = Ed25519PrivateKey::try_from(arr.as_ref()).unwrap();
    println!("--- tc::main: pri: {:x?}", pri.to_bytes());
    println!("--- tc::main: pri: {:?}", pri.to_bytes());
    let acckey = AccountKey::from_private_key(pri);
    println!("--- tc::main: pub: {:x?}", acckey.public_key());
    println!("--- tc::main: aut: {:x?}", acckey.authentication_key());
    println!(
        "--- tc::main: add: {:x?}",
        acckey.authentication_key().derived_address()
    );
    let mut alice = LocalAccount::new(acckey.authentication_key().derived_address(), acckey, 0);
    // ol alice addr 0xfda03992f666875ddf854193fccd3e62ea111d066029490dd37c891ed9c3f880

    // let bob = LocalAccount::generate(&mut rand::rngs::OsRng);
    // 0L: Using bob with fix address instead of random
    let derive_path = "m/44'/637'/0'/0'/0'";
    let mnemonic_phrase =
        "circle ship inner pact earn inflict valve retire mechanic talk mouse outer display snack dose ahead orient tooth shrimp achieve pink slam kingdom rifle";
    let bob = LocalAccount::from_derive_path(derive_path, mnemonic_phrase, 0).unwrap();

    // Print account addresses.
    println!("\n=== Addresses ===");
    println!("Alice: {}", alice.address().to_hex_literal());
    println!("Bob: {}", bob.address().to_hex_literal());

    // Create the accounts on chain, but only fund Alice.
    faucet_client
        .fund(alice.address(), 100_000_000)
        .await
        .context("Failed to fund Alice's account")?;
    faucet_client
        .create_account(bob.address())
        .await
        .context("Failed to fund Bob's account")?;

    // Print initial balances.
    println!("\n=== Initial Balances ===");
    println!(
        "Alice: {:?}",
        coin_client
            .get_account_balance(&alice.address())
            .await
            .context("Failed to get Alice's account balance")?
    );
    println!(
        "Bob: {:?}",
        coin_client
            .get_account_balance(&bob.address())
            .await
            .context("Failed to get Bob's account balance")?
    );

    // Have Alice send Bob some coins.
    let txn_hash = coin_client
        .transfer(&mut alice, bob.address(), 1_000, None)
        .await
        .context("Failed to submit transaction to transfer coins")?;
    rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("Failed when waiting for the transfer transaction")?;

    // Print intermediate balances.
    println!("\n=== Intermediate Balances ===");
    println!(
        "Alice: {:?}",
        coin_client
            .get_account_balance(&alice.address())
            .await
            .context("Failed to get Alice's account balance the second time")?
    );
    println!(
        "Bob: {:?}",
        coin_client
            .get_account_balance(&bob.address())
            .await
            .context("Failed to get Bob's account balance the second time")?
    );

    // Have Alice send Bob some more coins.
    let txn_hash = coin_client
        .transfer(&mut alice, bob.address(), 1_000, None)
        .await
        .context("Failed to submit transaction to transfer coins")?;

    rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("Failed when waiting for the transfer transaction")?;

    // Print final balances.
    println!("\n=== Final Balances ===");
    println!(
        "Alice: {:?}",
        coin_client
            .get_account_balance(&alice.address())
            .await
            .context("Failed to get Alice's account balance the second time")?
    );
    println!(
        "Bob: {:?}",
        coin_client
            .get_account_balance(&bob.address())
            .await
            .context("Failed to get Bob's account balance the second time")?
    );

    Ok(())
}

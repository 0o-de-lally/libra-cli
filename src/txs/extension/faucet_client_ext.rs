use crate::txs::config::Config;
use aptos_sdk::rest_client::FaucetClient;
use once_cell::sync::Lazy;
use std::str::FromStr;
use url::Url;

pub trait FaucetClientExt {
    fn default() -> FaucetClient;
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

impl FaucetClientExt for FaucetClient {
    fn default() -> FaucetClient {
        let config = Config::default();
        let node_url = Url::from_str(&config.node_url).unwrap();
        FaucetClient::new(FAUCET_URL.clone(), node_url)
    }
}

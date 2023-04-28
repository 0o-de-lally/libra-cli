use crate::txs_core::extension::ed25519_private_key_ext::Ed25519PrivateKeyExt;
use anyhow::{Context, Result};
use aptos_sdk::crypto::{ed25519::Ed25519PrivateKey, ValidCryptoMaterialStringExt};
use indoc::formatdoc;
use libra_wallet::keys::validator_keygen;
use std::path::PathBuf;

pub async fn run(private_key: &str, output_dir: Option<PathBuf>) -> Result<String> {
    let private_key = if private_key.is_empty() {
        let (_, _, private_identity, _) = validator_keygen(output_dir)?;
        private_identity.account_private_key
    } else {
        Ed25519PrivateKey::from_encoded_string(private_key)
            .context(format!("Unable to decode the private key: {private_key}"))?
    };

    let account = private_key.get_account(Some(0)).await?;
    let private_key = hex::encode(account.private_key().to_bytes());
    let public_key = account.public_key();
    let authentication_key = account.authentication_key();
    let account_address = authentication_key.derived_address().to_hex_literal();

    Ok(formatdoc!(
        r#"
            ====================================
            Private key: {private_key}
            Public key: {public_key}
            Authentication key: {authentication_key}
            Account address: {account_address}
        "#
    ))
}

#[cfg(test)]
mod tests {
    use super::run;
    use anyhow::{bail, Result};
    use std::{fs, path::PathBuf};

    #[tokio::test]
    async fn generate_account_properly() -> Result<()> {
        let output_dir = "temp";
        let result = run("", Some(PathBuf::from(output_dir))).await.unwrap();
        let result = result.split("\n").collect::<Vec<_>>();

        let private_key = hex::decode(result[1].replace("Private key: ", "")).unwrap();
        assert_eq!(32, private_key.len());
        let public_key = hex::decode(result[2].replace("Public key: ", "")).unwrap();
        assert_eq!(32, public_key.len());
        let authentication_key =
            hex::decode(result[3].replace("Authentication key: ", "")).unwrap();
        assert_eq!(32, authentication_key.len());
        assert!(result[4].starts_with("Account address: 0x"));

        // Ensure yaml files exist
        for yaml_file in [
            "private-keys.yaml",
            "public-keys.yaml",
            "validator-full-node-identity.yaml",
            "validator-identity.yaml",
        ] {
            let path = format!("{output_dir}/{yaml_file}");
            if !fs::metadata(&path).is_ok() {
                // Clean up
                fs::remove_dir_all(output_dir).ok();
                // Stop the test with error
                bail!("File does not exist: {path}");
            }
        }

        // Clean up
        fs::remove_dir_all(output_dir).ok();
        Ok(())
    }

    #[tokio::test]
    async fn generate_account_from_private_key_properly() {
        let private_key = "c43f57994644ebda1eabfebf84def73fbd1d3ce442a9d2b2f4cb9f4da7b9908c";
        let result = run(private_key, None).await.unwrap();
        let result = result.split("\n").collect::<Vec<_>>();
        let expected_private_key = format!("Private key: {private_key}");
        let expected_public_key =
            "Public key: ef00c7b6f6246543445a847a6d136d293c107b05044f7fc105a063c93c50d7a0";
        let expected_auth_key =
            "Authentication key: fda03992f666875ddf854193fccd3e62ea111d066029490dd37c891ed9c3f880";
        let expected_account_address =
            "Account address: 0xfda03992f666875ddf854193fccd3e62ea111d066029490dd37c891ed9c3f880";

        assert_eq!(expected_private_key, result[1]);
        assert_eq!(expected_public_key, result[2]);
        assert_eq!(expected_auth_key, result[3]);
        assert_eq!(expected_account_address, result[4]);
    }
}

use crate::txs_core::{
    crypto::ed25519::Ed25519PrivateKey,
    types::{AccountKey, LocalAccount},
};
use anyhow::{bail, Context, Result};
use indoc::formatdoc;

pub fn run(private_key: &str) -> Result<String> {
    let account = if private_key.is_empty() {
        LocalAccount::generate(&mut rand::rngs::OsRng)
    } else {
        generate_account_from(private_key)?
    };

    let private_key = hex::encode(account.private_key().to_bytes());
    let public_key = account.public_key();
    let authentication_key = account.authentication_key();
    let account_address = authentication_key.derived_address().to_hex_literal();

    Ok(formatdoc!(
        r#"
            Private key: {private_key}
            Public key: {public_key}
            Authentication key: {authentication_key}
            Account address: {account_address}
        "#
    ))
}

fn generate_account_from(private_key: &str) -> Result<LocalAccount> {
    let private_key = hex::decode(private_key)
        .context(format!("Unable to decode the private key: {private_key}"))?;

    if private_key.len() != 32 {
        bail!(
            "Incorrect length of the private key: {} bytes (it should be 32 bytes)",
            private_key.len()
        )
    }

    let private_key = Ed25519PrivateKey::try_from(&private_key[..])?;
    let account_key = AccountKey::from_private_key(private_key);
    let account_address = account_key.authentication_key().derived_address();
    Ok(LocalAccount::new(account_address, account_key, 0))
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn generate_account_properly() {
        let result = run("").unwrap();
        let result = result.split("\n").collect::<Vec<_>>();

        let private_key = hex::decode(result[0].replace("Private key: ", "")).unwrap();
        assert_eq!(32, private_key.len());

        let public_key = hex::decode(result[1].replace("Public key: ", "")).unwrap();
        assert_eq!(32, public_key.len());

        let authentication_key =
            hex::decode(result[2].replace("Authentication key: ", "")).unwrap();
        assert_eq!(32, authentication_key.len());

        assert!(result[3].starts_with("Account address: 0x"));
    }

    #[test]
    fn generate_account_from_private_key_properly() {
        let private_key = "c43f57994644ebda1eabfebf84def73fbd1d3ce442a9d2b2f4cb9f4da7b9908c";
        let result = run(private_key).unwrap();
        let result = result.split("\n").collect::<Vec<_>>();
        let expected_private_key = format!("Private key: {private_key}");
        let expected_public_key =
            "Public key: ef00c7b6f6246543445a847a6d136d293c107b05044f7fc105a063c93c50d7a0";
        let expected_auth_key =
            "Authentication key: fda03992f666875ddf854193fccd3e62ea111d066029490dd37c891ed9c3f880";
        let expected_account_address =
            "Account address: 0xfda03992f666875ddf854193fccd3e62ea111d066029490dd37c891ed9c3f880";

        assert_eq!(expected_private_key, result[0]);
        assert_eq!(expected_public_key, result[1]);
        assert_eq!(expected_auth_key, result[2]);
        assert_eq!(expected_account_address, result[3]);
    }
}

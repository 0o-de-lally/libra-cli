use anyhow::Result;
use aptos::common::{
    init::InitTool,
    types::{CliCommand, ProfileOptions},
};
use clap::Parser;
use std::path::PathBuf;

pub async fn run(
    profile_options: &ProfileOptions,
    skip_faucet: bool,
    private_key_file: Option<PathBuf>,
    private_key: Option<String>,
) -> Result<()> {
    let mut input = vec![""];
    if let Some(profile) = &profile_options.profile {
        input.push("--profile");
        input.push(profile);
    }

    if skip_faucet {
        input.push("--skip-faucet");
    }

    if let Some(private_key_file) = &private_key_file {
        input.push("--private-key-file");
        input.push(private_key_file.to_str().unwrap_or_default());
    }

    if let Some(private_key) = &private_key {
        input.push("--private-key");
        input.push(private_key);
    }

    let init_tool = InitTool::parse_from(input);
    init_tool.execute().await?;
    Ok(())
}

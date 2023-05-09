use anyhow::{Context, Result};
use aptos::common::{init::InitTool, types::CliCommand};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    InitTool::parse()
        .execute()
        .await
        .context("Failed to init configuration!")
}

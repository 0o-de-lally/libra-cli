use crate::txs_core::client::Client;
use anyhow::Result;

pub async fn run(
    function_id: &str,
    type_args: Option<String>,
    args: Option<String>,
) -> Result<String> {
    let client = Client::default();
    let result = client
        .view(function_id, type_args, args)
        .await?
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();
    println!("\n=======OUTPUT=======");
    Ok(format!("[{}]", result.join(", ")))
}

use rmcp::transport::stdio;
use rmcp::ServiceExt;
use std::env;
use synthetic_web_search_mcp_rs::search::SearchService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = env::var("SYNTHETIC_API_KEY").map_err(|_| {
        anyhow::anyhow!("ERROR: SYNTHETIC_API_KEY environment variable is required")
    })?;

    let default_limit: usize = env::var("DEFAULT_SEARCH_LIMIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5);

    let search_service = SearchService::new(api_key, default_limit);

    let transport = stdio();

    let service = search_service.serve(transport).await?;
    service.waiting().await?;

    Ok(())
}

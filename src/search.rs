use std::time::Duration;

use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{InitializeResult, ServerCapabilities},
    schemars::{self, JsonSchema},
    tool, tool_handler, tool_router, ErrorData as McpError, Json, ServerHandler,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchRequest {
    #[schemars(description = "The search query")]
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchResponse {
    #[schemars(description = "The search results")]
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchResult {
    #[schemars(description = "The title of the search result")]
    pub title: String,
    #[schemars(description = "The URL of the search result")]
    pub url: String,
    #[schemars(description = "The text content of the search result")]
    pub text: String,
    #[schemars(description = "The published date of the search result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<String>,
}

pub struct SearchService {
    api_key: String,
    client: reqwest::Client,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl SearchService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Search the web using Synthetic API and return relevant results")]
    async fn search_web(
        &self,
        Parameters(SearchRequest { query }): Parameters<SearchRequest>,
    ) -> Result<Json<SearchResponse>, McpError> {
        if query.is_empty() {
            return Err(McpError::invalid_params(
                "Query parameter is required",
                None,
            ));
        }

        let response = self
            .client
            .post("https://api.synthetic.new/v2/search")
            .timeout(Duration::from_secs(15))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "query": query }))
            .send()
            .await;

        match response {
            Ok(resp) => {
                if !resp.status().is_success() {
                    let status = resp.status();
                    let text = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unable to read error response".to_string());
                    return Err(McpError::internal_error(
                        format!(
                            "API error: {} {} - {}",
                            status.as_u16(),
                            status.canonical_reason().unwrap_or("Unknown"),
                            text
                        ),
                        None,
                    ));
                }

                match resp.json::<SearchResponse>().await {
                    Ok(data) => Ok(Json(data)),
                    Err(e) => Err(McpError::parse_error(
                        format!("Failed to parse response: {}", e),
                        None,
                    )),
                }
            }
            Err(e) => Err(McpError::internal_error(
                format!("Request failed: {}", e),
                None,
            )),
        }
    }
}

#[tool_handler]
impl ServerHandler for SearchService {
    fn get_info(&self) -> InitializeResult {
        InitializeResult {
            protocol_version: rmcp::model::ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: rmcp::model::Implementation {
                name: "synthetic-web-search-mcp".into(),
                version: "0.1.0".into(),
                ..Default::default()
            },
            instructions: Some("Search the web using Synthetic API. Returns relevant search results with title, URL, content.".into()),
        }
    }
}

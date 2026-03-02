# Synthetic Web Search MCP Server (Rust)

A Rust implementation of the Model Context Protocol (MCP) server for web search using the Synthetic API.

## Overview

This MCP server provides AI assistants with web search capabilities through the Synthetic API. It allows OpenCode and other MCP-compatible clients to perform web searches and retrieve relevant results with metadata including URLs, titles, content snippets, publication dates.

## Installation

```bash
cargo build --release
```

This will create the binary at `./target/release/synthetic-web-search-mcp`.

## Usage with opencode

Add the following to your opencode MCP configuration:

```json
{
  "mcpServers": {
    "synthetic-web-search": {
      "command": "/path/to/synthetic-web-search-mcp",
      "env": {
        "SYNTHETIC_API_KEY": "your_api_key_here"
      }
    }
  }
}
```

## Testing with MCP Inspector

To test the server manually, use the MCP Inspector:

```bash
npx @modelcontextprotocol/inspector /path/to/your/binary
```

## Available Tools

### search_web

Search the web using the Synthetic API.

**Arguments:**

- `query` (String): The search query string
- `limit` (Option<usize>): Maximum number of results to return (default: 5)

**Returns:**
An array of search results with the following fields:

- `url` (String): The URL of the search result
- `title` (String): The title of the page
- `text` (String): A content snippet or description
- `published` (Option\<String\>): Publication date if available

## API Details

- **Endpoint:** https://api.synthetic.new/v2/search
- **Method:** POST
- **Authentication:** Bearer token (SYNTHETIC_API_KEY)

## Environment Variables

- `SYNTHETIC_API_KEY` (Required): Your Synthetic API key
- `DEFAULT_SEARCH_LIMIT` (Optional): Default maximum number of results to return (default: 5)

## Project Structure

```
synthetic-web-search-mcp-rs/
├── Cargo.toml              # Rust project manifest
├── src/
│   ├── main.rs            # Entry point and MCP server setup
│   ├── lib.rs             # Library exports
│   └── search.rs          # Search tool implementation
└── README.md              # This file
```

## License

See [LICENSE](LICENSE) for details.

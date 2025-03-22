# langfuse Rust Client

[![Crates.io](https://img.shields.io/crates/v/langfuse.svg)](https://crates.io/crates/langfuse)
[![Documentation](https://docs.rs/langfuse/badge.svg)](https://docs.rs/langfuse)
[![Github](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/adolfousier/langfuse-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A non official open-source Rust client for tracking simple interactions with Langfuse (https://langfuse.com) the open-source LLM observability platform. 

## Background

This Rust crate has been serving as a critical component in Neura AI's backend infrastructure (https://meetneura.ai), adopted across more than 20 applications over the past 8 months. After repeatedly duplicating this crate across various projects, I've decided to share this crate with the amazing Rust community by publishing it on crates.io, empowering developers to effectively monitor and optimize their LLM-based applications.

## Features

- Track user interactions with LangFuse
- Handle both successful and error cases
- Configure LangFuse API settings
- Support for custom trace names
- Track tokens used and model name
- Track processing time
- Flexible metadata and observation tracking

## Installation

Type this inside your project directory
````
cargo add langfuse
````

Or if you prefer to use a `Cargo.toml` file:
Add this to your `Cargo.toml`:

```toml
[dependencies]
Langfuse = "0.1.5"
```

## Usage

Basic example:

```rust
use langfuse::{LangFuseConfig, send_interaction};

#[tokio::main]
async fn main() {
    let config = LangFuseConfig::new(
        "your-public-key",
        "your-secret-key",
        "https://cloud.langfuse.com",
    );

    let result = send_interaction(
        &config,
        "request-id-123",
        Some("user-1"), 
        Some("session-1"),
        "Hello, who are you?", 
        "I'm an AI assistant, ready to help you",
        "here should be full raw response", 
        100, 
        false,
        Some("model-x"), 
        Some(TokenUsage {
            input_tokens: 100,
            output_tokens: 900,
            total_tokens: 1000,
        }),                           // Detailed token usage
        Some("json_endpoint_request_trace"), 
    ).await;

    match result {
        Ok(_) => println!("Successfully tracked interaction"),
        Err(e) => eprintln!("Error tracking interaction: {}", e),
    }
}
```

## API Documentation

### `LangFuseConfig`

Configuration for LangFuse API credentials and endpoint.

```rust
pub struct LangFuseConfig {
    pub public_key: String,
    pub secret_key: String,
    pub base_url: String,
}

impl LangFuseConfig {
    pub fn new(public_key: &str, secret_key: &str, base_url: &str) -> Self;
}
```

### `send_interaction`

Track an interaction with LangFuse.

```rust
pub async fn send_interaction(
    config: &LangFuseConfig,
    request_id: &str,
    user_id: Option<&str>,
    session_id: Option<&str>,
    input: &str,
    output: &str,
    raw_response: Option<&str>,
    processing_time_ms: u128,
    is_error: bool,
    model_name: Option<&str>,
    token_usage: Option<TokenUsage>,
    trace_name: Option<&str>,
) -> Result<(), LangFuseTrackerError>;

// TokenUsage structure for detailed token tracking
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}
```

### `LangFuseTrackerError`

Error types for LangFuse tracking operations.

```rust
pub enum LangFuseTrackerError {
    InvalidCredentials,
    NetworkError(reqwest::Error),
    InvalidResponseFormat,
    JsonParseError(serde_json::Error),
    InvalidTimestampFormat,
    InvalidBaseUrlFormat,
    Unknown(String),
}
```

## Project Structure

```
langfuse_tracker/
├── src/
│   ├── lib.rs
│   ├── client.rs
│   ├── error.rs
│   ├── types.rs
│   └── utils/
│       └── mod.rs
├── example/
│   └── usage.rs
├── tests/
│   └── integration.rs
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.
If you have any questions or suggestions, please contact open an issue or a pull request on https://github.com/adolfousier/langfuse-rust

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
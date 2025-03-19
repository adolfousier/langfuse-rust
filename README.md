# langfuse_tracker

[![Crates.io](https://img.shields.io/crates/v/langfuse_tracker.svg)](https://crates.io/crates/langfuse-rust)
[![Documentation](https://docs.rs/langfuse_tracker/badge.svg)](https://docs.rs/langfuse-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust client for tracking interactions with LangFuse, the open-source LLM observability platform.

## Features

- Track user interactions with LangFuse
- Handle both successful and error cases
- Configure LangFuse API settings
- Support for custom trace names
- Flexible metadata and observation tracking

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
log = "0.4"
chrono = "0.4"
base64 = "0.22.1"
thiserror = "1.0"

[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
```

## Usage

Basic example:

```rust
use langfuse_tracker::{LangFuseConfig, send_interaction};

#[tokio::main]
async fn main() {
    let config = LangFuseConfig::new(
        "your-public-key",
        "your-secret-key",
        "https://cloud.langfuse.com",
    );

    let result = send_interaction(
        &config,
        "request-123",
        Some("user-1"),
        Some("session-1"),
        "Hello, how can I help you?",
        "I'm here to help!",
        None,
        100,
        false,
        Some("model-x"),
        Some(1000),
        None,
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
    tokens_used: Option<u32>,
    trace_name: Option<&str>,
) -> Result<(), LangFuseTrackerError>;
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
├── examples/
│   └── simple.rs
├── tests/
│   └── integration.rs
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
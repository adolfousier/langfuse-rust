# LangFuse Rust

A Rust client for tracking interactions with LangFuse.

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
langfuse_rs = "0.1.0"
```

## Usage
use langfuse_rs::{LangFuseConfig, send_interaction};

#[tokio::main]
async fn main() {
    let config = LangFuseConfig::new(
        "your-public-key",
        "your-secret-key",
        "https://your-langfuse-instance.com",
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


langfuse_tracker/
├── Cargo.toml
├── Cargo.lock
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
├── README.md
└── CONTRIBUTING.md

[package]
name = "langfuse_tracker"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A Rust client for tracking interactions with LangFuse."
license = "MIT"
repository = "https://github.com/yourusername/langfuse_tracker"

[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
log = "0.4"
chrono = "0.4"
base64 = "0.13"
thiserror = "1.0"

[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LangFuseTrackerError {
    #[error("Invalid API credentials")]
    InvalidCredentials,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid response format")]
    InvalidResponseFormat,

    #[error("Error parsing JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Invalid timestamp format")]
    InvalidTimestampFormat,

    #[error("Invalid base URL format")]
    InvalidBaseUrlFormat,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl LangFuseTrackerError {
    pub fn unknown(error: impl Into<String>) -> Self {
        Self::Unknown(error.into())
    }
}

use serde::Serialize;
use chrono::Utc;

#[derive(Debug, Serialize)]
pub struct LangFuseConfig {
    pub public_key: String,
    pub secret_key: String,
    pub base_url: String,
}

impl LangFuseConfig {
    pub fn new(public_key: &str, secret_key: &str, base_url: &str) -> Self {
        Self {
            public_key: public_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InteractionMetadata {
    pub processing_time_ms: u128,
    pub status: String,
    pub model: String,
    pub tokens: Option<u32>,
    pub raw_response: Option<String>,
    pub timestamp_utc: String,
}

impl InteractionMetadata {
    pub fn new(
        processing_time_ms: u128,
        is_error: bool,
        model_name: Option<&str>,
        tokens_used: Option<u32>,
        raw_response: Option<&str>,
    ) -> Self {
        Self {
            processing_time_ms,
            status: if is_error { "error" } else { "success" }.to_string(),
            model: model_name.unwrap_or("unknown").to_string(),
            tokens: tokens_used,
            raw_response: raw_response.map(|s| s.to_string()),
            timestamp_utc: Utc::now().to_rfc3339(),
        }
    }
}

use super::{
    error::LangFuseTrackerError,
    types::{LangFuseConfig, InteractionMetadata},
};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use log::{debug, error};
use chrono::Utc;

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
) -> Result<(), LangFuseTrackerError> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();

    let auth_string = format!("{}:{}", config.public_key, config.secret_key);
    let auth_header = format!("Basic {}", STANDARD.encode(auth_string));
    
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&auth_header)?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let now = Utc::now();
    let event_id = format!("{}-event", request_id);
    
    let trace_name = trace_name.unwrap_or("neura_rda_user_interaction");

    let metadata = InteractionMetadata::new(
        processing_time_ms,
        is_error,
        model_name,
        tokens_used,
        raw_response,
    );

    let batch_payload = json!({
        "batch": [{
            "id": event_id,
            "timestamp": now.to_rfc3339(),
            "type": "trace-create",
            "body": {
                "id": request_id,
                "timestamp": now.to_rfc3339(),
                "name": trace_name,
                "userId": user_id.unwrap_or_default(),
                "sessionId": session_id.unwrap_or_default(),
                "input": input,
                "output": output,
                "metadata": metadata,
                "observations": [
                    {
                        "type": "generation",
                        "id": format!("{}-generation", request_id),
                        "startTime": now.checked_sub_signed(chrono::Duration::milliseconds(processing_time_ms as i64))
                            .unwrap_or(now)
                            .to_rfc3339(),
                        "endTime": now.to_rfc3339(),
                        "model": model_name.unwrap_or("unknown").to_string(),
                        "input": input,
                        "output": output,
                        "metadata": {
                            "request_id": request_id,
                            "latency_ms": processing_time_ms,
                            "tokens": tokens_used,
                            "raw_response": raw_response
                        }
                    }
                ]
            }
        }]
    });

    let langfuse_url = format!(
        "{}/api/public/ingestion",
        config.base_url.trim_end_matches('/')
    );

    debug!("Langfuse URL: {}", langfuse_url);
    debug!("Sending trace payload: {}", serde_json::to_string_pretty(&batch_payload).unwrap());

    let response = client
        .post(&langfuse_url)
        .headers(headers)
        .json(&batch_payload)
        .send()
        .await?;

    match response.status() {
        status if status.is_success() => {
            debug!("Successfully sent trace to Langfuse for request_id: {}", request_id);
            Ok(())
        },
        status if status.as_u16() == 207 => {
            let error_text = response.text().await?;
            error!("Langfuse partial success with errors: {}", error_text);
            Err(LangFuseTrackerError::unknown(error_text))
        },
        _ => {
            let error_text = response.text().await?;
            error!("Langfuse API error: {}", error_text);
            Err(LangFuseTrackerError::unknown(error_text))
        }
    }
}

pub use self::client::send_interaction;
pub use self::error::LangFuseTrackerError;
pub use self::types::{LangFuseConfig, InteractionMetadata};

#[macro_use]
extern crate log;

use langfuse_tracker::{LangFuseConfig, send_interaction};

#[tokio::main]
async fn main() {
    let config = LangFuseConfig::new(
        "your-public-key",
        "your-secret-key",
        "https://your-langfuse-instance.com",
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

#[tokio::test]
async fn test_send_interaction_success() {
    let config = LangFuseConfig::new(
        "test-public-key",
        "test-secret-key",
        "http://localhost:8080",
    );

    let result = send_interaction(
        &config,
        "test-request",
        Some("test-user"),
        Some("test-session"),
        "Test input",
        "Test output",
        None,
        50,
        false,
        Some("test-model"),
        Some(500),
        None,
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_send_interaction_error() {
    let config = LangFuseConfig::new(
        "invalid-public-key",
        "invalid-secret-key",
        "http://localhost:8080",
    );

    let result = send_interaction(
        &config,
        "error-request",
        Some("error-user"),
        Some("error-session"),
        "Error input",
        "Error output",
        None,
        100,
        true,
        Some("error-model"),
        Some(1000),
        None,
    ).await;

    assert!(result.is_err());
}

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on contributing to this crate.

## License

Licensed under the MIT license.

# Contributing to LangFuse Tracker

We welcome contributions of all kinds! Please read through this guide to get started.

## Setup

1. Clone the repository:

2. Install dependencies:

## Running Tests

Run the tests with:

## Submitting a Pull Request

1. Fork the repository.
2. Create a new branch:

3. Commit your changes with meaningful commit messages.
4. Push to the branch:

5. Open a Pull Request against the `main` branch.

## Code of Conduct

- Be respectful and considerate in your interactions.
- Follow Rust coding conventions.
- Include tests with your changes.
- Ensure documentation is updated with any API changes.

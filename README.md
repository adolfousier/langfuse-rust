# langfuse-rust

A Rust client for the [Langfuse](https://langfuse.com) observability platform.

## Features

- Async/await support
- Type-safe API
- Comprehensive error handling
- Batch request support
- Full support for Langfuse's data model
- Logging integration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
langfuse-rust = "0.1.0"
```

## Usage

First, set up your environment variables:

```bash
export LANGFUSE_BASE_URL="https://cloud.langfuse.com"
export LANGFUSE_PUBLIC_KEY="your-public-key"
export LANGFUSE_SECRET_KEY="your-secret-key"
```

Then, use the client in your code:

```rust
use langfuse_rust::{LangfuseClient, LangfuseApi, ObservationLevel};
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LangfuseClient::new()?;

    // Create a trace
    let trace_id = client.create_trace(
        "example-trace",
        Some(serde_json::json!({
            "environment": "production"
        }))
    ).await?;

    // Track a span
    client.track_span(
        "example-span",
        Utc::now(),
        Utc::now(),
        ObservationLevel::Default,
        Some(serde_json::json!({
            "duration_ms": 150
        }))
    ).await?;

    Ok(())
}
```

## Examples

Check out the `examples` directory for more usage examples.

## API Documentation

For detailed API documentation, run:

```bash
cargo doc --open
```

## Features

- [ ] Trace creation and management
- [x] Span tracking
- [ ] Generation tracking
- [ ] Score tracking
- [ ] Event tracking
- [x] Batch processing
- [x] Error handling
- [x] Logging integration

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
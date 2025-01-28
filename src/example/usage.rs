// examples/usage.rs
use langfuse_rust::{LangfuseClient, LangfuseApi, ObservationLevel};
use chrono::Utc;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    pretty_env_logger::init();

    // Create client
    let client = LangfuseClient::new()?;

    // Create a trace
    let trace_id = client.create_trace(
        "example-trace",
        Some(serde_json::json!({
            "environment": "production",
            "version": "1.0.0"
        }))
    ).await?;

    println!("Created trace: {}", trace_id);

    // Track a span with all fields
    client.track_span(
        "example-span",
        Some(trace_id.clone()),
        Utc::now(),
        Utc::now(),
        ObservationLevel::Default,
        Some(serde_json::json!({
            "duration_ms": 150,
            "status": "success"
        })),
        Some(serde_json::json!({
            "query": "What is the weather?",
        })),
        Some(serde_json::json!({
            "response": "The weather is sunny.",
        })),
        None,  // parent_observation_id
        Some("1.0.0".to_string()),  // version
    ).await?;

    Ok(())
}
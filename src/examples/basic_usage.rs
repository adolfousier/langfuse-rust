use langfuse_rs::{Langfuse, Observation};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Langfuse::new(
        "your_public_key".to_string(),
        "your_secret_key".to_string(),
    );

    // Create a new trace
    let trace_id = client.create_trace("example-trace").await?;

    // Create an observation
    let observation = Observation {
        name: "example-observation".to_string(),
        trace_id: trace_id.clone(),
        start_time: Some(chrono::Utc::now().to_rfc3339()),
        end_time: None,
        metadata: Some(serde_json::json!({
            "model": "gpt-4",
            "temperature": 0.7
        })),
    };

    client.create_observation(observation).await?;
    
    Ok(())
}
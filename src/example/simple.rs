use langfuse_tracker::{LangFuseConfig, send_interaction};

#[tokio::main]
async fn main() {
    // Configure LangFuseTracker with your credentials and LangFuse instance URL
    let config = LangFuseConfig::new(
        "your-public-key",      // Your LangFuse public API key
        "your-secret-key",      // Your LangFuse private API secret
        "https://your-langfuse-instance.com",  // Base URL of your LangFuse instance
    );

    // Track an interaction with detailed parameters
    let result = send_interaction(
        &config,                      // Configuration containing API credentials
        "request-123",                // Unique identifier for the request
        Some("user-1"),                // Optional user identifier
        Some("session-1"),            // Optional session identifier
        "Hello, how can I help you?",  // Input text from the user
        "I'm here to help!",           // Output response
        None,                         // No raw response included
        100,                          // Processing time in milliseconds
        false,                        // Indicates the request was successful (not an error)
        Some("model-x"),               // Optional name of the model used
        Some(1000),                    // Optional number of tokens used
        None,                         // No custom trace name provided
    ).await;

    // Handle the result to check if the interaction was successfully tracked
    match result {
        Ok(_) => println!("Successfully tracked interaction"),
        Err(e) => eprintln!("Error tracking interaction: {}", e),
    }
}
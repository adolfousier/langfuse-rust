// example/usage.rs
use langfuse::{LangFuseConfig, TokenUsage, send_interaction};
use chrono::Utc;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging for better debugging
    pretty_env_logger::init();

    // Step 1: Configure the LangFuse client with your credentials
    // Replace these with your actual LangFuse API credentials and instance URL
    let config = LangFuseConfig::new(
        "your-public-key",      // Your LangFuse public API key
        "your-secret-key",      // Your LangFuse private API secret
        "https://your-langfuse-instance.com",  // Base URL of your LangFuse instance
    );

    // Step 2: Track a user interaction with detailed metadata
    // This is a comprehensive example that demonstrates all available parameters
    let result = send_interaction(
        &config,                      // Configuration containing API credentials
        "request-id-123",                // Unique identifier for the request
        Some("user-1"),                // Optional user identifier
        Some("session-1"),            // Optional session identifier
        "Hello, who are you?",  // Input text from the user
        "I'm an AI assistant, ready to help you",           // Output response
        "here should be full raw response",                         // No raw response included
        6500,                          // Processing time in milliseconds
        false,                        // Indicates the request was successful (not an error), if tracking error this must be true
        Some("model-x"),               // Optional name of the model used
        Some(TokenUsage {               
            input_tokens: 200,
            output_tokens: 800,
            total_tokens: 1000,
        }),                                         // Optional detailed token usage
        Some("json_endpoint_request_trace"),              // Custom trace name provided
    ).await;

    // Step 3: Handle the result to check if the interaction was successfully tracked
    match result {
        Ok(_) => println!("Successfully tracked interaction"),
        Err(e) => eprintln!("Error tracking interaction: {}", e),
    }

    // Additional examples of tracking different types of interactions
    // -----------------------------------------------------------------
    
    // Example: Tracking a failed request
    let result_failed = send_interaction(
        &config,
        "request-456",
        Some("user-2"),
        Some("session-2"),
        "Invalid input",
        "Request failed due to invalid input",
        None,
        3200,
        true,  // Set to true to indicate an error
        Some("model-y"),
        Some(TokenUsage {
            input_tokens: 50,
            output_tokens: 450,
            total_tokens: 500,
        }),
        Some("error-handling"),  // Custom trace name for error tracking
    ).await;

    match result_failed {
        Ok(_) => println!("Successfully tracked failed interaction"),
        Err(e) => eprintln!("Error tracking failed interaction: {}", e),
    }

    // Example: Tracking a successful response with custom metadata
    let result_success = send_interaction(
        &config,
        "request-789",
        Some("user-3"),
        Some("session-3"),
        "What is the weather like?",
        "The weather is sunny.",
        None,
        150,
        false,
        Some("model-z"),
        Some(TokenUsage {
            input_tokens: 150,
            output_tokens: 600,
            total_tokens: 750,
        }),
        Some("weather-forecast"),  // Custom trace name for weather forecasts
    ).await;

    match result_success {
        Ok(_) => println!("Successfully tracked successful interaction"),
        Err(e) => eprintln!("Error tracking successful interaction: {}", e),
    }

    // Example: Minimal usage - Tracking an interaction with basic parameters
    let result_minimal = send_interaction(
        &config,
        "request-101",
        None,  // No user ID
        None,  // No session ID
        "Hello, world!",
        "Hello from LangFuse!",
        None,
        5340,
        false,
        None,  // No model name
        None,  // No token count
        None,  // No detailed token usage
        None,  // No custom trace name
    ).await;

    match result_minimal {
        Ok(_) => println!("Successfully tracked minimal interaction"),
        Err(e) => eprintln!("Error tracking minimal interaction: {}", e),
    }

    // Example: Tracking an error condition with custom metadata
    let result_error = send_interaction(
        &config,
        "request-202",
        Some("user-4"),
        Some("session-4"),
        "Invalid request format",
        "Request could not be processed",
        None,
        5300,
        true,  // Set to true to indicate an error
        Some("model-a"),
        Some(TokenUsage {
            input_tokens: 50,
            output_tokens: 200,
            total_tokens: 250,
        }),
        Some("validation-error"),  
    ).await;

    match result_error {
        Ok(_) => println!("Successfully tracked error condition"),
        Err(e) => eprintln!("Error tracking error condition: {}", e),
    }

    // Example: Tracking a streaming response interaction
    let result_stream = send_interaction(
        &config,
        "request-303",
        Some("user-5"),
        Some("session-5"),
        "What is the latest news?",
        "Streaming news updates...",
        None,
        2400,
        false,
        Some("model-b"),
        Some(TokenUsage {
            input_tokens: 200,
            output_tokens: 800,
            total_tokens: 1000,
        }),
        Some("streaming-response"),  // Custom trace name for streaming responses
    ).await;

    match result_stream {
        Ok(_) => println!("Successfully tracked streaming response interaction"),
        Err(e) => eprintln!("Error tracking streaming response interaction: {}", e),
    }

    // Example: Tracking a long-running operation
    let result_long = send_interaction(
        &config,
        "request-404",
        Some("user-6"),
        Some("session-6"),
        "Processing your request...",
        "Request completed successfully",
        None,
        21000,
        false,
        Some("model-c"),
        Some(TokenUsage {
            input_tokens: 500,
            output_tokens: 1500,
            total_tokens: 2000,
        }),
        Some("long-running-operation"),  // Custom trace name for long operations
    ).await;

    match result_long {
        Ok(_) => println!("Successfully tracked long-running operation"),
        Err(e) => eprintln!("Error tracking long-running operation: {}", e),
    }

    // Example: Tracking a request with custom metadata
    let result_custom = send_interaction(
        &config,
        "request-505",
        Some("user-7"),
        Some("session-7"),
        "What is machine learning?",
        "Machine learning is the study of algorithms...",
        None,
        2250,
        false,
        Some("model-d"),
        Some(TokenUsage {
            input_tokens: 300,
            output_tokens: 1200,
            total_tokens: 1500,
        }),
        Some("custom-metadata"),  
    ).await;

    match result_custom {
        Ok(_) => println!("Successfully tracked custom metadata request"),
        Err(e) => eprintln!("Error tracking custom metadata request: {}", e),
    }

    Ok(())
}
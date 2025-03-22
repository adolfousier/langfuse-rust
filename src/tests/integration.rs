#[tokio::test]
pub async fn test_send_interaction_success() {
    let config = LangFuseConfig::new(
        "your-test-public-key",
        "your-test-secret-key",
        "https://cloud.langfuse.com",
    );

    let token_usage = TokenUsage {
        input_tokens: 100,
        output_tokens: 400,
        total_tokens: 500
    };

    let result = send_interaction(
        &config,
        "test-request",
        Some("test-user"),
        Some("test-session"),
        "user query",
        "ai response",
        None,
        50,
        false,
        Some("test-model"),
        Some(token_usage),
        Some("trace-name"),
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
pub async fn test_send_interaction_error() {
    let config = LangFuseConfig::new(
        "invalid-public-key",
        "invalid-secret-key",
        "https://cloud.langfuse.com",
    );

    let token_usage = TokenUsage {
        input_tokens: 200,
        output_tokens: 800,
        total_tokens: 1000
    };

    let result = send_interaction(
        &config,
        "error-request",
        Some("error-user"),
        Some("error-session"),
        "user query",
        "Error response",
        None,
        100,
        true,
        Some("error-model"),
        Some(token_usage),
        Some("trace-name")
    ).await;

    assert!(result.is_err());
}
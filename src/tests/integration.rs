#[tokio::test]
pub async fn test_send_interaction_success() {
    let config = LangFuseConfig::new(
        "your-test-public-key",
        "your-test-secret-key",
        "https://cloud.langfuse.com",
    );

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
        Some(500),
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
        Some(1000),
        Some("trace-name")    ).await;

    assert!(result.is_err());
}
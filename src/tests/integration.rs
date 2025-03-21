#[tokio::test]
pub async fn test_send_interaction_success() {
    let config = LangFuseConfig::new(
        "test-public-key",
        "test-secret-key",
        "http://localhost:9090",
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
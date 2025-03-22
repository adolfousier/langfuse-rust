use serde::Serialize;
use chrono::Utc;

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Serialize)]
pub struct InteractionMetadata {
    pub processing_time_ms: u128,
    pub status: String,
    pub model: String,
    pub token_usage: Option<TokenUsage>,
    pub raw_response: Option<String>,
    pub timestamp_utc: String,
}

impl InteractionMetadata {
    pub fn new(
        processing_time_ms: u128,
        is_error: bool,
        model_name: Option<&str>,
        token_usage: Option<&TokenUsage>,
        raw_response: Option<&str>,
    ) -> Self {
        Self {
            processing_time_ms,
            status: if is_error { "error" } else { "success" }.to_string(),
            model: model_name.unwrap_or("unknown").to_string(),
            token_usage: token_usage.cloned(),
            raw_response: raw_response.map(|s| s.to_string()),
            timestamp_utc: Utc::now().to_rfc3339(),
        }
    }
}
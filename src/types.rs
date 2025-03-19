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
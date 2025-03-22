// src/client.rs
use super::{
    error::LangFuseTrackerError,
    types::{LangFuseConfig, InteractionMetadata, TokenUsage},
};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use log::{debug, error};
use chrono::Utc;

pub async fn send_interaction(
    config: &LangFuseConfig,
    request_id: &str,
    user_id: Option<&str>,
    session_id: Option<&str>,
    input: &str,
    output: &str,
    raw_response: Option<&str>,
    processing_time_ms: u128,
    is_error: bool,
    model_name: Option<&str>,
    token_usage: Option<TokenUsage>,
    trace_name: Option<&str>,
) -> Result<(), LangFuseTrackerError> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();

    let auth_string = format!("{}:{}", config.public_key, config.secret_key);
    let auth_header = format!("Basic {}", STANDARD.encode(auth_string));
    
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&auth_header)?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let now = Utc::now();
    let event_id = format!("{}-event", request_id);
    
    let trace_name = trace_name.unwrap_or("your_app_user_interaction");

    // Create a local token_usage variable to avoid cloning
    let token_usage_local = token_usage;
    
    let metadata = InteractionMetadata::new(
        processing_time_ms,
        is_error,
        model_name,
        token_usage_local.as_ref(),
        raw_response,
    );

    let batch_payload = json!({
        "batch": [{
            "id": event_id,
            "timestamp": now.to_rfc3339(),
            "type": "trace-create",
            "body": {
                "id": request_id,
                "timestamp": now.to_rfc3339(),
                "name": trace_name,
                "userId": user_id.unwrap_or_default(),
                "sessionId": session_id.unwrap_or_default(),
                "input": input,
                "output": output,
                "metadata": metadata,
                "observations": [
                    {
                        "type": "generation",
                        "id": format!("{}-generation", request_id),
                        "startTime": now.checked_sub_signed(chrono::Duration::milliseconds(processing_time_ms as i64))
                            .unwrap_or(now)
                            .to_rfc3339(),
                        "endTime": now.to_rfc3339(),
                        "model": model_name.unwrap_or("unknown").to_string(),
                        "input": input,
                        "output": output,
                        "metadata": {
                            "request_id": request_id,
                            "latency_ms": processing_time_ms,
                            "token_usage": token_usage_local.as_ref().map(|t| {
                                json!({
                                    "input_tokens": t.input_tokens,
                                    "output_tokens": t.output_tokens,
                                    "total_tokens": t.total_tokens
                                })
                            }),
                            "raw_response": raw_response
                        }
                    }
                ]
            }
        }]
    });

    let langfuse_url = format!(
        "{}/api/public/ingestion",
        config.base_url.trim_end_matches('/')
    );

    debug!("Langfuse URL: {}", langfuse_url);
    debug!("Sending trace payload: {}", serde_json::to_string_pretty(&batch_payload).unwrap());

    let response = client
        .post(&langfuse_url)
        .headers(headers)
        .json(&batch_payload)
        .send()
        .await?;

    match response.status() {
        status if status.is_success() => {
            debug!("Successfully sent trace to Langfuse for request_id: {}", request_id);
            Ok(())
        },
        status if status.as_u16() == 207 => {
            let error_text = response.text().await?;
            error!("Langfuse partial success with errors: {}", error_text);
            Err(LangFuseTrackerError::unknown(error_text))
        },
        _ => {
            let error_text = response.text().await?;
            error!("Langfuse API error: {}", error_text);
            Err(LangFuseTrackerError::unknown(error_text))
        }
    }
}

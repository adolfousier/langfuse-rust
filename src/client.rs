// src/client.rs
use crate::{types::*, error::LangfuseError};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::{debug, error, info};
use reqwest::Client;
use serde_json::Value;
use std::env;
use uuid::Uuid;

#[async_trait]
pub trait LangfuseApi {
    async fn track_span(
        &self,
        name: &str,
        trace_id: Option<String>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        level: ObservationLevel,
        metadata: Option<Value>,
        input: Option<Value>,
        output: Option<Value>,
        parent_observation_id: Option<String>,
        version: Option<String>,
    ) -> Result<(), LangfuseError>;
    
    async fn create_trace(
        &self,
        name: &str,
        metadata: Option<Value>,
    ) -> Result<String, LangfuseError>;
}

pub struct LangfuseClient {
    client: Client,
    base_url: String,
    public_key: String,
    secret_key: String,
}

impl LangfuseClient {
    pub fn new() -> Result<Self, LangfuseError> {
        let base_url = env::var("LANGFUSE_BASE_URL")
            .map_err(|_| LangfuseError::EnvError("LANGFUSE_BASE_URL".to_string()))?;
            
        let public_key = env::var("LANGFUSE_PUBLIC_KEY")
            .map_err(|_| LangfuseError::EnvError("LANGFUSE_PUBLIC_KEY".to_string()))?;
            
        let secret_key = env::var("LANGFUSE_SECRET_KEY")
            .map_err(|_| LangfuseError::EnvError("LANGFUSE_SECRET_KEY".to_string()))?;

        Ok(Self {
            client: Client::new(),
            base_url,
            public_key,
            secret_key,
        })
    }
    
    pub fn with_credentials(
        base_url: String,
        public_key: String,
        secret_key: String,
    ) -> Self {
        Self {
            client: Client::new(),
            base_url,
            public_key,
            secret_key,
        }
    }
}

#[async_trait]
impl LangfuseApi for LangfuseClient {
    async fn track_span(
        &self,
        name: &str,
        trace_id: Option<String>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        level: ObservationLevel,
        metadata: Option<Value>,
        input: Option<Value>,
        output: Option<Value>,
        parent_observation_id: Option<String>,
        version: Option<String>,
    ) -> Result<(), LangfuseError> {
        let event = LangfuseEvent {
            event_type: EventType::SpanCreate,
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            metadata: None,
            body: EventBody::Span(SpanBody {
                id: Some(Uuid::new_v4().to_string()),
                trace_id,
                name: Some(name.to_string()),
                start_time,
                end_time: Some(end_time),
                metadata,
                input,
                output,
                level,
                status_message: None,
                parent_observation_id,
                version,
            }),
        };
        self.send_batch(vec![event]).await
    }

    async fn create_trace(
        &self,
        name: &str,
        metadata: Option<Value>,
    ) -> Result<String, LangfuseError> {
        let trace_id = Uuid::new_v4().to_string();
        let event = LangfuseEvent {
            event_type: EventType::TraceCreate,
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            metadata: None,
            body: EventBody::Trace(TraceBody {
                id: Some(trace_id.clone()),
                name: Some(name.to_string()),
                timestamp: Some(Utc::now()),
                metadata,
                user_id: None,
                session_id: None,
                version: None,
                public: None,
                // Added missing required fields
                input: None,
                output: None,
                release: None,
                tags: None,
            }),
        };
        self.send_batch(vec![event]).await?;
        Ok(trace_id)
    }
}

impl LangfuseClient {
    async fn send_batch(&self, batch: Vec<LangfuseEvent>) -> Result<(), LangfuseError> {
        let response = self.client
            .post(format!("{}/api/public/ingestion", self.base_url))
            .basic_auth(&self.public_key, Some(&self.secret_key))
            .json(&serde_json::json!({ "batch": batch }))
            .send()
            .await?;

        let status = response.status();
        
        if status.is_success() || status.as_u16() == 207 {
            let response_body: Value = response.json().await?;
            
            if let Some(errors) = response_body.get("errors").and_then(|e| e.as_array()) {
                if !errors.is_empty() {
                    return Err(LangfuseError::ApiError {
                        status_code: 207,
                        message: format!("Batch partially failed: {:?}", errors),
                    });
                }
            }
            
            Ok(())
        } else {
            Err(LangfuseError::ApiError {
                status_code: status.as_u16(),
                message: format!("Request failed: {}", status),
            })
        }
    }
}

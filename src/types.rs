// src/types.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum EventType {
    TraceCreate,
    SpanCreate,
    SpanUpdate,
    GenerationCreate,
    GenerationUpdate,
    EventCreate,
    ObservationCreate,
    ObservationUpdate,
    ScoreCreate,
    SdkLog,
}

#[derive(Debug, Serialize, Clone)]
pub struct LangfuseEvent {
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub id: String,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub body: EventBody,
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum EventBody {
    Trace(TraceBody),
    Span(SpanBody),
    Generation(GenerationBody),
    Event(EventBody),
    Observation(ObservationBody),
    Score(ScoreBody),
    SdkLog(SdkLogBody),
}

#[derive(Debug, Serialize, Clone)]
pub struct TraceBody {
    pub id: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SpanBody {
    pub id: Option<String>,
    pub trace_id: Option<String>,
    pub name: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    pub level: ObservationLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_observation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct GenerationBody {
    pub id: Option<String>,
    pub trace_id: Option<String>,
    pub name: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_start_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_parameters: Option<HashMap<String, ModelParameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    pub level: ObservationLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_observation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_details: Option<UsageDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_details: Option<HashMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_version: Option<i32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct EventBody {
    pub id: Option<String>,
    pub trace_id: Option<String>,
    pub name: Option<String>,
    pub start_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    pub level: ObservationLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_observation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ObservationBody {
    pub id: Option<String>,
    pub trace_id: Option<String>,
    #[serde(rename = "type")]
    pub observation_type: ObservationType,
    pub name: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_start_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_parameters: Option<HashMap<String, ModelParameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    pub level: ObservationLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_observation_id: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ScoreBody {
    pub id: Option<String>,
    pub trace_id: String,
    pub name: String,
    pub value: ScoreValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub data_type: ScoreDataType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SdkLogBody {
    pub log: serde_json::Value,
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum ModelParameter {
    String(Option<String>),
    Integer(Option<i64>),
    Boolean(Option<bool>),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum ScoreValue {
    Numeric(f64),
    String(String),
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ScoreDataType {
    Numeric,
    Boolean,
    Categorical,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ObservationType {
    Span,
    Generation,
    Event,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ObservationLevel {
    Debug,
    Default,
    Warning,
    Error,
}

#[derive(Debug, Serialize, Clone)]
pub struct Usage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    pub unit: ModelUsageUnit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum UsageDetails {
    Generic(HashMap<String, i32>),
    OpenAI(OpenAIUsage),
}

#[derive(Debug, Serialize, Clone)]
pub struct OpenAIUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<HashMap<String, i32>>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ModelUsageUnit {
    Characters,
    Tokens,
    Milliseconds,
    Seconds,
    Images,
    Requests,
}

// Response types for API calls
#[derive(Debug, Deserialize)]
pub struct IngestionResponse {
    pub successes: Vec<IngestionResult>,
    pub errors: Vec<IngestionError>,
}

#[derive(Debug, Deserialize)]
pub struct IngestionResult {
    pub id: String,
    pub status: i32,
}

#[derive(Debug, Deserialize)]
pub struct IngestionError {
    pub id: String,
    pub status: i32,
    pub message: Option<String>,
    pub error: Option<serde_json::Value>,
}
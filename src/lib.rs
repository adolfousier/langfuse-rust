use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Langfuse {
    client: Client,
    public_key: String,
    secret_key: String,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Observation {
    pub name: String,
    pub trace_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Langfuse {
    pub fn new(public_key: String, secret_key: String) -> Self {
        Self {
            client: Client::new(),
            public_key,
            secret_key,
            base_url: "https://cloud.langfuse.com/api".to_string(),
        }
    }

    pub async fn create_trace(&self, name: &str) -> Result<String, Box<dyn Error>> {
        let trace_id = uuid::Uuid::new_v4().to_string();
        let url = format!("{}/traces", self.base_url);
        
        let response = self.client
            .post(&url)
            .basic_auth(&self.public_key, Some(&self.secret_key))
            .json(&serde_json::json!({
                "name": name,
                "id": trace_id,
            }))
            .send()
            .await?;

        response.error_for_status()?;
        Ok(trace_id)
    }

    pub async fn create_observation(&self, observation: Observation) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/observations", self.base_url);
        
        let response = self.client
            .post(&url)
            .basic_auth(&self.public_key, Some(&self.secret_key))
            .json(&observation)
            .send()
            .await?;

        response.error_for_status()?;
        Ok(())
    }
}
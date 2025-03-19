use thiserror::Error;

#[derive(Error, Debug)]
pub enum LangFuseTrackerError {
    #[error("Invalid API credentials")]
    InvalidCredentials,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid response format")]
    InvalidResponseFormat,

    #[error("Error parsing JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Invalid timestamp format")]
    InvalidTimestampFormat,

    #[error("Invalid base URL format")]
    InvalidBaseUrlFormat,

    #[error("Header value error: {0}")]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl LangFuseTrackerError {
    pub fn unknown(error: impl Into<String>) -> Self {
        Self::Unknown(error.into())
    }
}
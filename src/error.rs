// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LangfuseError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Environment variable not found: {0}")]
    EnvError(String),
    
    #[error("API error: {status_code} - {message}")]
    ApiError {
        status_code: u16,
        message: String,
    },
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}
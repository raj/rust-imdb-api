use thiserror::Error;

/// Error types for the IMDB API client
#[derive(Error, Debug)]
pub enum ImdbApiError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid IMDB ID format: {0}")]
    InvalidImdbId(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Title not found: {0}")]
    TitleNotFound(String),

    #[error("Authentication error")]
    AuthenticationError,

    #[error("Base64 encoding error: {0}")]
    Base64Error(#[from] base64::DecodeError),
}

pub type Result<T> = std::result::Result<T, ImdbApiError>;

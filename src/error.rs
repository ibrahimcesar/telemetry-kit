//! Error types for telemetry-kit

use thiserror::Error;

/// Result type for telemetry operations
pub type Result<T> = std::result::Result<T, TelemetryError>;

/// Errors that can occur during telemetry operations
#[derive(Debug, Error)]
pub enum TelemetryError {
    /// SQLite database error
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// HTTP request error
    #[cfg(feature = "sync")]
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded, retry after {retry_after} seconds")]
    RateLimitExceeded {
        /// Seconds to wait before retrying
        retry_after: u64,
    },

    /// Server error with status code
    #[error("Server error: {status} - {message}")]
    ServerError {
        /// HTTP status code
        status: u16,
        /// Error message from server
        message: String,
    },

    /// Maximum retries exceeded
    #[error("Maximum retries exceeded")]
    MaxRetriesExceeded,

    /// Invalid event schema
    #[error("Invalid event schema: {0}")]
    InvalidSchema(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Machine ID error
    #[error("Failed to get machine ID: {0}")]
    MachineId(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl TelemetryError {
    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            TelemetryError::RateLimitExceeded { .. } => true,
            TelemetryError::ServerError { status, .. } if *status >= 500 => true,
            _ => false,
        }
    }
}

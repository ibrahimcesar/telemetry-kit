//! Error types for telemetry-kit
//!
//! All errors include helpful context and actionable suggestions where possible.

use thiserror::Error;

/// Result type for telemetry operations
pub type Result<T> = std::result::Result<T, TelemetryError>;

/// Errors that can occur during telemetry operations
#[derive(Debug, Error)]
pub enum TelemetryError {
    /// SQLite database error
    ///
    /// Common causes:
    /// - Database file is locked (another process using it)
    /// - Insufficient permissions to write to database file
    /// - Disk full or quota exceeded
    /// - Corrupted database file
    ///
    /// Suggestions:
    /// - Check file permissions on the database directory
    /// - Ensure no other process is using the database
    /// - Try deleting the database file to recreate it (data will be lost)
    #[error("Database error: {0}\n\nSuggestion: Check file permissions and ensure the database isn't locked by another process")]
    Database(#[from] rusqlite::Error),

    /// HTTP request error
    ///
    /// Common causes:
    /// - Network connectivity issues
    /// - Invalid endpoint URL
    /// - Server is down or unreachable
    /// - DNS resolution failure
    /// - SSL/TLS certificate issues
    ///
    /// Suggestions:
    /// - Check your internet connection
    /// - Verify the endpoint URL is correct
    /// - Check if the server is accessible via curl/ping
    #[cfg(feature = "sync")]
    #[error("HTTP request failed: {0}\n\nSuggestion: Check network connectivity and verify the endpoint URL")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    ///
    /// Common causes:
    /// - Invalid JSON structure
    /// - Schema version mismatch
    /// - Missing required fields
    /// - Type conversion errors
    ///
    /// Suggestions:
    /// - Check that SDK and server versions are compatible
    /// - Verify custom event data is valid JSON
    #[error("JSON serialization error: {0}\n\nSuggestion: Ensure event data is valid JSON and SDK version matches server")]
    Json(#[from] serde_json::Error),

    /// Invalid configuration
    ///
    /// Suggestions:
    /// - Review the configuration documentation
    /// - Ensure all required fields are provided
    /// - Check that UUIDs are in valid format
    #[error("Invalid configuration: {0}\n\nSuggestion: Review configuration requirements in the documentation")]
    InvalidConfig(String),

    /// Authentication error
    ///
    /// Common causes:
    /// - Invalid or expired token
    /// - Incorrect HMAC secret
    /// - Token doesn't have required permissions
    ///
    /// Suggestions:
    /// - Verify your token and secret are correct
    /// - Generate a new token if the current one is expired
    /// - Check that the token has sync permissions
    #[error("Authentication failed: {0}\n\nSuggestion: Verify your token and secret are correct")]
    Auth(String),

    /// Rate limit exceeded
    ///
    /// You've exceeded the rate limit for your tier.
    ///
    /// Suggestions:
    /// - Wait the specified duration before retrying
    /// - Reduce event frequency
    /// - Batch events together
    /// - Consider upgrading to a higher tier
    #[error("Rate limit exceeded. Retry after {retry_after} seconds.\n\nSuggestion: Batch events together or upgrade your plan")]
    RateLimitExceeded {
        /// Seconds to wait before retrying
        retry_after: u64,
    },

    /// Server error with status code
    ///
    /// The server encountered an error processing your request.
    ///
    /// Suggestions:
    /// - If 5xx error, retry the request after a delay
    /// - If 4xx error, check your request parameters
    /// - Check server status page if available
    #[error("Server error ({status}): {message}\n\nSuggestion: {}", Self::server_error_suggestion(*status))]
    ServerError {
        /// HTTP status code
        status: u16,
        /// Error message from server
        message: String,
    },

    /// Maximum retries exceeded
    ///
    /// The operation failed after multiple retry attempts.
    ///
    /// Common causes:
    /// - Persistent network issues
    /// - Server is down
    /// - Invalid credentials
    ///
    /// Suggestions:
    /// - Check server health and network connectivity
    /// - Verify authentication credentials
    /// - Enable offline mode to queue events locally
    #[error("Maximum retries exceeded\n\nSuggestion: Check server health and network connectivity, or enable offline mode")]
    MaxRetriesExceeded,

    /// Invalid event schema
    ///
    /// The event structure doesn't match the expected schema.
    ///
    /// Suggestions:
    /// - Ensure SDK version is compatible with server
    /// - Check that required event fields are present
    /// - Review event schema documentation
    #[error(
        "Invalid event schema: {0}\n\nSuggestion: Ensure SDK version is compatible with server"
    )]
    InvalidSchema(String),

    /// IO error
    ///
    /// Common causes:
    /// - File or directory doesn't exist
    /// - Insufficient permissions
    /// - Disk full
    ///
    /// Suggestions:
    /// - Check file/directory exists and is accessible
    /// - Verify write permissions
    /// - Ensure sufficient disk space
    #[error("IO error: {0}\n\nSuggestion: Check file permissions and available disk space")]
    Io(#[from] std::io::Error),

    /// Machine ID error
    ///
    /// Failed to generate or retrieve a unique machine identifier.
    ///
    /// Common causes:
    /// - System doesn't have machine UUID available
    /// - Insufficient permissions to access machine ID
    ///
    /// Suggestions:
    /// - This is usually safe to ignore (fallback ID will be used)
    /// - On Docker/CI, this is expected behavior
    #[error("Failed to get machine ID: {0}\n\nNote: This is normal in Docker/CI environments. A fallback ID will be used.")]
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

    /// Get actionable suggestion based on HTTP status code
    fn server_error_suggestion(status: u16) -> &'static str {
        match status {
            400 => "Check request parameters and ensure they're valid",
            401 => "Verify your authentication token and secret",
            403 => "Your token doesn't have permission for this operation",
            404 => "Check the endpoint URL - resource not found",
            413 => "Request payload too large - reduce batch size",
            429 => "Rate limited - wait before retrying or upgrade plan",
            500..=599 => "Server error - retry with exponential backoff",
            _ => "Check server logs for details",
        }
    }

    /// Create a helpful InvalidConfig error with context
    pub fn invalid_config(field: &str, reason: &str) -> Self {
        Self::InvalidConfig(format!("{}: {}", field, reason))
    }

    /// Create a helpful InvalidConfig error for UUID validation
    pub fn invalid_uuid(field: &str, value: &str) -> Self {
        Self::InvalidConfig(format!(
            "{} '{}' is not a valid UUID. Expected format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
            field, value
        ))
    }

    /// Create a helpful error for missing required fields
    pub fn missing_field(field: &str) -> Self {
        Self::InvalidConfig(format!("Missing required field: {}", field))
    }
}

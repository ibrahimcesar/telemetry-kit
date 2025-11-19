//! Sync protocol implementation for telemetry-kit
//!
//! This module handles synchronization of local events to the telemetry-kit.dev service
//! using HMAC-SHA256 authentication.

mod auth;
mod client;
mod config;
mod retry;

pub use auth::HmacAuth;
pub use client::SyncClient;
pub use config::{SyncConfig, SyncConfigBuilder};
pub use retry::RetryStrategy;

use serde::{Deserialize, Serialize};

/// Response from the sync endpoint
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum SyncResponse {
    /// All events accepted
    #[serde(rename = "success")]
    Success {
        /// Number of events accepted
        accepted: usize,
        /// Number of events rejected
        rejected: usize,
        /// Success message
        message: String,
    },

    /// Partial success (some events rejected)
    #[serde(rename = "partial")]
    Partial {
        /// Number of events accepted
        accepted: usize,
        /// Number of events rejected
        rejected: usize,
        /// Error details for rejected events
        errors: Vec<EventError>,
    },
}

/// Error for a specific event
#[derive(Debug, Serialize, Deserialize)]
pub struct EventError {
    /// Event ID that failed
    pub event_id: uuid::Uuid,
    /// Error code
    pub error: String,
    /// Human-readable error message
    pub message: String,
}

/// Error response from server
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error code
    pub error: String,
    /// Error message
    pub message: String,
    /// Additional details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// Retry after (for 429 responses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

impl SyncResponse {
    /// Get the number of accepted events
    pub fn accepted(&self) -> usize {
        match self {
            SyncResponse::Success { accepted, .. } => *accepted,
            SyncResponse::Partial { accepted, .. } => *accepted,
        }
    }

    /// Get the number of rejected events
    pub fn rejected(&self) -> usize {
        match self {
            SyncResponse::Success { rejected, .. } => *rejected,
            SyncResponse::Partial { rejected, .. } => *rejected,
        }
    }

    /// Check if all events were accepted
    pub fn is_success(&self) -> bool {
        matches!(self, SyncResponse::Success { .. })
    }
}

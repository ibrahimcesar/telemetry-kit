//! Event schema types and builders

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Schema version for events
pub const SCHEMA_VERSION: &str = "1.0.0";

/// Complete event structure as sent to the server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Schema version
    pub schema_version: String,

    /// Unique event identifier
    pub event_id: Uuid,

    /// When the event occurred
    pub timestamp: DateTime<Utc>,

    /// Service information
    pub service: ServiceInfo,

    /// User identification (anonymous)
    pub user_id: String,

    /// Session identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// Environment information
    pub environment: Environment,

    /// The actual event data
    pub event: EventData,

    /// Metadata about transmission
    pub metadata: Metadata,
}

/// Service/application information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service name (e.g., "my-cli")
    pub name: String,

    /// Service version (e.g., "1.2.0")
    pub version: String,

    /// Programming language
    pub language: String,

    /// Language version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_version: Option<String>,
}

/// Environment/system information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// Operating system (linux, macos, windows, etc.)
    pub os: String,

    /// OS version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,

    /// System architecture (x86_64, aarch64, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,

    /// Whether running in CI environment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ci: Option<bool>,

    /// Shell type (bash, zsh, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
}

/// Event data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    /// Event type (command_execution, feature_used, etc.)
    #[serde(rename = "type")]
    pub event_type: String,

    /// Event category (usage, error, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Event-specific data
    pub data: serde_json::Value,
}

/// Transmission metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// SDK version
    pub sdk_version: String,

    /// When the event was transmitted
    pub transmission_timestamp: DateTime<Utc>,

    /// Number of events in this batch
    pub batch_size: usize,

    /// Number of retry attempts
    pub retry_count: u32,
}

/// Builder for command execution events
#[derive(Debug, Default)]
pub struct CommandEventBuilder {
    command: String,
    subcommand: Option<String>,
    flags: Vec<String>,
    success: Option<bool>,
    duration_ms: Option<u64>,
    exit_code: Option<i32>,
}

impl CommandEventBuilder {
    /// Create a new command event builder
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            ..Default::default()
        }
    }

    /// Set subcommand
    pub fn subcommand(mut self, subcommand: impl Into<String>) -> Self {
        self.subcommand = Some(subcommand.into());
        self
    }

    /// Add a flag
    pub fn flag(mut self, flag: impl Into<String>) -> Self {
        self.flags.push(flag.into());
        self
    }

    /// Set success status
    pub fn success(mut self, success: bool) -> Self {
        self.success = Some(success);
        self
    }

    /// Set duration in milliseconds
    pub fn duration_ms(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }

    /// Set exit code
    pub fn exit_code(mut self, exit_code: i32) -> Self {
        self.exit_code = Some(exit_code);
        self
    }

    /// Build the event data
    pub fn build(self) -> serde_json::Value {
        let mut data = HashMap::new();
        data.insert("command".to_string(), serde_json::json!(self.command));

        if let Some(subcommand) = self.subcommand {
            data.insert("subcommand".to_string(), serde_json::json!(subcommand));
        }

        if !self.flags.is_empty() {
            data.insert("flags".to_string(), serde_json::json!(self.flags));
        }

        if let Some(success) = self.success {
            data.insert("success".to_string(), serde_json::json!(success));
        }

        if let Some(duration_ms) = self.duration_ms {
            data.insert("duration_ms".to_string(), serde_json::json!(duration_ms));
        }

        if let Some(exit_code) = self.exit_code {
            data.insert("exit_code".to_string(), serde_json::json!(exit_code));
        }

        serde_json::json!(data)
    }
}

/// Builder for feature usage events
#[derive(Debug, Default)]
pub struct FeatureEventBuilder {
    feature: String,
    method: Option<String>,
    success: Option<bool>,
    custom_data: HashMap<String, serde_json::Value>,
}

impl FeatureEventBuilder {
    /// Create a new feature event builder
    pub fn new(feature: impl Into<String>) -> Self {
        Self {
            feature: feature.into(),
            ..Default::default()
        }
    }

    /// Set method/variant
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    /// Set success status
    pub fn success(mut self, success: bool) -> Self {
        self.success = Some(success);
        self
    }

    /// Add custom data field
    pub fn data(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.custom_data.insert(key.into(), value);
        self
    }

    /// Build the event data
    pub fn build(self) -> serde_json::Value {
        let mut data = HashMap::new();
        data.insert("feature".to_string(), serde_json::json!(self.feature));

        if let Some(method) = self.method {
            data.insert("method".to_string(), serde_json::json!(method));
        }

        if let Some(success) = self.success {
            data.insert("success".to_string(), serde_json::json!(success));
        }

        for (key, value) in self.custom_data {
            data.insert(key, value);
        }

        serde_json::json!(data)
    }
}

/// Batch of events for transmission
#[derive(Debug, Serialize, Deserialize)]
pub struct EventBatch {
    /// Events in this batch
    pub events: Vec<Event>,
}

impl EventBatch {
    /// Create a new event batch
    pub fn new(events: Vec<Event>) -> Self {
        Self { events }
    }

    /// Get the batch size
    pub fn size(&self) -> usize {
        self.events.len()
    }

    /// Check if batch is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

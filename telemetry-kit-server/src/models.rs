//! Database models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// API token with associated secret
#[derive(Debug, Clone, FromRow)]
pub struct ApiToken {
    pub id: Uuid,
    pub org_id: Uuid,
    pub app_id: Uuid,
    pub token: String,
    pub secret: String,
    pub tier: TokenTier,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// Token tier for rate limiting
#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "token_tier", rename_all = "lowercase")]
pub enum TokenTier {
    Free,
    Pro,
    Business,
    Enterprise,
}

/// Stored telemetry event
#[derive(Debug, Clone, FromRow)]
pub struct StoredEvent {
    pub id: i64,
    pub event_id: Uuid,
    pub org_id: Uuid,
    pub app_id: Uuid,
    pub schema_version: String,
    pub timestamp: DateTime<Utc>,

    // Service info
    pub service_name: String,
    pub service_version: String,
    pub service_language: String,
    pub service_language_version: Option<String>,

    // User info (anonymous)
    pub user_id: String,
    pub session_id: Option<String>,

    // Environment
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub arch: Option<String>,
    pub ci: Option<bool>,
    pub shell: Option<String>,

    // Event
    pub event_type: String,
    pub event_category: Option<String>,
    pub event_data: sqlx::types::Json<serde_json::Value>,

    // Metadata
    pub sdk_version: String,
    pub transmission_timestamp: DateTime<Utc>,
    pub batch_size: i32,
    pub retry_count: i32,

    // Tracking
    pub received_at: DateTime<Utc>,
}

/// Incoming event from client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingEvent {
    pub schema_version: String,
    pub event_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub service: ServiceInfo,
    pub user_id: String,
    pub session_id: Option<String>,
    pub environment: Environment,
    pub event: EventData,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub language: String,
    pub language_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub os: String,
    pub os_version: Option<String>,
    pub arch: Option<String>,
    pub ci: Option<bool>,
    pub shell: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    #[serde(rename = "type")]
    pub event_type: String,
    pub category: Option<String>,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub sdk_version: String,
    pub transmission_timestamp: DateTime<Utc>,
    pub batch_size: usize,
    pub retry_count: u32,
}

/// Batch of events from client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBatch {
    pub events: Vec<IncomingEvent>,
}

impl IncomingEvent {
    /// Convert to StoredEvent for database insertion
    pub fn to_stored(&self, org_id: Uuid, app_id: Uuid) -> StoredEvent {
        StoredEvent {
            id: 0, // Will be assigned by database
            event_id: self.event_id,
            org_id,
            app_id,
            schema_version: self.schema_version.clone(),
            timestamp: self.timestamp,
            service_name: self.service.name.clone(),
            service_version: self.service.version.clone(),
            service_language: self.service.language.clone(),
            service_language_version: self.service.language_version.clone(),
            user_id: self.user_id.clone(),
            session_id: self.session_id.clone(),
            os: Some(self.environment.os.clone()),
            os_version: self.environment.os_version.clone(),
            arch: self.environment.arch.clone(),
            ci: self.environment.ci,
            shell: self.environment.shell.clone(),
            event_type: self.event.event_type.clone(),
            event_category: self.event.category.clone(),
            event_data: sqlx::types::Json(self.event.data.clone()),
            sdk_version: self.metadata.sdk_version.clone(),
            transmission_timestamp: self.metadata.transmission_timestamp,
            batch_size: self.metadata.batch_size as i32,
            retry_count: self.metadata.retry_count as i32,
            received_at: Utc::now(),
        }
    }
}

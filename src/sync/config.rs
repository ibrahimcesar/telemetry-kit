//! Sync configuration

use crate::error::{Result, TelemetryError};
use uuid::Uuid;

/// Default sync endpoint
pub const DEFAULT_ENDPOINT: &str = "https://telemetry-kit.dev";

/// Staging endpoint for testing
pub const STAGING_ENDPOINT: &str = "https://staging.telemetry-kit.dev";

/// Maximum batch size (per protocol spec)
pub const MAX_BATCH_SIZE: usize = 1000;

/// Default batch size
pub const DEFAULT_BATCH_SIZE: usize = 100;

/// Sync configuration
#[derive(Debug, Clone)]
pub struct SyncConfig {
    /// API endpoint base URL
    pub endpoint: String,

    /// Organization ID
    pub org_id: Uuid,

    /// Application ID
    pub app_id: Uuid,

    /// API token
    pub token: String,

    /// API secret for HMAC signing
    pub secret: String,

    /// Batch size (1-1000)
    pub batch_size: usize,

    /// Maximum retry attempts
    pub max_retries: u32,

    /// Sync interval in seconds (0 = manual sync only)
    pub sync_interval_secs: u64,

    /// Enable DNT (Do Not Track) check
    pub respect_dnt: bool,
}

impl SyncConfig {
    /// Create a new sync configuration builder
    pub fn builder() -> SyncConfigBuilder {
        SyncConfigBuilder::new()
    }

    /// Get the full ingestion URL
    pub fn ingestion_url(&self) -> String {
        format!("{}/v1/ingest/{}/{}", self.endpoint, self.org_id, self.app_id)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.token.is_empty() {
            return Err(TelemetryError::InvalidConfig(
                "Token cannot be empty".to_string(),
            ));
        }

        if self.secret.is_empty() {
            return Err(TelemetryError::InvalidConfig(
                "Secret cannot be empty".to_string(),
            ));
        }

        if self.batch_size == 0 || self.batch_size > MAX_BATCH_SIZE {
            return Err(TelemetryError::InvalidConfig(format!(
                "Batch size must be between 1 and {}",
                MAX_BATCH_SIZE
            )));
        }

        Ok(())
    }
}

/// Builder for sync configuration
#[derive(Debug, Default)]
pub struct SyncConfigBuilder {
    endpoint: Option<String>,
    org_id: Option<Uuid>,
    app_id: Option<Uuid>,
    token: Option<String>,
    secret: Option<String>,
    batch_size: Option<usize>,
    max_retries: Option<u32>,
    sync_interval_secs: Option<u64>,
    respect_dnt: Option<bool>,
}

impl SyncConfigBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the API endpoint (default: production)
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    /// Use staging endpoint for testing
    pub fn use_staging(mut self) -> Self {
        self.endpoint = Some(STAGING_ENDPOINT.to_string());
        self
    }

    /// Set organization ID
    pub fn org_id(mut self, org_id: impl Into<String>) -> Result<Self> {
        let org_id_str = org_id.into();
        let uuid = Uuid::parse_str(&org_id_str).map_err(|e| {
            TelemetryError::InvalidConfig(format!("Invalid org_id UUID: {}", e))
        })?;
        self.org_id = Some(uuid);
        Ok(self)
    }

    /// Set organization ID from UUID
    pub fn org_id_uuid(mut self, org_id: Uuid) -> Self {
        self.org_id = Some(org_id);
        self
    }

    /// Set application ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Result<Self> {
        let app_id_str = app_id.into();
        let uuid = Uuid::parse_str(&app_id_str).map_err(|e| {
            TelemetryError::InvalidConfig(format!("Invalid app_id UUID: {}", e))
        })?;
        self.app_id = Some(uuid);
        Ok(self)
    }

    /// Set application ID from UUID
    pub fn app_id_uuid(mut self, app_id: Uuid) -> Self {
        self.app_id = Some(app_id);
        self
    }

    /// Set API token
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Set API secret
    pub fn secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(secret.into());
        self
    }

    /// Set batch size (1-1000)
    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = Some(batch_size);
        self
    }

    /// Set maximum retry attempts
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Set sync interval in seconds (0 = manual only)
    pub fn sync_interval_secs(mut self, interval: u64) -> Self {
        self.sync_interval_secs = Some(interval);
        self
    }

    /// Enable/disable DNT (Do Not Track) check
    pub fn respect_dnt(mut self, respect: bool) -> Self {
        self.respect_dnt = Some(respect);
        self
    }

    /// Build the configuration
    pub fn build(self) -> Result<SyncConfig> {
        let config = SyncConfig {
            endpoint: self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_string()),
            org_id: self
                .org_id
                .ok_or_else(|| TelemetryError::InvalidConfig("org_id is required".to_string()))?,
            app_id: self
                .app_id
                .ok_or_else(|| TelemetryError::InvalidConfig("app_id is required".to_string()))?,
            token: self
                .token
                .ok_or_else(|| TelemetryError::InvalidConfig("token is required".to_string()))?,
            secret: self
                .secret
                .ok_or_else(|| TelemetryError::InvalidConfig("secret is required".to_string()))?,
            batch_size: self.batch_size.unwrap_or(DEFAULT_BATCH_SIZE),
            max_retries: self.max_retries.unwrap_or(5),
            sync_interval_secs: self.sync_interval_secs.unwrap_or(3600), // 1 hour default
            respect_dnt: self.respect_dnt.unwrap_or(true),
        };

        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = SyncConfig::builder()
            .org_id("550e8400-e29b-41d4-a716-446655440000")
            .unwrap()
            .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")
            .unwrap()
            .token("tk_test_token")
            .secret("test_secret")
            .build()
            .unwrap();

        assert_eq!(config.endpoint, DEFAULT_ENDPOINT);
        assert_eq!(config.batch_size, DEFAULT_BATCH_SIZE);
        assert!(config.respect_dnt);
    }

    #[test]
    fn test_staging_endpoint() {
        let config = SyncConfig::builder()
            .use_staging()
            .org_id("550e8400-e29b-41d4-a716-446655440000")
            .unwrap()
            .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")
            .unwrap()
            .token("tk_test_token")
            .secret("test_secret")
            .build()
            .unwrap();

        assert_eq!(config.endpoint, STAGING_ENDPOINT);
    }

    #[test]
    fn test_ingestion_url() {
        let config = SyncConfig::builder()
            .org_id("550e8400-e29b-41d4-a716-446655440000")
            .unwrap()
            .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")
            .unwrap()
            .token("tk_test_token")
            .secret("test_secret")
            .build()
            .unwrap();

        let url = config.ingestion_url();
        assert!(url.contains("/v1/ingest/"));
        assert!(url.contains("550e8400-e29b-41d4-a716-446655440000"));
        assert!(url.contains("7c9e6679-7425-40de-944b-e07fc1f90ae7"));
    }

    #[test]
    fn test_validation_empty_token() {
        let result = SyncConfig::builder()
            .org_id("550e8400-e29b-41d4-a716-446655440000")
            .unwrap()
            .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")
            .unwrap()
            .token("")
            .secret("test_secret")
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_validation_invalid_batch_size() {
        let result = SyncConfig::builder()
            .org_id("550e8400-e29b-41d4-a716-446655440000")
            .unwrap()
            .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")
            .unwrap()
            .token("tk_test_token")
            .secret("test_secret")
            .batch_size(2000) // Exceeds MAX_BATCH_SIZE
            .build();

        assert!(result.is_err());
    }
}

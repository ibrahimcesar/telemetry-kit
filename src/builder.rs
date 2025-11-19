//! Builder for TelemetryKit

use crate::error::{Result, TelemetryError};
use crate::telemetry::TelemetryKit;

#[cfg(feature = "sync")]
use crate::sync::SyncConfig;

use std::path::PathBuf;

/// Builder for configuring telemetry
#[derive(Debug, Default)]
pub struct TelemetryBuilder {
    service_name: Option<String>,
    service_version: Option<String>,
    db_path: Option<PathBuf>,

    #[cfg(feature = "sync")]
    sync_config: Option<SyncConfig>,

    #[cfg(feature = "sync")]
    auto_sync: bool,
}

impl TelemetryBuilder {
    /// Create a new telemetry builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the service name (required)
    pub fn service_name(mut self, name: impl Into<String>) -> Result<Self> {
        let name_str = name.into();

        // Validate service name format
        if !name_str
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        {
            return Err(TelemetryError::InvalidConfig(
                "Service name must contain only lowercase alphanumeric, dashes, and underscores"
                    .to_string(),
            ));
        }

        self.service_name = Some(name_str);
        Ok(self)
    }

    /// Set the service version (recommended)
    pub fn service_version(mut self, version: impl Into<String>) -> Self {
        self.service_version = Some(version.into());
        self
    }

    /// Set custom database path for event storage
    pub fn db_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.db_path = Some(path.into());
        self
    }

    /// Configure sync settings
    #[cfg(feature = "sync")]
    pub fn sync(mut self, config: SyncConfig) -> Self {
        self.sync_config = Some(config);
        self
    }

    /// Enable automatic background syncing
    #[cfg(feature = "sync")]
    pub fn enable_auto_sync(mut self) -> Self {
        self.auto_sync = true;
        self
    }

    /// Shorthand for setting sync credentials
    #[cfg(feature = "sync")]
    pub fn with_sync_credentials(
        mut self,
        org_id: impl Into<String>,
        app_id: impl Into<String>,
        token: impl Into<String>,
        secret: impl Into<String>,
    ) -> Result<Self> {
        let config = SyncConfig::builder()
            .org_id(org_id)?
            .app_id(app_id)?
            .token(token)
            .secret(secret)
            .build()?;

        self.sync_config = Some(config);
        Ok(self)
    }

    /// Build the TelemetryKit instance
    pub fn build(self) -> Result<TelemetryKit> {
        let service_name = self
            .service_name
            .ok_or_else(|| TelemetryError::InvalidConfig("service_name is required".to_string()))?;

        let service_version = self
            .service_version
            .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());

        // Determine database path
        let db_path = if let Some(path) = self.db_path {
            path
        } else {
            // Default: ~/.telemetry-kit/<service_name>.db
            let mut path = dirs::home_dir()
                .ok_or_else(|| TelemetryError::InvalidConfig("Cannot determine home directory".to_string()))?;
            path.push(".telemetry-kit");
            path.push(format!("{}.db", service_name));
            path
        };

        TelemetryKit::new(
            service_name,
            service_version,
            db_path,
            #[cfg(feature = "sync")]
            self.sync_config,
            #[cfg(feature = "sync")]
            self.auto_sync,
        )
    }
}

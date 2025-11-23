//! Builder for TelemetryKit

use crate::error::{Result, TelemetryError};
use crate::telemetry::TelemetryKit;

#[cfg(feature = "sync")]
use crate::sync::SyncConfig;

#[cfg(feature = "sync")]
use crate::auto_sync::AutoSyncConfig;

#[cfg(feature = "privacy")]
use crate::privacy::PrivacyConfig;

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
    auto_sync_enabled: bool,

    #[cfg(feature = "sync")]
    auto_sync_config: AutoSyncConfig,

    #[cfg(feature = "privacy")]
    privacy_config: Option<PrivacyConfig>,
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
            return Err(TelemetryError::invalid_config(
                "service_name",
                &format!("'{}' contains invalid characters. Use only lowercase letters, numbers, dashes, and underscores (e.g., 'my-app', 'cli_tool')", name_str)
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

    /// Enable automatic background syncing (enabled by default)
    #[cfg(feature = "sync")]
    pub fn auto_sync(mut self, enabled: bool) -> Self {
        self.auto_sync_enabled = enabled;
        self
    }

    /// Set the auto-sync interval in seconds (default: 60)
    #[cfg(feature = "sync")]
    pub fn sync_interval(mut self, seconds: u64) -> Self {
        self.auto_sync_config.interval = seconds;
        self
    }

    /// Configure whether to sync on shutdown (default: true)
    #[cfg(feature = "sync")]
    pub fn sync_on_shutdown(mut self, enabled: bool) -> Self {
        self.auto_sync_config.sync_on_shutdown = enabled;
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

    /// Configure privacy settings
    #[cfg(feature = "privacy")]
    pub fn privacy(mut self, config: PrivacyConfig) -> Self {
        self.privacy_config = Some(config);
        self
    }

    /// Shorthand for enabling strict privacy mode (GDPR-compliant)
    #[cfg(feature = "privacy")]
    pub fn strict_privacy(mut self) -> Self {
        self.privacy_config = Some(PrivacyConfig::strict());
        self
    }

    /// Shorthand for minimal privacy mode
    #[cfg(feature = "privacy")]
    pub fn minimal_privacy(mut self) -> Self {
        self.privacy_config = Some(PrivacyConfig::minimal());
        self
    }

    /// Require user consent before tracking
    #[cfg(feature = "privacy")]
    pub fn consent_required(mut self, required: bool) -> Self {
        let config = self.privacy_config.unwrap_or_default();
        self.privacy_config = Some(PrivacyConfig {
            consent_required: required,
            ..config
        });
        self
    }

    /// Set data retention period in days (0 = forever)
    #[cfg(feature = "privacy")]
    pub fn data_retention(mut self, days: u32) -> Self {
        let config = self.privacy_config.unwrap_or_default();
        self.privacy_config = Some(PrivacyConfig {
            data_retention_days: days,
            ..config
        });
        self
    }

    /// Enable or disable path sanitization
    #[cfg(feature = "privacy")]
    pub fn sanitize_paths(mut self, enabled: bool) -> Self {
        let config = self.privacy_config.unwrap_or_default();
        self.privacy_config = Some(PrivacyConfig {
            sanitize_paths: enabled,
            ..config
        });
        self
    }

    /// Enable or disable email sanitization
    #[cfg(feature = "privacy")]
    pub fn sanitize_emails(mut self, enabled: bool) -> Self {
        let config = self.privacy_config.unwrap_or_default();
        self.privacy_config = Some(PrivacyConfig {
            sanitize_emails: enabled,
            ..config
        });
        self
    }

    /// Prompt for user consent interactively before building
    ///
    /// This will show an interactive consent dialog on the first run (when consent status is Unknown).
    /// On subsequent runs, it will use the stored consent preference.
    ///
    /// This method is only available when both 'privacy' and 'cli' features are enabled.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use telemetry_kit::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let telemetry = TelemetryKit::builder()
    ///     .service_name("my-app")?
    ///     .service_version("1.0.0")
    ///     .prompt_for_consent()?  // Shows interactive prompt on first run
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(all(feature = "privacy", feature = "cli"))]
    pub fn prompt_for_consent(mut self) -> Result<Self> {
        use crate::privacy::PrivacyManager;

        let service_name = self
            .service_name
            .as_ref()
            .ok_or_else(|| TelemetryError::missing_field("service_name"))?
            .clone();

        let service_version = self
            .service_version
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("unknown");

        // Create a privacy manager with current config
        let config = self.privacy_config.clone().unwrap_or_default();
        let manager = PrivacyManager::new(config.clone(), &service_name)?;

        // Prompt for consent
        let _consent_granted = manager.prompt_for_consent(&service_name, service_version)?;

        // Set consent_required to true to respect the user's choice
        self.privacy_config = Some(PrivacyConfig {
            consent_required: true,
            ..config
        });

        Ok(self)
    }

    /// Prompt for user consent with minimal message
    ///
    /// Similar to `prompt_for_consent` but shows a shorter, one-line prompt.
    ///
    /// This method is only available when both 'privacy' and 'cli' features are enabled.
    #[cfg(all(feature = "privacy", feature = "cli"))]
    pub fn prompt_minimal(mut self) -> Result<Self> {
        use crate::privacy::PrivacyManager;

        let service_name = self
            .service_name
            .as_ref()
            .ok_or_else(|| TelemetryError::missing_field("service_name"))?
            .clone();

        // Create a privacy manager with current config
        let config = self.privacy_config.clone().unwrap_or_default();
        let manager = PrivacyManager::new(config.clone(), &service_name)?;

        // Prompt for consent
        let _consent_granted = manager.prompt_minimal(&service_name)?;

        // Set consent_required to true to respect the user's choice
        self.privacy_config = Some(PrivacyConfig {
            consent_required: true,
            ..config
        });

        Ok(self)
    }

    /// Build the TelemetryKit instance
    pub fn build(self) -> Result<TelemetryKit> {
        let service_name = self
            .service_name
            .ok_or_else(|| TelemetryError::missing_field("service_name"))?;

        let service_version = self
            .service_version
            .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());

        // Determine database path
        let db_path = if let Some(path) = self.db_path {
            path
        } else {
            // Default: ~/.telemetry-kit/<service_name>.db
            let mut path = dirs::home_dir()
                .ok_or_else(|| TelemetryError::invalid_config(
                    "database_path",
                    "Cannot determine home directory. Please set an explicit database path with .db_path()"
                ))?;
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
            self.auto_sync_enabled,
            #[cfg(feature = "sync")]
            self.auto_sync_config,
            #[cfg(feature = "privacy")]
            self.privacy_config,
        )
    }
}

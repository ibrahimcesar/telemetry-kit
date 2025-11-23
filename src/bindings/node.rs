//! Node.js bindings via napi-rs
//!
//! This module provides JavaScript/TypeScript bindings for telemetry-kit
//! using napi-rs for native Node.js addons.

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{TelemetryKit, TelemetryBuilder, Result as TelemetryResult};

/// Configuration for sync credentials
#[napi(object)]
pub struct SyncConfig {
    pub organization_id: String,
    pub application_id: String,
    pub token: String,
    pub secret: String,
    pub endpoint: Option<String>,
}

/// Configuration for privacy settings
#[napi(object)]
pub struct PrivacyConfig {
    pub anonymize_emails: Option<bool>,
    pub sanitize_paths: Option<bool>,
    pub respect_do_not_track: Option<bool>,
}

/// Configuration for consent management
#[napi(object)]
pub struct ConsentConfig {
    pub required: Option<bool>,
    pub default_granted: Option<bool>,
}

/// Builder configuration from JavaScript
#[napi(object)]
pub struct BuilderConfig {
    pub service_name: String,
    pub sync_config: Option<SyncConfig>,
    pub privacy_config: Option<PrivacyConfig>,
    pub consent_config: Option<ConsentConfig>,
    pub auto_sync: Option<bool>,
    pub sync_interval: Option<u64>,
    pub sync_on_shutdown: Option<bool>,
}

/// Options for tracking commands
#[napi(object)]
pub struct CommandOptions {
    pub success: Option<bool>,
    pub duration_ms: Option<u64>,
    pub metadata: Option<serde_json::Value>,
}

/// Options for tracking features
#[napi(object)]
pub struct FeatureOptions {
    pub enabled: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

/// Main TelemetryKit class for Node.js
///
/// This wraps the Rust TelemetryKit in a JavaScript-friendly API.
#[napi]
pub struct TelemetryKitNode {
    inner: Arc<RwLock<TelemetryKit>>,
}

#[napi]
impl TelemetryKitNode {
    /// Create a new TelemetryKit instance from configuration
    ///
    /// # Example (JavaScript)
    /// ```js
    /// const telemetry = new TelemetryKit({
    ///   serviceName: 'my-app',
    ///   autoSync: true
    /// });
    /// ```
    #[napi(constructor)]
    pub fn new(config: BuilderConfig) -> Result<Self> {
        // Start with builder
        let mut builder = TelemetryKit::builder();

        // Set service name
        builder = builder
            .service_name(&config.service_name)
            .map_err(|e| Error::from_reason(format!("Invalid service name: {}", e)))?;

        // Configure sync if provided
        if let Some(sync) = config.sync_config {
            builder = builder
                .with_sync_credentials(
                    &sync.organization_id,
                    &sync.application_id,
                    &sync.token,
                    &sync.secret,
                )
                .map_err(|e| Error::from_reason(format!("Invalid sync credentials: {}", e)))?;

            if let Some(endpoint) = sync.endpoint {
                builder = builder.endpoint(&endpoint);
            }
        }

        // Configure auto-sync settings
        if let Some(auto_sync) = config.auto_sync {
            builder = builder.auto_sync(auto_sync);
        }

        if let Some(interval) = config.sync_interval {
            builder = builder.sync_interval(interval);
        }

        if let Some(sync_on_shutdown) = config.sync_on_shutdown {
            builder = builder.sync_on_shutdown(sync_on_shutdown);
        }

        // Build the telemetry instance
        let telemetry = builder
            .build()
            .map_err(|e| Error::from_reason(format!("Failed to build telemetry: {}", e)))?;

        Ok(Self {
            inner: Arc::new(RwLock::new(telemetry)),
        })
    }

    /// Track a command execution
    ///
    /// # Example (JavaScript)
    /// ```js
    /// await telemetry.trackCommand('deploy', {
    ///   success: true,
    ///   durationMs: 1234
    /// });
    /// ```
    #[napi]
    pub async fn track_command(&self, name: String, options: Option<CommandOptions>) -> Result<()> {
        let telemetry = self.inner.read().await;

        telemetry
            .track_command(&name, |event| {
                if let Some(opts) = options {
                    if let Some(success) = opts.success {
                        event.success(success);
                    }
                    if let Some(duration_ms) = opts.duration_ms {
                        event.duration_ms(duration_ms);
                    }
                    // TODO: Add metadata support
                }
                event
            })
            .await
            .map_err(|e| Error::from_reason(format!("Failed to track command: {}", e)))
    }

    /// Track a feature usage
    ///
    /// # Example (JavaScript)
    /// ```js
    /// await telemetry.trackFeature('dark-mode', {
    ///   enabled: true
    /// });
    /// ```
    #[napi]
    pub async fn track_feature(&self, name: String, options: Option<FeatureOptions>) -> Result<()> {
        let telemetry = self.inner.read().await;

        telemetry
            .track_feature(&name, |event| {
                if let Some(opts) = options {
                    if let Some(enabled) = opts.enabled {
                        event.enabled(enabled);
                    }
                    // TODO: Add metadata support
                }
                event
            })
            .await
            .map_err(|e| Error::from_reason(format!("Failed to track feature: {}", e)))
    }

    /// Manually trigger synchronization
    ///
    /// # Example (JavaScript)
    /// ```js
    /// await telemetry.sync();
    /// ```
    #[napi]
    pub async fn sync(&self) -> Result<()> {
        let telemetry = self.inner.read().await;

        telemetry
            .sync()
            .await
            .map_err(|e| Error::from_reason(format!("Sync failed: {}", e)))
    }

    /// Gracefully shutdown telemetry
    ///
    /// This performs a final sync if configured and cleans up resources.
    ///
    /// # Example (JavaScript)
    /// ```js
    /// await telemetry.shutdown();
    /// ```
    #[napi]
    pub async fn shutdown(&self) -> Result<()> {
        let telemetry = self.inner.write().await;

        telemetry
            .shutdown()
            .await
            .map_err(|e| Error::from_reason(format!("Shutdown failed: {}", e)))
    }

    /// Get statistics about tracked events
    ///
    /// Returns an object with event counts.
    ///
    /// # Example (JavaScript)
    /// ```js
    /// const stats = await telemetry.stats();
    /// console.log(`Total: ${stats.total}, Synced: ${stats.synced}`);
    /// ```
    #[napi(object)]
    pub struct EventStats {
        pub total: u32,
        pub synced: u32,
        pub unsynced: u32,
    }

    #[napi]
    pub async fn stats(&self) -> Result<EventStats> {
        let telemetry = self.inner.read().await;

        // TODO: Implement actual stats from storage
        // For now, return placeholder
        Ok(EventStats {
            total: 0,
            synced: 0,
            unsynced: 0,
        })
    }
}

/// Builder for TelemetryKit
///
/// # Example (JavaScript)
/// ```js
/// const telemetry = TelemetryKit.builder()
///   .serviceName('my-app')
///   .withSyncCredentials(orgId, appId, token, secret)
///   .autoSync(true)
///   .build();
/// ```
#[napi]
pub struct TelemetryKitBuilder {
    config: BuilderConfig,
}

#[napi]
impl TelemetryKitBuilder {
    /// Create a new builder
    #[napi(factory)]
    pub fn new(service_name: String) -> Self {
        Self {
            config: BuilderConfig {
                service_name,
                sync_config: None,
                privacy_config: None,
                consent_config: None,
                auto_sync: None,
                sync_interval: None,
                sync_on_shutdown: None,
            },
        }
    }

    /// Set sync credentials
    #[napi]
    pub fn with_sync_credentials(
        &mut self,
        organization_id: String,
        application_id: String,
        token: String,
        secret: String,
    ) -> &Self {
        self.config.sync_config = Some(SyncConfig {
            organization_id,
            application_id,
            token,
            secret,
            endpoint: None,
        });
        self
    }

    /// Set custom sync endpoint
    #[napi]
    pub fn endpoint(&mut self, endpoint: String) -> &Self {
        if let Some(ref mut sync) = self.config.sync_config {
            sync.endpoint = Some(endpoint);
        }
        self
    }

    /// Enable or disable auto-sync
    #[napi]
    pub fn auto_sync(&mut self, enabled: bool) -> &Self {
        self.config.auto_sync = Some(enabled);
        self
    }

    /// Set sync interval in seconds
    #[napi]
    pub fn sync_interval(&mut self, seconds: u32) -> &Self {
        self.config.sync_interval = Some(seconds as u64);
        self
    }

    /// Enable sync on shutdown
    #[napi]
    pub fn sync_on_shutdown(&mut self, enabled: bool) -> &Self {
        self.config.sync_on_shutdown = Some(enabled);
        self
    }

    /// Build the TelemetryKit instance
    #[napi]
    pub fn build(&self) -> Result<TelemetryKitNode> {
        TelemetryKitNode::new(self.config.clone())
    }
}

// Helper to implement Clone for BuilderConfig (required for napi object)
impl Clone for BuilderConfig {
    fn clone(&self) -> Self {
        Self {
            service_name: self.service_name.clone(),
            sync_config: self.sync_config.clone(),
            privacy_config: self.privacy_config.clone(),
            consent_config: self.consent_config.clone(),
            auto_sync: self.auto_sync,
            sync_interval: self.sync_interval,
            sync_on_shutdown: self.sync_on_shutdown,
        }
    }
}

impl Clone for SyncConfig {
    fn clone(&self) -> Self {
        Self {
            organization_id: self.organization_id.clone(),
            application_id: self.application_id.clone(),
            token: self.token.clone(),
            secret: self.secret.clone(),
            endpoint: self.endpoint.clone(),
        }
    }
}

impl Clone for PrivacyConfig {
    fn clone(&self) -> Self {
        Self {
            anonymize_emails: self.anonymize_emails,
            sanitize_paths: self.sanitize_paths,
            respect_do_not_track: self.respect_do_not_track,
        }
    }
}

impl Clone for ConsentConfig {
    fn clone(&self) -> Self {
        Self {
            required: self.required,
            default_granted: self.default_granted,
        }
    }
}

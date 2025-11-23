//! Main telemetry SDK

use crate::builder::TelemetryBuilder;
use crate::error::{Result, TelemetryError};
use crate::event::{
    CommandEventBuilder, Environment, Event, EventBatch, EventData, FeatureEventBuilder, Metadata,
    ServiceInfo, SCHEMA_VERSION,
};
use crate::storage::EventStorage;
use crate::user::{generate_session_id, generate_user_id};
use chrono::Utc;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[cfg(feature = "sync")]
use crate::sync::{SyncClient, SyncConfig};

#[cfg(feature = "sync")]
use crate::auto_sync::{AutoSyncConfig, AutoSyncTask};

#[cfg(feature = "sync")]
use tokio::sync::Mutex;

#[cfg(feature = "privacy")]
use crate::privacy::{PrivacyConfig, PrivacyManager};

const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Main telemetry SDK
pub struct TelemetryKit {
    inner: Arc<TelemetryKitInner>,
}

struct TelemetryKitInner {
    service_name: String,
    service_version: String,
    user_id: String,
    session_id: String,
    environment: Environment,
    storage: Arc<RwLock<EventStorage>>,

    #[cfg(feature = "sync")]
    sync_client: Option<SyncClient>,

    #[cfg(feature = "sync")]
    auto_sync_task: Option<Arc<Mutex<AutoSyncTask>>>,

    #[cfg(feature = "privacy")]
    privacy_manager: Option<PrivacyManager>,
}

impl TelemetryKit {
    /// Create a new TelemetryKit builder
    pub fn builder() -> TelemetryBuilder {
        TelemetryBuilder::new()
    }

    /// Create a new TelemetryKit instance (internal)
    pub(crate) fn new(
        service_name: String,
        service_version: String,
        db_path: PathBuf,
        #[cfg(feature = "sync")] sync_config: Option<SyncConfig>,
        #[cfg(feature = "sync")] auto_sync_enabled: bool,
        #[cfg(feature = "sync")] auto_sync_config: AutoSyncConfig,
        #[cfg(feature = "privacy")] privacy_config: Option<PrivacyConfig>,
    ) -> Result<Self> {
        let user_id = generate_user_id()?;
        let session_id = generate_session_id();
        let environment = detect_environment();
        let storage = EventStorage::new(db_path)?;
        let storage_arc = Arc::new(RwLock::new(storage));

        #[cfg(feature = "sync")]
        let sync_client = if let Some(config) = sync_config {
            Some(SyncClient::new(config)?)
        } else {
            None
        };

        // Start auto-sync task if enabled and sync is configured
        #[cfg(feature = "sync")]
        let auto_sync_task = if auto_sync_enabled {
            if let Some(client) = sync_client.as_ref() {
                let task = AutoSyncTask::start(
                    Arc::new(client.clone()),
                    storage_arc.clone(),
                    auto_sync_config,
                );
                Some(Arc::new(Mutex::new(task)))
            } else {
                None
            }
        } else {
            None
        };

        // Create privacy manager if privacy config is provided
        #[cfg(feature = "privacy")]
        let privacy_manager = if let Some(config) = privacy_config {
            Some(PrivacyManager::new(config, &service_name)?)
        } else {
            None
        };

        let inner = Arc::new(TelemetryKitInner {
            service_name,
            service_version,
            user_id,
            session_id,
            environment,
            storage: storage_arc,
            #[cfg(feature = "sync")]
            sync_client,
            #[cfg(feature = "sync")]
            auto_sync_task,
            #[cfg(feature = "privacy")]
            privacy_manager,
        });

        Ok(Self { inner })
    }

    /// Track a command execution
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use telemetry_kit::prelude::*;
    /// # async fn example(telemetry: &TelemetryKit) -> Result<()> {
    /// telemetry.track_command("build", |event| {
    ///     event
    ///         .flag("--release")
    ///         .duration_ms(1234)
    ///         .success(true)
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn track_command<F>(&self, command: impl Into<String>, builder_fn: F) -> Result<()>
    where
        F: FnOnce(CommandEventBuilder) -> CommandEventBuilder,
    {
        let builder = CommandEventBuilder::new(command);
        let event_data = builder_fn(builder).build();

        self.track_event("command_execution", Some("usage"), event_data)
            .await
    }

    /// Track a feature usage
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use telemetry_kit::prelude::*;
    /// # async fn example(telemetry: &TelemetryKit) -> Result<()> {
    /// telemetry.track_feature("authentication", |event| {
    ///     event
    ///         .method("oauth")
    ///         .success(true)
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn track_feature<F>(&self, feature: impl Into<String>, builder_fn: F) -> Result<()>
    where
        F: FnOnce(FeatureEventBuilder) -> FeatureEventBuilder,
    {
        let builder = FeatureEventBuilder::new(feature);
        let event_data = builder_fn(builder).build();

        self.track_event("feature_used", Some("library"), event_data)
            .await
    }

    /// Track a custom event
    pub async fn track_custom(
        &self,
        event_type: impl Into<String>,
        data: serde_json::Value,
    ) -> Result<()> {
        self.track_event(event_type, None, data).await
    }

    /// Internal method to track an event
    async fn track_event(
        &self,
        event_type: impl Into<String>,
        category: Option<&str>,
        data: serde_json::Value,
    ) -> Result<()> {
        // Check privacy settings - should we track this event?
        #[cfg(feature = "privacy")]
        if let Some(privacy_manager) = &self.inner.privacy_manager {
            if !privacy_manager.should_track()? {
                // User has opted out or denied consent - don't track
                return Ok(());
            }
        }

        // Apply data sanitization
        let mut sanitized_data = data;
        #[cfg(feature = "privacy")]
        if let Some(privacy_manager) = &self.inner.privacy_manager {
            privacy_manager.sanitize_data(&mut sanitized_data);
        }

        let event = Event {
            schema_version: SCHEMA_VERSION.to_string(),
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            service: ServiceInfo {
                name: self.inner.service_name.clone(),
                version: self.inner.service_version.clone(),
                language: "rust".to_string(),
                language_version: Some(rustc_version()),
            },
            user_id: self.inner.user_id.clone(),
            session_id: Some(self.inner.session_id.clone()),
            environment: self.inner.environment.clone(),
            event: EventData {
                event_type: event_type.into(),
                category: category.map(|s| s.to_string()),
                data: sanitized_data,
            },
            metadata: Metadata {
                sdk_version: format!("telemetry-kit-rust/{}", SDK_VERSION),
                transmission_timestamp: Utc::now(),
                batch_size: 1,
                retry_count: 0,
            },
        };

        // Store event
        let storage = self.inner.storage.write().await;
        storage.insert(&event)?;
        drop(storage);

        // Auto-sync task will pick up the event on next interval (if enabled)

        Ok(())
    }

    /// Manually trigger a sync
    #[cfg(feature = "sync")]
    pub async fn sync(&self) -> Result<()> {
        if let Some(client) = &self.inner.sync_client {
            Self::sync_events(Arc::new(client.clone()), self.inner.storage.clone()).await
        } else {
            Err(TelemetryError::invalid_config(
                "sync",
                "Sync is not configured. Use .with_sync_credentials() when building TelemetryKit",
            ))
        }
    }

    /// Internal sync implementation
    #[cfg(feature = "sync")]
    async fn sync_events(
        client: Arc<SyncClient>,
        storage: Arc<RwLock<EventStorage>>,
    ) -> Result<()> {
        let storage_read = storage.read().await;
        let events = storage_read.get_unsynced(client.config().batch_size)?;
        drop(storage_read);

        if events.is_empty() {
            return Ok(());
        }

        let event_ids: Vec<Uuid> = events.iter().map(|e| e.event_id).collect();
        let batch = EventBatch::new(events);

        match client.sync(batch).await {
            Ok(response) => {
                if response.accepted() > 0 {
                    let storage_write = storage.write().await;
                    storage_write.mark_synced(&event_ids)?;
                }
                Ok(())
            }
            Err(e) => {
                let storage_write = storage.write().await;
                storage_write.increment_retry(&event_ids)?;
                Err(e)
            }
        }
    }

    /// Get statistics about buffered events
    pub async fn stats(&self) -> Result<EventStats> {
        let storage = self.inner.storage.read().await;
        let total = storage.total_count()?;
        let unsynced = storage.unsynced_count()?;

        Ok(EventStats {
            total_events: total,
            unsynced_events: unsynced,
            synced_events: total - unsynced,
        })
    }

    /// Clean up old synced events (older than 7 days)
    pub async fn cleanup(&self) -> Result<usize> {
        let storage = self.inner.storage.write().await;
        storage.cleanup_old_events()
    }

    /// Grant user consent for telemetry tracking
    #[cfg(feature = "privacy")]
    pub fn grant_consent(&self) -> Result<()> {
        if let Some(privacy_manager) = &self.inner.privacy_manager {
            privacy_manager.grant_consent(&self.inner.service_name)
        } else {
            Err(TelemetryError::invalid_config(
                "privacy",
                "Privacy features are not enabled. Use .privacy() or .strict_privacy() when building TelemetryKit"
            ))
        }
    }

    /// Deny user consent for telemetry tracking
    #[cfg(feature = "privacy")]
    pub fn deny_consent(&self) -> Result<()> {
        if let Some(privacy_manager) = &self.inner.privacy_manager {
            privacy_manager.deny_consent(&self.inner.service_name)
        } else {
            Err(TelemetryError::invalid_config(
                "privacy",
                "Privacy features are not enabled. Use .privacy() or .strict_privacy() when building TelemetryKit"
            ))
        }
    }

    /// Opt out of telemetry tracking (equivalent to DO_NOT_TRACK)
    #[cfg(feature = "privacy")]
    pub fn opt_out(&self) -> Result<()> {
        if let Some(privacy_manager) = &self.inner.privacy_manager {
            privacy_manager.opt_out(&self.inner.service_name)
        } else {
            Err(TelemetryError::InvalidConfig(
                "Privacy features are not enabled".to_string(),
            ))
        }
    }

    /// Check if DO_NOT_TRACK environment variable is set
    #[cfg(feature = "privacy")]
    pub fn is_do_not_track_enabled() -> bool {
        PrivacyManager::is_do_not_track_enabled()
    }

    /// Gracefully shutdown auto-sync task and optionally perform final sync
    #[cfg(feature = "sync")]
    pub async fn shutdown(&self) -> Result<()> {
        if let Some(task_mutex) = &self.inner.auto_sync_task {
            let mut task = task_mutex.lock().await;

            // Perform final sync if configured
            if task.should_sync_on_shutdown() {
                if let Some(client) = &self.inner.sync_client {
                    let _ = Self::sync_events(Arc::new(client.clone()), self.inner.storage.clone())
                        .await;
                }
            }

            // Shutdown the background task
            task.shutdown();
            task.join().await?;
        }
        Ok(())
    }
}

#[cfg(feature = "sync")]
impl Drop for TelemetryKit {
    fn drop(&mut self) {
        // Note: We can't perform async operations in Drop
        // Users should call .shutdown() explicitly for graceful shutdown
        // The auto-sync task will stop automatically via its own Drop
    }
}

impl Clone for SyncClient {
    fn clone(&self) -> Self {
        // This is a workaround since SyncClient doesn't derive Clone
        // In production, we'd use Arc<SyncClient> instead
        Self::new(self.config().clone()).expect("Failed to clone SyncClient")
    }
}

/// Event statistics
#[derive(Debug, Clone)]
pub struct EventStats {
    /// Total number of events
    pub total_events: usize,
    /// Number of unsynced events
    pub unsynced_events: usize,
    /// Number of synced events
    pub synced_events: usize,
}

/// Detect environment information
fn detect_environment() -> Environment {
    Environment {
        os: std::env::consts::OS.to_string(),
        os_version: None, // Could use sys-info crate for this
        arch: Some(std::env::consts::ARCH.to_string()),
        ci: Some(is_ci()),
        shell: detect_shell(),
    }
}

/// Detect if running in CI
fn is_ci() -> bool {
    std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || std::env::var("JENKINS_HOME").is_ok()
        || std::env::var("TRAVIS").is_ok()
        || std::env::var("CIRCLECI").is_ok()
}

/// Detect shell
fn detect_shell() -> Option<String> {
    std::env::var("SHELL").ok().and_then(|s| {
        std::path::Path::new(&s)
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
    })
}

/// Get Rust compiler version
fn rustc_version() -> String {
    // This is a simplified version - in production we'd use rustc_version crate
    env!("CARGO_PKG_RUST_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_telemetry_creation() {
        use uuid::Uuid;
        let unique_name = format!("test-creation-{}", Uuid::new_v4());
        let telemetry = TelemetryKit::builder()
            .service_name(&unique_name)
            .unwrap()
            .service_version("1.0.0")
            .build();

        assert!(telemetry.is_ok());
    }

    #[tokio::test]
    async fn test_track_command() {
        use uuid::Uuid;
        let unique_name = format!("test-track-{}", Uuid::new_v4());
        let telemetry = TelemetryKit::builder()
            .service_name(&unique_name)
            .unwrap()
            .service_version("1.0.0")
            .build()
            .unwrap();

        let result = telemetry
            .track_command("test", |event| event.success(true).duration_ms(100))
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_event_stats() {
        use uuid::Uuid;
        let unique_name = format!("test-stats-{}", Uuid::new_v4());
        let telemetry = TelemetryKit::builder()
            .service_name(&unique_name)
            .unwrap()
            .service_version("1.0.0")
            .build()
            .unwrap();

        telemetry
            .track_command("test", |event| event.success(true))
            .await
            .unwrap();

        let stats = telemetry.stats().await.unwrap();
        assert_eq!(stats.total_events, 1);
        assert_eq!(stats.unsynced_events, 1);
    }

    #[test]
    fn test_ci_detection() {
        std::env::remove_var("CI");
        assert!(!is_ci());

        std::env::set_var("CI", "true");
        assert!(is_ci());

        std::env::remove_var("CI");
    }
}

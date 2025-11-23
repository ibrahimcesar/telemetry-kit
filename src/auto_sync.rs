//! Auto-sync background task for telemetry-kit
//!
//! Automatically syncs buffered events to the server in the background.

use crate::error::Result;
use crate::storage::EventStorage;
use crate::sync::SyncClient;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

/// Configuration for auto-sync behavior
#[derive(Debug, Clone)]
pub struct AutoSyncConfig {
    /// Interval between sync attempts (seconds)
    pub interval: u64,
    /// Whether to sync on shutdown
    pub sync_on_shutdown: bool,
    /// Maximum batch size per sync
    pub batch_size: usize,
}

impl Default for AutoSyncConfig {
    fn default() -> Self {
        Self {
            interval: 60,           // 60 seconds
            sync_on_shutdown: true, // Sync before dropping
            batch_size: 100,        // Match sync client default
        }
    }
}

/// Background task that automatically syncs events
pub struct AutoSyncTask {
    handle: Option<JoinHandle<()>>,
    shutdown: Arc<AtomicBool>,
    config: AutoSyncConfig,
}

impl AutoSyncTask {
    /// Start a new auto-sync background task
    pub fn start(
        client: Arc<SyncClient>,
        storage: Arc<RwLock<EventStorage>>,
        config: AutoSyncConfig,
    ) -> Self {
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = shutdown.clone();
        let interval = Duration::from_secs(config.interval);

        let handle = tokio::spawn(async move {
            loop {
                // Check if shutdown requested
                if shutdown_clone.load(Ordering::SeqCst) {
                    break;
                }

                // Perform sync
                if let Err(e) = Self::sync_once(client.clone(), storage.clone()).await {
                    // Log error but don't crash - sync will retry on next interval
                    eprintln!("Auto-sync error: {}", e);
                }

                // Wait for next interval
                tokio::time::sleep(interval).await;
            }
        });

        Self {
            handle: Some(handle),
            shutdown,
            config,
        }
    }

    /// Perform a single sync operation
    async fn sync_once(client: Arc<SyncClient>, storage: Arc<RwLock<EventStorage>>) -> Result<()> {
        use crate::event::EventBatch;
        use uuid::Uuid;

        // Get unsynced events
        let storage_read = storage.read().await;
        let events = storage_read.get_unsynced(client.config().batch_size)?;
        drop(storage_read);

        if events.is_empty() {
            return Ok(());
        }

        let event_ids: Vec<Uuid> = events.iter().map(|e| e.event_id).collect();
        let batch = EventBatch::new(events);

        // Attempt sync
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

    /// Request graceful shutdown of the background task
    pub fn shutdown(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }

    /// Wait for the background task to complete
    pub async fn join(&mut self) -> Result<()> {
        if let Some(handle) = self.handle.take() {
            handle.await.map_err(|e| {
                crate::error::TelemetryError::Other(format!("Task join error: {}", e))
            })?;
        }
        Ok(())
    }

    /// Check if auto-sync should sync on shutdown
    pub fn should_sync_on_shutdown(&self) -> bool {
        self.config.sync_on_shutdown
    }
}

impl Drop for AutoSyncTask {
    fn drop(&mut self) {
        // Request shutdown when dropped
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::SyncConfig;
    use std::path::PathBuf;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_auto_sync_task_creation() {
        // Create test storage
        let unique_id = Uuid::new_v4();
        let db_path = PathBuf::from(format!("/tmp/telemetry-test-autosync-{}.db", unique_id));
        let storage = Arc::new(RwLock::new(EventStorage::new(db_path.clone()).unwrap()));

        // Create test sync client (will fail to connect, but that's ok for this test)
        let config = SyncConfig::builder()
            .org_id("550e8400-e29b-41d4-a716-446655440000")
            .unwrap()
            .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")
            .unwrap()
            .token("test-token")
            .secret("test-secret")
            .build()
            .unwrap();
        let client = Arc::new(SyncClient::new(config).unwrap());

        // Create auto-sync task
        let mut task = AutoSyncTask::start(
            client,
            storage,
            AutoSyncConfig {
                interval: 1,
                sync_on_shutdown: true,
                batch_size: 100,
            },
        );

        // Wait a bit
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Shutdown
        task.shutdown();
        task.join().await.unwrap();

        // Cleanup
        let _ = std::fs::remove_file(&db_path);
    }

    #[tokio::test]
    async fn test_auto_sync_config_defaults() {
        let config = AutoSyncConfig::default();
        assert_eq!(config.interval, 60);
        assert!(config.sync_on_shutdown);
        assert_eq!(config.batch_size, 100);
    }
}

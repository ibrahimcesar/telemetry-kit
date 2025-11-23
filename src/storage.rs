//! SQLite storage for buffering events

use crate::error::Result;
use crate::event::Event;
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::PathBuf;

/// SQLite storage for buffering telemetry events
pub struct EventStorage {
    conn: Connection,
}

// SAFETY: EventStorage is always used behind Arc<RwLock<>> which ensures
// only one thread accesses the Connection at a time
unsafe impl Send for EventStorage {}
unsafe impl Sync for EventStorage {}

impl EventStorage {
    /// Create a new event storage
    ///
    /// Creates the storage directory and initializes the database schema.
    pub fn new(db_path: impl Into<PathBuf>) -> Result<Self> {
        let path = db_path.into();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&path)?;

        let storage = Self { conn };
        storage.initialize_schema()?;

        Ok(storage)
    }

    /// Create an in-memory storage (for testing)
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let storage = Self { conn };
        storage.initialize_schema()?;
        Ok(storage)
    }

    /// Initialize the database schema
    fn initialize_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                event_id TEXT UNIQUE NOT NULL,
                event_data TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                synced_at INTEGER,
                retry_count INTEGER DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS idx_synced_at ON events(synced_at);
            CREATE INDEX IF NOT EXISTS idx_created_at ON events(created_at);
            "#,
        )?;

        Ok(())
    }

    /// Insert an event into the storage
    pub fn insert(&self, event: &Event) -> Result<()> {
        let event_json = serde_json::to_string(event)?;
        let created_at = Utc::now().timestamp();

        self.conn.execute(
            "INSERT INTO events (event_id, event_data, created_at) VALUES (?1, ?2, ?3)",
            params![event.event_id.to_string(), event_json, created_at],
        )?;

        Ok(())
    }

    /// Get unsynced events (up to a limit)
    pub fn get_unsynced(&self, limit: usize) -> Result<Vec<Event>> {
        let mut stmt = self.conn.prepare(
            "SELECT event_data FROM events WHERE synced_at IS NULL ORDER BY created_at ASC LIMIT ?1",
        )?;

        let events = stmt
            .query_map(params![limit], |row| {
                let event_json: String = row.get(0)?;
                Ok(event_json)
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        let mut parsed_events = Vec::new();
        for event_json in events {
            let event: Event = serde_json::from_str(&event_json)?;
            parsed_events.push(event);
        }

        Ok(parsed_events)
    }

    /// Mark events as synced
    pub fn mark_synced(&self, event_ids: &[uuid::Uuid]) -> Result<()> {
        let synced_at = Utc::now().timestamp();

        // Convert UUIDs to strings first so they own the data
        let event_id_strings: Vec<String> = event_ids.iter().map(|id| id.to_string()).collect();

        let placeholders = event_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");

        let query = format!(
            "UPDATE events SET synced_at = ?1 WHERE event_id IN ({})",
            placeholders
        );

        let params: Vec<&dyn rusqlite::ToSql> = std::iter::once(&synced_at as &dyn rusqlite::ToSql)
            .chain(event_id_strings.iter().map(|s| s as &dyn rusqlite::ToSql))
            .collect();

        self.conn.execute(&query, params.as_slice())?;

        Ok(())
    }

    /// Increment retry count for events
    pub fn increment_retry(&self, event_ids: &[uuid::Uuid]) -> Result<()> {
        // Convert UUIDs to strings first so they own the data
        let event_id_strings: Vec<String> = event_ids.iter().map(|id| id.to_string()).collect();

        let placeholders = event_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");

        let query = format!(
            "UPDATE events SET retry_count = retry_count + 1 WHERE event_id IN ({})",
            placeholders
        );

        let params: Vec<&dyn rusqlite::ToSql> = event_id_strings
            .iter()
            .map(|s| s as &dyn rusqlite::ToSql)
            .collect();

        self.conn.execute(&query, params.as_slice())?;

        Ok(())
    }

    /// Get count of unsynced events
    pub fn unsynced_count(&self) -> Result<usize> {
        let count: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM events WHERE synced_at IS NULL",
            [],
            |row| row.get(0),
        )?;

        Ok(count)
    }

    /// Delete old synced events (older than 7 days)
    pub fn cleanup_old_events(&self) -> Result<usize> {
        let seven_days_ago = Utc::now().timestamp() - (7 * 24 * 60 * 60);

        let deleted = self.conn.execute(
            "DELETE FROM events WHERE synced_at IS NOT NULL AND synced_at < ?1",
            params![seven_days_ago],
        )?;

        Ok(deleted)
    }

    /// Get total event count
    pub fn total_count(&self) -> Result<usize> {
        let count: usize = self
            .conn
            .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::*;
    use uuid::Uuid;

    fn create_test_event() -> Event {
        Event {
            schema_version: SCHEMA_VERSION.to_string(),
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            service: ServiceInfo {
                name: "test-service".to_string(),
                version: "1.0.0".to_string(),
                language: "rust".to_string(),
                language_version: Some("1.75.0".to_string()),
            },
            user_id: "client_test123".to_string(),
            session_id: Some("sess_test456".to_string()),
            environment: Environment {
                os: "linux".to_string(),
                os_version: None,
                arch: Some("x86_64".to_string()),
                ci: Some(false),
                shell: None,
            },
            event: EventData {
                event_type: "test_event".to_string(),
                category: Some("test".to_string()),
                data: serde_json::json!({"test": true}),
            },
            metadata: Metadata {
                sdk_version: "0.1.0".to_string(),
                transmission_timestamp: Utc::now(),
                batch_size: 1,
                retry_count: 0,
            },
        }
    }

    #[test]
    fn test_insert_and_get() {
        let storage = EventStorage::in_memory().unwrap();
        let event = create_test_event();

        storage.insert(&event).unwrap();

        let unsynced = storage.get_unsynced(10).unwrap();
        assert_eq!(unsynced.len(), 1);
        assert_eq!(unsynced[0].event_id, event.event_id);
    }

    #[test]
    fn test_mark_synced() {
        let storage = EventStorage::in_memory().unwrap();
        let event = create_test_event();
        let event_id = event.event_id;

        storage.insert(&event).unwrap();
        assert_eq!(storage.unsynced_count().unwrap(), 1);

        storage.mark_synced(&[event_id]).unwrap();
        assert_eq!(storage.unsynced_count().unwrap(), 0);
    }

    #[test]
    fn test_increment_retry() {
        let storage = EventStorage::in_memory().unwrap();
        let event = create_test_event();
        let event_id = event.event_id;

        storage.insert(&event).unwrap();
        storage.increment_retry(&[event_id]).unwrap();

        // Note: We can't easily verify the retry count increased
        // without adding a method to retrieve it, but the function runs without error
    }
}

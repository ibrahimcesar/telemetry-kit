//! User identification utilities

use crate::error::{Result, TelemetryError};
use machine_uid;
use sha2::{Digest, Sha256};

const SALT: &str = "telemetry-kit-v1";

/// Generate an anonymous user ID based on machine ID
///
/// Format: `client_<first_64_chars_of_hash>`
///
/// This creates a stable, anonymous identifier that:
/// - Persists across runs on the same machine
/// - Cannot be reverse-engineered to get the machine ID
/// - Does not contain any PII
pub fn generate_user_id() -> Result<String> {
    let machine_id = machine_uid::get()
        .map_err(|e| TelemetryError::MachineId(format!("Failed to get machine ID: {}", e)))?;

    let mut hasher = Sha256::new();
    hasher.update(machine_id.as_bytes());
    hasher.update(SALT.as_bytes());
    let hash = hasher.finalize();

    let hex_hash = hex::encode(hash);
    Ok(format!("client_{}", &hex_hash[..64]))
}

/// Generate a session ID
///
/// Format: `sess_<uuid>`
///
/// Sessions are unique per process run and help group events from a single execution.
pub fn generate_session_id() -> String {
    let uuid = uuid::Uuid::new_v4();
    format!("sess_{}", uuid.simple())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_generation() {
        let user_id = generate_user_id().unwrap();
        assert!(user_id.starts_with("client_"));
        assert_eq!(user_id.len(), 7 + 64); // "client_" + 64 hex chars
    }

    #[test]
    fn test_user_id_stability() {
        // User ID should be the same across calls
        let id1 = generate_user_id().unwrap();
        let id2 = generate_user_id().unwrap();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_session_id_generation() {
        let session_id = generate_session_id();
        assert!(session_id.starts_with("sess_"));
    }

    #[test]
    fn test_session_id_uniqueness() {
        // Session IDs should be different
        let id1 = generate_session_id();
        let id2 = generate_session_id();
        assert_ne!(id1, id2);
    }
}

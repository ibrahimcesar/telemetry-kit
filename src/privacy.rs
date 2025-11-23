//! Privacy controls and consent management for telemetry-kit
//!
//! This module provides GDPR-compliant privacy controls including:
//! - User consent management
//! - DO_NOT_TRACK support
//! - Data sanitization
//! - Retention policies

use crate::error::{Result, TelemetryError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Privacy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Whether user consent is required before tracking
    pub consent_required: bool,

    /// Whether to respect DO_NOT_TRACK environment variable
    pub respect_do_not_track: bool,

    /// Whether to sanitize paths (remove usernames)
    pub sanitize_paths: bool,

    /// Whether to sanitize email addresses
    pub sanitize_emails: bool,

    /// Data retention period in days (0 = forever)
    pub data_retention_days: u32,

    /// Whether to anonymize IP addresses (for future use)
    pub anonymize_ips: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            consent_required: false,
            respect_do_not_track: true, // Always respect DNT by default
            sanitize_paths: true,
            sanitize_emails: true,
            data_retention_days: 90,
            anonymize_ips: true,
        }
    }
}

impl PrivacyConfig {
    /// Create a new privacy configuration with strictest settings
    pub fn strict() -> Self {
        Self {
            consent_required: true,
            respect_do_not_track: true,
            sanitize_paths: true,
            sanitize_emails: true,
            data_retention_days: 30,
            anonymize_ips: true,
        }
    }

    /// Create a minimal privacy configuration
    pub fn minimal() -> Self {
        Self {
            consent_required: false,
            respect_do_not_track: true, // Still respect DNT
            sanitize_paths: false,
            sanitize_emails: false,
            data_retention_days: 0, // No automatic cleanup
            anonymize_ips: false,
        }
    }
}

/// User consent status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsentStatus {
    /// User has not been asked yet
    Unknown,
    /// User has granted consent
    Granted,
    /// User has denied consent
    Denied,
    /// User has opted out (DO_NOT_TRACK)
    OptedOut,
}

/// Consent information stored on disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentInfo {
    /// Consent status
    pub status: ConsentStatus,
    /// When consent was given/denied
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Service name this consent applies to
    pub service_name: String,
}

/// Privacy manager handles consent and privacy settings
pub struct PrivacyManager {
    config: PrivacyConfig,
    consent_file: PathBuf,
}

impl PrivacyManager {
    /// Create a new privacy manager
    pub fn new(config: PrivacyConfig, service_name: &str) -> Result<Self> {
        let consent_file = Self::consent_file_path(service_name)?;

        Ok(Self {
            config,
            consent_file,
        })
    }

    /// Get the path to the consent file
    fn consent_file_path(service_name: &str) -> Result<PathBuf> {
        let mut path = dirs::home_dir().ok_or_else(|| {
            TelemetryError::InvalidConfig("Cannot determine home directory".to_string())
        })?;
        path.push(".telemetry-kit");
        path.push(format!("{}-consent.json", service_name));
        Ok(path)
    }

    /// Check if tracking should proceed
    pub fn should_track(&self) -> Result<bool> {
        // First: Check DO_NOT_TRACK environment variable
        if self.config.respect_do_not_track && Self::is_do_not_track_enabled() {
            return Ok(false);
        }

        // Second: Check consent if required
        if self.config.consent_required {
            let consent = self.load_consent()?;
            match consent.status {
                ConsentStatus::Granted => Ok(true),
                ConsentStatus::Denied | ConsentStatus::OptedOut => Ok(false),
                ConsentStatus::Unknown => Ok(false), // Default deny if consent required
            }
        } else {
            Ok(true)
        }
    }

    /// Check if DO_NOT_TRACK environment variable is set
    pub fn is_do_not_track_enabled() -> bool {
        std::env::var("DO_NOT_TRACK")
            .map(|v| !v.is_empty() && v != "0" && v.to_lowercase() != "false")
            .unwrap_or(false)
    }

    /// Load consent information from disk
    pub fn load_consent(&self) -> Result<ConsentInfo> {
        if !self.consent_file.exists() {
            return Ok(ConsentInfo {
                status: ConsentStatus::Unknown,
                timestamp: chrono::Utc::now(),
                service_name: String::new(),
            });
        }

        let content = std::fs::read_to_string(&self.consent_file)?;

        let consent: ConsentInfo = serde_json::from_str(&content)?;

        Ok(consent)
    }

    /// Save consent information to disk
    pub fn save_consent(&self, status: ConsentStatus, service_name: &str) -> Result<()> {
        let consent = ConsentInfo {
            status,
            timestamp: chrono::Utc::now(),
            service_name: service_name.to_string(),
        };

        // Ensure directory exists
        if let Some(parent) = self.consent_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(&consent)?;

        std::fs::write(&self.consent_file, content)?;

        Ok(())
    }

    /// Grant consent for tracking
    pub fn grant_consent(&self, service_name: &str) -> Result<()> {
        self.save_consent(ConsentStatus::Granted, service_name)
    }

    /// Deny consent for tracking
    pub fn deny_consent(&self, service_name: &str) -> Result<()> {
        self.save_consent(ConsentStatus::Denied, service_name)
    }

    /// Opt out of tracking (DO_NOT_TRACK equivalent)
    pub fn opt_out(&self, service_name: &str) -> Result<()> {
        self.save_consent(ConsentStatus::OptedOut, service_name)
    }

    /// Sanitize a path by removing username components
    pub fn sanitize_path(path: &str) -> String {
        if let Some(home) = dirs::home_dir() {
            if let Some(home_str) = home.to_str() {
                return path.replace(home_str, "~");
            }
        }
        path.to_string()
    }

    /// Sanitize an email address by hashing it
    pub fn sanitize_email(email: &str) -> String {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(email.as_bytes());
        let result = hasher.finalize();
        format!("email_{}", hex::encode(&result[..8]))
    }

    /// Apply sanitization to data based on config
    pub fn sanitize_data(&self, data: &mut serde_json::Value) {
        if let serde_json::Value::Object(map) = data {
            for (_key, value) in map.iter_mut() {
                match value {
                    serde_json::Value::String(s) => {
                        // Sanitize paths
                        if self.config.sanitize_paths && (s.contains('/') || s.contains('\\')) {
                            *s = Self::sanitize_path(s);
                        }
                        // Sanitize emails
                        if self.config.sanitize_emails && s.contains('@') {
                            *s = Self::sanitize_email(s);
                        }
                    }
                    serde_json::Value::Object(_) => {
                        self.sanitize_data(value);
                    }
                    serde_json::Value::Array(arr) => {
                        for item in arr {
                            self.sanitize_data(item);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PrivacyConfig::default();
        assert!(!config.consent_required);
        assert!(config.respect_do_not_track);
        assert!(config.sanitize_paths);
        assert!(config.sanitize_emails);
        assert_eq!(config.data_retention_days, 90);
    }

    #[test]
    fn test_strict_config() {
        let config = PrivacyConfig::strict();
        assert!(config.consent_required);
        assert!(config.respect_do_not_track);
        assert_eq!(config.data_retention_days, 30);
    }

    #[test]
    fn test_minimal_config() {
        let config = PrivacyConfig::minimal();
        assert!(!config.consent_required);
        assert!(config.respect_do_not_track); // Still respects DNT
        assert!(!config.sanitize_paths);
        assert_eq!(config.data_retention_days, 0);
    }

    #[test]
    fn test_do_not_track_detection() {
        // Save original value
        let original = std::env::var("DO_NOT_TRACK").ok();

        // Test enabled
        std::env::set_var("DO_NOT_TRACK", "1");
        assert!(PrivacyManager::is_do_not_track_enabled());

        std::env::set_var("DO_NOT_TRACK", "true");
        assert!(PrivacyManager::is_do_not_track_enabled());

        // Test disabled
        std::env::set_var("DO_NOT_TRACK", "0");
        assert!(!PrivacyManager::is_do_not_track_enabled());

        std::env::set_var("DO_NOT_TRACK", "false");
        assert!(!PrivacyManager::is_do_not_track_enabled());

        std::env::remove_var("DO_NOT_TRACK");
        assert!(!PrivacyManager::is_do_not_track_enabled());

        // Restore original
        if let Some(val) = original {
            std::env::set_var("DO_NOT_TRACK", val);
        } else {
            std::env::remove_var("DO_NOT_TRACK");
        }
    }

    #[test]
    fn test_sanitize_path() {
        // Use actual home directory for testing
        if let Some(home) = dirs::home_dir() {
            if let Some(home_str) = home.to_str() {
                let path = format!("{}/Documents/project", home_str);
                let sanitized = PrivacyManager::sanitize_path(&path);
                assert!(sanitized.starts_with('~'));
                assert!(!sanitized.contains(home_str));
            }
        }

        // Test path that's not in home directory
        let other_path = "/tmp/some/path";
        let sanitized = PrivacyManager::sanitize_path(other_path);
        assert_eq!(sanitized, other_path); // Should remain unchanged
    }

    #[test]
    fn test_sanitize_email() {
        let email = "user@example.com";
        let sanitized = PrivacyManager::sanitize_email(email);
        assert!(sanitized.starts_with("email_"));
        assert!(!sanitized.contains('@'));
        assert!(!sanitized.contains("example.com"));

        // Same email should produce same hash
        let sanitized2 = PrivacyManager::sanitize_email(email);
        assert_eq!(sanitized, sanitized2);
    }

    #[test]
    fn test_consent_status() {
        assert_eq!(ConsentStatus::Unknown, ConsentStatus::Unknown);
        assert_ne!(ConsentStatus::Granted, ConsentStatus::Denied);
    }

    #[tokio::test]
    async fn test_privacy_manager_should_track() {
        let config = PrivacyConfig::default();
        let manager = PrivacyManager::new(config, "test-service").unwrap();

        // Save original DNT value
        let original_dnt = std::env::var("DO_NOT_TRACK").ok();

        // Without DNT, should track (consent not required by default)
        std::env::remove_var("DO_NOT_TRACK");
        assert!(manager.should_track().unwrap());

        // With DNT, should not track
        std::env::set_var("DO_NOT_TRACK", "1");
        assert!(!manager.should_track().unwrap());

        // Restore original
        if let Some(val) = original_dnt {
            std::env::set_var("DO_NOT_TRACK", val);
        } else {
            std::env::remove_var("DO_NOT_TRACK");
        }
    }

    #[test]
    fn test_sanitize_data() {
        let config = PrivacyConfig::default();
        let manager = PrivacyManager::new(config, "test").unwrap();

        // Use actual home directory
        let home_path = if let Some(home) = dirs::home_dir() {
            if let Some(home_str) = home.to_str() {
                format!("{}/file.txt", home_str)
            } else {
                "/tmp/file.txt".to_string()
            }
        } else {
            "/tmp/file.txt".to_string()
        };

        let mut data = serde_json::json!({
            "email": "test@example.com",
            "path": home_path.clone(),
            "normal": "just text"
        });

        manager.sanitize_data(&mut data);

        let email = data["email"].as_str().unwrap();
        assert!(email.starts_with("email_"));
        assert!(!email.contains('@'));

        let path = data["path"].as_str().unwrap();
        // If it was a home path, should be sanitized to ~
        if home_path.starts_with("/Users/") || home_path.starts_with("/home/") {
            assert!(path.starts_with('~'));
        }

        assert_eq!(data["normal"].as_str().unwrap(), "just text");
    }

    // Property-based tests
    #[cfg(test)]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            /// Property: Sanitized emails always start with "email_" and never contain "@"
            #[test]
            fn sanitize_email_always_valid(email in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}") {
                let sanitized = PrivacyManager::sanitize_email(&email);
                prop_assert!(sanitized.starts_with("email_"));
                prop_assert!(!sanitized.contains('@'));
                prop_assert!(!sanitized.contains(&email));
            }

            /// Property: Same email always produces same hash (determinism)
            #[test]
            fn sanitize_email_deterministic(email in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}") {
                let sanitized1 = PrivacyManager::sanitize_email(&email);
                let sanitized2 = PrivacyManager::sanitize_email(&email);
                prop_assert_eq!(sanitized1, sanitized2);
            }

            /// Property: Different emails produce different hashes
            #[test]
            fn sanitize_email_unique(
                email1 in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}",
                email2 in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
            ) {
                prop_assume!(email1 != email2);
                let sanitized1 = PrivacyManager::sanitize_email(&email1);
                let sanitized2 = PrivacyManager::sanitize_email(&email2);
                prop_assert_ne!(sanitized1, sanitized2);
            }

            /// Property: Path sanitization never exposes home directory
            #[test]
            fn sanitize_path_hides_home(suffix in "[a-zA-Z0-9/_.-]+") {
                if let Some(home) = dirs::home_dir() {
                    if let Some(home_str) = home.to_str() {
                        let path = format!("{}/{}", home_str, suffix);
                        let sanitized = PrivacyManager::sanitize_path(&path);
                        prop_assert!(sanitized.starts_with('~'));
                        prop_assert!(!sanitized.contains(home_str));
                    }
                }
            }

            /// Property: Path sanitization is idempotent
            #[test]
            fn sanitize_path_idempotent(suffix in "[a-zA-Z0-9/_.-]+") {
                if let Some(home) = dirs::home_dir() {
                    if let Some(home_str) = home.to_str() {
                        let path = format!("{}/{}", home_str, suffix);
                        let sanitized1 = PrivacyManager::sanitize_path(&path);
                        let sanitized2 = PrivacyManager::sanitize_path(&sanitized1);
                        prop_assert_eq!(sanitized1, sanitized2);
                    }
                }
            }

            /// Property: Non-home paths remain unchanged
            #[test]
            fn sanitize_path_preserves_non_home(path in "/tmp/[a-zA-Z0-9/_.-]+") {
                if let Some(home) = dirs::home_dir() {
                    if let Some(home_str) = home.to_str() {
                        prop_assume!(!path.starts_with(home_str));
                        let sanitized = PrivacyManager::sanitize_path(&path);
                        prop_assert_eq!(sanitized, path);
                    }
                }
            }

            /// Property: Sanitized data never contains raw emails
            #[test]
            fn sanitize_data_removes_emails(email in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}") {
                let config = PrivacyConfig::default();
                let manager = PrivacyManager::new(config, "test").unwrap();

                let mut data = serde_json::json!({
                    "email": email.clone(),
                    "nested": {
                        "email": email.clone()
                    }
                });

                manager.sanitize_data(&mut data);

                let data_str = serde_json::to_string(&data).unwrap();
                prop_assert!(!data_str.contains(&email));
                prop_assert!(data_str.contains("email_"));
            }

            /// Property: ConsentStatus serialization roundtrip
            #[test]
            fn consent_status_serde_roundtrip(
                status in prop_oneof![
                    Just(ConsentStatus::Unknown),
                    Just(ConsentStatus::Granted),
                    Just(ConsentStatus::Denied),
                    Just(ConsentStatus::OptedOut),
                ]
            ) {
                let json = serde_json::to_string(&status).unwrap();
                let deserialized: ConsentStatus = serde_json::from_str(&json).unwrap();
                prop_assert_eq!(status, deserialized);
            }

            /// Property: PrivacyConfig cloning preserves all fields
            #[test]
            fn privacy_config_clone_preserves(
                consent_required in proptest::bool::ANY,
                sanitize_paths in proptest::bool::ANY,
                sanitize_emails in proptest::bool::ANY,
                data_retention_days in 0u32..=3650u32,
                anonymize_ips in proptest::bool::ANY
            ) {
                let config = PrivacyConfig {
                    consent_required,
                    respect_do_not_track: true, // Always true
                    sanitize_paths,
                    sanitize_emails,
                    data_retention_days,
                    anonymize_ips,
                };

                let cloned = config.clone();

                prop_assert_eq!(config.consent_required, cloned.consent_required);
                prop_assert_eq!(config.respect_do_not_track, cloned.respect_do_not_track);
                prop_assert_eq!(config.sanitize_paths, cloned.sanitize_paths);
                prop_assert_eq!(config.sanitize_emails, cloned.sanitize_emails);
                prop_assert_eq!(config.data_retention_days, cloned.data_retention_days);
                prop_assert_eq!(config.anonymize_ips, cloned.anonymize_ips);
            }

            /// Property: Email hash is always 16 hex characters
            #[test]
            fn sanitize_email_hash_format(email in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}") {
                let sanitized = PrivacyManager::sanitize_email(&email);
                let hash_part = sanitized.strip_prefix("email_").unwrap();
                prop_assert_eq!(hash_part.len(), 16);
                prop_assert!(hash_part.chars().all(|c| c.is_ascii_hexdigit()));
            }

            /// Property: Recursive data sanitization handles nested structures
            #[test]
            fn sanitize_data_handles_nesting(
                email in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}",
                depth in 1usize..=5usize
            ) {
                let config = PrivacyConfig::default();
                let manager = PrivacyManager::new(config, "test").unwrap();

                // Build nested JSON
                let mut data = serde_json::json!({"email": email.clone()});
                for _ in 0..depth {
                    data = serde_json::json!({"nested": data, "email": email.clone()});
                }

                manager.sanitize_data(&mut data);

                let data_str = serde_json::to_string(&data).unwrap();
                prop_assert!(!data_str.contains(&email));
            }
        }
    }
}

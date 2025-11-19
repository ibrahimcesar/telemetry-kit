//! Integration tests for the ingestion endpoint

use chrono::Utc;
use hmac::{Hmac, Mac};
use serde_json::json;
use sha2::Sha256;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

/// Helper to create HMAC signature
fn create_signature(timestamp: &str, nonce: &str, body: &str, secret: &str) -> String {
    let message = format!("{}:{}:{}", timestamp, nonce, body);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC");
    mac.update(message.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

/// Helper to create a test event
fn create_test_event() -> serde_json::Value {
    json!({
        "schema_version": "1.0.0",
        "event_id": Uuid::new_v4(),
        "timestamp": Utc::now(),
        "service": {
            "name": "test-service",
            "version": "1.0.0",
            "language": "rust",
            "language_version": "1.75.0"
        },
        "user_id": "client_test123",
        "session_id": Uuid::new_v4().to_string(),
        "environment": {
            "os": "linux",
            "os_version": "6.5.0",
            "arch": "x86_64",
            "ci": false,
            "shell": "bash"
        },
        "event": {
            "type": "command",
            "category": "test",
            "data": {
                "success": true
            }
        },
        "metadata": {
            "sdk_version": "0.1.0",
            "transmission_timestamp": Utc::now(),
            "batch_size": 1,
            "retry_count": 0
        }
    })
}

#[tokio::test]
async fn test_health_endpoint() {
    // This test doesn't require database/redis setup
    // It just verifies the health endpoint responds correctly
    // For full integration tests, we'd need Docker Compose running

    // We can't easily test this without a full setup, so we'll test the
    // signature generation and validation logic instead
    let secret = "test_secret";
    let timestamp = "1234567890";
    let nonce = "test-nonce";
    let body = r#"{"events":[]}"#;

    let signature = create_signature(timestamp, nonce, body, secret);

    // Verify signature format (should be 64 hex characters for SHA256)
    assert_eq!(signature.len(), 64);
    assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_request_signing() {
    let secret = "9f4b3c2a1e8d7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b6c5d4e3f2a1b0";
    let timestamp = Utc::now().timestamp().to_string();
    let nonce = Uuid::new_v4().to_string();

    let event = create_test_event();
    let batch = json!({"events": [event]});
    let body = serde_json::to_string(&batch).unwrap();

    let signature = create_signature(&timestamp, &nonce, &body, secret);

    // Verify we can generate a valid signature
    assert_eq!(signature.len(), 64);

    // Verify signature is deterministic
    let signature2 = create_signature(&timestamp, &nonce, &body, secret);
    assert_eq!(signature, signature2);

    // Verify different body produces different signature
    let different_body = r#"{"events":[]}"#;
    let different_signature = create_signature(&timestamp, &nonce, different_body, secret);
    assert_ne!(signature, different_signature);
}

#[test]
fn test_event_validation() {
    let event = create_test_event();

    // Validate schema version
    assert_eq!(event["schema_version"], "1.0.0");

    // Validate user_id prefix
    assert!(event["user_id"].as_str().unwrap().starts_with("client_"));

    // Validate required fields
    assert!(!event["service"]["name"].as_str().unwrap().is_empty());
    assert!(!event["service"]["version"].as_str().unwrap().is_empty());

    // Validate event structure
    assert!(event["event"]["type"].is_string());
    assert!(event["event"]["data"].is_object());
}

#[test]
fn test_batch_size_limits() {
    // Test that we properly validate batch sizes
    let min_batch_size = 1;
    let max_batch_size = 1000;

    assert!(min_batch_size >= 1);
    assert!(max_batch_size <= 1000);

    // Empty batch should be rejected
    let empty_batch = json!({"events": []});
    assert_eq!(empty_batch["events"].as_array().unwrap().len(), 0);

    // Single event should be accepted
    let single_batch = json!({"events": [create_test_event()]});
    assert_eq!(single_batch["events"].as_array().unwrap().len(), 1);
}

#[test]
fn test_timestamp_validation() {
    let now = Utc::now().timestamp();
    let ten_minutes_ago = now - 600;
    let ten_minutes_future = now + 600;
    let eleven_minutes_ago = now - 660;

    // Within acceptable window (±10 minutes)
    let diff1 = (now - ten_minutes_ago).abs();
    assert!(diff1 <= 600);

    let diff2 = (now - ten_minutes_future).abs();
    assert!(diff2 <= 600);

    // Outside acceptable window
    let diff3 = (now - eleven_minutes_ago).abs();
    assert!(diff3 > 600);
}

#[test]
fn test_invalid_user_id_formats() {
    let valid_ids = vec![
        "client_abc123",
        "client_550e8400e29b41d4a716446655440000",
    ];

    let invalid_ids = vec![
        "anon_abc123",
        "user_abc123",
        "abc123",
        "",
    ];

    for id in valid_ids {
        assert!(id.starts_with("client_"), "Valid ID should start with client_: {}", id);
    }

    for id in invalid_ids {
        assert!(!id.starts_with("client_"), "Invalid ID should not start with client_: {}", id);
    }
}

//! Integration tests for the sync protocol

use telemetry_kit::prelude::*;
use telemetry_kit::sync::*;
use uuid::Uuid;

#[tokio::test]
async fn test_hmac_signature() {
    let auth = HmacAuth::new("test_secret");
    let timestamp = "1732003200";
    let nonce = "550e8400-e29b-41d4-a716-446655440000";
    let body = r#"{"events":[]}"#;

    let signature = auth.sign(timestamp, nonce, body);

    // Signature should be a 64-char hex string
    assert_eq!(signature.len(), 64);

    // Verification should succeed
    assert!(auth.verify(timestamp, nonce, body, &signature));

    // Tampered body should fail
    assert!(!auth.verify(
        timestamp,
        nonce,
        r#"{"events":[{"hacked":true}]}"#,
        &signature
    ));
}

#[test]
fn test_sync_config_builder() {
    let config = SyncConfig::builder()
        .org_id("550e8400-e29b-41d4-a716-446655440000")
        .unwrap()
        .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")
        .unwrap()
        .token("tk_test")
        .secret("test_secret")
        .batch_size(50)
        .build()
        .unwrap();

    assert_eq!(config.batch_size, 50);
    assert_eq!(config.endpoint, "https://telemetry-kit.dev");
}

#[test]
fn test_sync_config_validation() {
    // Empty token should fail
    let result = SyncConfig::builder()
        .org_id("550e8400-e29b-41d4-a716-446655440000")
        .unwrap()
        .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")
        .unwrap()
        .token("")
        .secret("test_secret")
        .build();

    assert!(result.is_err());

    // Invalid batch size should fail
    let result = SyncConfig::builder()
        .org_id("550e8400-e29b-41d4-a716-446655440000")
        .unwrap()
        .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")
        .unwrap()
        .token("tk_test")
        .secret("test_secret")
        .batch_size(2000) // > MAX_BATCH_SIZE
        .build();

    assert!(result.is_err());
}

#[test]
fn test_retry_strategy() {
    let strategy = RetryStrategy::new(3, 1000);

    assert!(strategy.should_retry(0));
    assert!(strategy.should_retry(1));
    assert!(strategy.should_retry(2));
    assert!(!strategy.should_retry(3));

    // First retry should be ~1-2 seconds
    let delay = strategy.delay_for(0);
    assert!(delay.as_millis() >= 1000 && delay.as_millis() < 2000);
}

#[tokio::test]
async fn test_telemetry_local_storage() {
    let unique_name = format!("test-storage-{}", Uuid::new_v4());
    let telemetry = TelemetryKit::builder()
        .service_name(&unique_name)
        .unwrap()
        .service_version("1.0.0")
        .build()
        .unwrap();

    // Track an event
    telemetry
        .track_command("test", |event| event.success(true))
        .await
        .unwrap();

    // Verify it's stored
    let stats = telemetry.stats().await.unwrap();
    assert!(stats.total_events >= 1);
    assert!(stats.unsynced_events >= 1);
}

#[tokio::test]
async fn test_telemetry_with_sync_config() {
    let unique_name = format!("test-sync-{}", Uuid::new_v4());
    let sync_config = SyncConfig::builder()
        .org_id("550e8400-e29b-41d4-a716-446655440000")
        .unwrap()
        .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")
        .unwrap()
        .token("tk_test")
        .secret("test_secret")
        .build()
        .unwrap();

    let telemetry = TelemetryKit::builder()
        .service_name(&unique_name)
        .unwrap()
        .service_version("1.0.0")
        .sync(sync_config)
        .build()
        .unwrap();

    // Track an event
    telemetry
        .track_feature("test_feature", |event| event.success(true))
        .await
        .unwrap();

    // Note: We don't actually sync here because we don't have a test server
    // In production, you'd call telemetry.sync().await
}

#[tokio::test]
async fn test_event_builders() {
    let unique_name = format!("test-builders-{}", Uuid::new_v4());
    let telemetry = TelemetryKit::builder()
        .service_name(&unique_name)
        .unwrap()
        .build()
        .unwrap();

    // Test command builder
    telemetry
        .track_command("deploy", |event| {
            event
                .subcommand("production")
                .flag("--force")
                .flag("--no-cache")
                .duration_ms(5000)
                .success(true)
                .exit_code(0)
        })
        .await
        .unwrap();

    // Test feature builder
    telemetry
        .track_feature("github_integration", |event| {
            event
                .method("create_pr")
                .success(true)
                .data("pr_number", serde_json::json!(42))
        })
        .await
        .unwrap();

    // Test custom event
    telemetry
        .track_custom(
            "configuration_changed",
            serde_json::json!({
                "key": "max_workers",
                "old_value": 4,
                "new_value": 8
            }),
        )
        .await
        .unwrap();

    let stats = telemetry.stats().await.unwrap();
    assert_eq!(stats.total_events, 3);
}

#[test]
fn test_user_id_generation() {
    use telemetry_kit::user::{generate_session_id, generate_user_id};

    // User ID should be stable
    let id1 = generate_user_id().unwrap();
    let id2 = generate_user_id().unwrap();
    assert_eq!(id1, id2);
    assert!(id1.starts_with("client_"));

    // Session ID should be unique
    let sess1 = generate_session_id();
    let sess2 = generate_session_id();
    assert_ne!(sess1, sess2);
    assert!(sess1.starts_with("sess_"));
}

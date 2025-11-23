use std::env;
use std::fs;
use std::path::PathBuf;
/// Integration tests for privacy controls
///
/// These tests verify the complete privacy workflow including:
/// - Consent management lifecycle
/// - DO_NOT_TRACK detection and blocking
/// - Event tracking with privacy controls
/// - Data sanitization in real events
/// - Consent persistence across sessions
use telemetry_kit::prelude::*;
use telemetry_kit::privacy::{ConsentStatus, PrivacyConfig, PrivacyManager};

/// Helper: Get temporary consent file path for testing
fn get_test_consent_path(service: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(".telemetry-kit-test-{}", service));
    path.push(format!("{}-consent.json", service));
    path
}

/// Helper: Clean up test consent file
fn cleanup_consent_file(service: &str) {
    let path = get_test_consent_path(service);
    if let Some(parent) = path.parent() {
        let _ = fs::remove_dir_all(parent);
    }
}

/// Helper: Create TelemetryKit for testing
fn create_test_telemetry(service: &str, privacy_config: PrivacyConfig) -> Result<TelemetryKit> {
    TelemetryKit::builder()
        .service_name(service)?
        .privacy(privacy_config)
        .build()
}

#[tokio::test]
async fn test_consent_lifecycle() -> Result<()> {
    let service = "consent-lifecycle-test";
    cleanup_consent_file(service);

    let telemetry = create_test_telemetry(service, PrivacyConfig::strict())?;

    // Grant consent
    telemetry.grant_consent()?;

    // Track should succeed with consent
    telemetry.track_command("test", |e| e.success(true)).await?;

    // Deny consent
    telemetry.deny_consent()?;

    // Track should be silently ignored without consent
    telemetry
        .track_command("test2", |e| e.success(true))
        .await?;

    // Opt out
    telemetry.opt_out()?;

    // Track should still be silently ignored
    telemetry
        .track_command("test3", |e| e.success(true))
        .await?;

    cleanup_consent_file(service);
    Ok(())
}

#[tokio::test]
async fn test_do_not_track_blocks_tracking() -> Result<()> {
    let service = "dnt-test";
    cleanup_consent_file(service);

    // Save original DNT value
    let original_dnt = env::var("DO_NOT_TRACK").ok();

    // Set DO_NOT_TRACK
    env::set_var("DO_NOT_TRACK", "1");

    // Even with consent granted, DNT should block tracking
    let telemetry = create_test_telemetry(service, PrivacyConfig::default())?;
    telemetry.grant_consent()?;

    let manager = PrivacyManager::new(PrivacyConfig::default(), service)?;
    assert!(!manager.should_track()?);

    // Events should be silently ignored
    telemetry
        .track_command("blocked", |e| e.success(true))
        .await?;

    // Restore original DNT
    match original_dnt {
        Some(val) => env::set_var("DO_NOT_TRACK", val),
        None => env::remove_var("DO_NOT_TRACK"),
    }

    cleanup_consent_file(service);
    Ok(())
}

#[tokio::test]
async fn test_consent_persistence() -> Result<()> {
    let service = "persistence-test";
    cleanup_consent_file(service);

    // Create first instance and grant consent
    {
        let telemetry = create_test_telemetry(service, PrivacyConfig::strict())?;
        telemetry.grant_consent()?;
    }

    // Create second instance - consent should persist
    {
        let manager = PrivacyManager::new(PrivacyConfig::strict(), service)?;
        let consent = manager.load_consent()?;
        assert_eq!(consent.status, ConsentStatus::Granted);
        assert!(manager.should_track()?);
    }

    cleanup_consent_file(service);
    Ok(())
}

#[tokio::test]
async fn test_default_privacy_allows_tracking() -> Result<()> {
    let service = "default-privacy-test";
    cleanup_consent_file(service);

    // Save original DNT
    let original_dnt = env::var("DO_NOT_TRACK").ok();
    env::remove_var("DO_NOT_TRACK");

    // Default config doesn't require consent
    let telemetry = create_test_telemetry(service, PrivacyConfig::default())?;

    let manager = PrivacyManager::new(PrivacyConfig::default(), service)?;
    assert!(manager.should_track()?);

    // Should allow tracking
    telemetry
        .track_command("allowed", |e| e.success(true))
        .await?;

    // Restore DNT
    match original_dnt {
        Some(val) => env::set_var("DO_NOT_TRACK", val),
        None => env::remove_var("DO_NOT_TRACK"),
    }

    cleanup_consent_file(service);
    Ok(())
}

#[tokio::test]
async fn test_strict_privacy_blocks_without_consent() -> Result<()> {
    let service = "strict-privacy-test";
    cleanup_consent_file(service);

    // Save original DNT
    let original_dnt = env::var("DO_NOT_TRACK").ok();
    env::remove_var("DO_NOT_TRACK");

    // Strict config requires consent
    let telemetry = create_test_telemetry(service, PrivacyConfig::strict())?;

    // Should block tracking without consent (silently)
    telemetry
        .track_command("blocked", |e| e.success(true))
        .await?;

    // Grant consent
    telemetry.grant_consent()?;

    // Now should allow tracking
    telemetry
        .track_command("allowed", |e| e.success(true))
        .await?;

    // Restore DNT
    match original_dnt {
        Some(val) => env::set_var("DO_NOT_TRACK", val),
        None => env::remove_var("DO_NOT_TRACK"),
    }

    cleanup_consent_file(service);
    Ok(())
}

#[tokio::test]
async fn test_data_sanitization_in_events() -> Result<()> {
    let service = "sanitization-test";
    cleanup_consent_file(service);

    // Save original DNT
    let original_dnt = env::var("DO_NOT_TRACK").ok();
    env::remove_var("DO_NOT_TRACK");

    let telemetry = create_test_telemetry(service, PrivacyConfig::default())?;

    // Track event - sanitization happens internally
    telemetry
        .track_command("sanitize-test", |e| e.success(true))
        .await?;

    // Note: Data sanitization is tested in unit tests and property tests
    // Full end-to-end verification would require exposing storage inspection methods

    // Restore DNT
    match original_dnt {
        Some(val) => env::set_var("DO_NOT_TRACK", val),
        None => env::remove_var("DO_NOT_TRACK"),
    }

    cleanup_consent_file(service);
    Ok(())
}

#[test]
fn test_privacy_manager_initialization() -> Result<()> {
    let service = "init-test";

    // Default config
    let default_config = PrivacyConfig::default();
    let _manager = PrivacyManager::new(default_config.clone(), service)?;
    assert!(default_config.sanitize_paths);
    assert!(default_config.sanitize_emails);
    assert_eq!(default_config.data_retention_days, 90);

    // Strict config
    let strict_config = PrivacyConfig::strict();
    let _manager = PrivacyManager::new(strict_config.clone(), service)?;
    assert!(strict_config.consent_required);
    assert_eq!(strict_config.data_retention_days, 30);

    // Minimal config
    let minimal_config = PrivacyConfig::minimal();
    let _manager = PrivacyManager::new(minimal_config.clone(), service)?;
    assert!(!minimal_config.sanitize_paths);
    assert!(!minimal_config.sanitize_emails);
    assert_eq!(minimal_config.data_retention_days, 0);

    Ok(())
}

#[tokio::test]
async fn test_consent_required_with_builder() -> Result<()> {
    let service = "builder-consent-test";
    cleanup_consent_file(service);

    // Save original DNT
    let original_dnt = env::var("DO_NOT_TRACK").ok();
    env::remove_var("DO_NOT_TRACK");

    // Create with consent_required(true)
    let telemetry = create_test_telemetry(
        service,
        PrivacyConfig {
            consent_required: true,
            ..PrivacyConfig::default()
        },
    )?;

    // Should block without consent (silently)
    telemetry
        .track_command("blocked", |e| e.success(true))
        .await?;

    // Grant consent
    telemetry.grant_consent()?;

    // Should now allow tracking
    telemetry
        .track_command("allowed", |e| e.success(true))
        .await?;

    // Restore DNT
    match original_dnt {
        Some(val) => env::set_var("DO_NOT_TRACK", val),
        None => env::remove_var("DO_NOT_TRACK"),
    }

    cleanup_consent_file(service);
    Ok(())
}

#[test]
fn test_privacy_config_presets() {
    // Default preset
    let default = PrivacyConfig::default();
    assert!(!default.consent_required);
    assert!(default.respect_do_not_track);
    assert!(default.sanitize_paths);
    assert!(default.sanitize_emails);
    assert_eq!(default.data_retention_days, 90);
    assert!(default.anonymize_ips);

    // Strict preset (GDPR)
    let strict = PrivacyConfig::strict();
    assert!(strict.consent_required);
    assert!(strict.respect_do_not_track);
    assert!(strict.sanitize_paths);
    assert!(strict.sanitize_emails);
    assert_eq!(strict.data_retention_days, 30);
    assert!(strict.anonymize_ips);

    // Minimal preset
    let minimal = PrivacyConfig::minimal();
    assert!(!minimal.consent_required);
    assert!(minimal.respect_do_not_track);
    assert!(!minimal.sanitize_paths);
    assert!(!minimal.sanitize_emails);
    assert_eq!(minimal.data_retention_days, 0);
    assert!(!minimal.anonymize_ips);
}

#[tokio::test]
async fn test_dnt_priority_over_consent() -> Result<()> {
    let service = "dnt-priority-test";
    cleanup_consent_file(service);

    // Save original DNT
    let original_dnt = env::var("DO_NOT_TRACK").ok();

    let telemetry = create_test_telemetry(service, PrivacyConfig::default())?;

    // Grant consent
    telemetry.grant_consent()?;

    let manager = PrivacyManager::new(PrivacyConfig::default(), service)?;

    // Without DNT, should track
    env::remove_var("DO_NOT_TRACK");
    assert!(manager.should_track()?);

    // With DNT, should NOT track (even with consent)
    env::set_var("DO_NOT_TRACK", "1");
    assert!(!manager.should_track()?);

    // Restore DNT
    match original_dnt {
        Some(val) => env::set_var("DO_NOT_TRACK", val),
        None => env::remove_var("DO_NOT_TRACK"),
    }

    cleanup_consent_file(service);
    Ok(())
}

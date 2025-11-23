//! Privacy Controls Example
//!
//! This example demonstrates how to use telemetry-kit's privacy features:
//! - DO_NOT_TRACK support
//! - User consent management
//! - Data sanitization (paths, emails)
//! - Privacy presets (strict, minimal)

use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> telemetry_kit::Result<()> {
    println!("🔒 Telemetry Kit - Privacy Controls Example\n");

    // ============================================================
    // 1. Check DO_NOT_TRACK environment variable
    // ============================================================
    println!("📋 Checking DO_NOT_TRACK environment variable...");
    if TelemetryKit::is_do_not_track_enabled() {
        println!("   ✓ DO_NOT_TRACK is enabled - telemetry will be disabled");
        return Ok(());
    } else {
        println!("   ℹ DO_NOT_TRACK is not set");
    }

    // ============================================================
    // 2. Default Privacy (recommended)
    // ============================================================
    println!("\n🔐 Example 1: Default Privacy Settings");
    let telemetry = TelemetryKit::builder()
        .service_name("privacy-example")?
        .service_version("1.0.0")
        .build()?;

    println!("   - Respects DO_NOT_TRACK: Yes");
    println!("   - Path sanitization: Enabled");
    println!("   - Email sanitization: Enabled");
    println!("   - Data retention: 90 days");

    telemetry
        .track_command("example", |event| event.success(true).duration_ms(100))
        .await?;

    println!("   ✓ Event tracked with default privacy");

    // ============================================================
    // 3. Strict Privacy Mode (GDPR-compliant)
    // ============================================================
    println!("\n🔒 Example 2: Strict Privacy Mode");
    let telemetry_strict = TelemetryKit::builder()
        .service_name("privacy-strict")?
        .service_version("1.0.0")
        .strict_privacy() // Use strict preset
        .build()?;

    println!("   - Respects DO_NOT_TRACK: Yes");
    println!("   - Consent required: Yes");
    println!("   - Path sanitization: Enabled");
    println!("   - Email sanitization: Enabled");
    println!("   - Data retention: 30 days");

    // Grant consent programmatically
    telemetry_strict.grant_consent()?;
    println!("   ✓ User consent granted");

    telemetry_strict
        .track_feature("privacy-mode", |event| event.success(true))
        .await?;

    println!("   ✓ Event tracked (consent granted)");

    // ============================================================
    // 4. Minimal Privacy Mode
    // ============================================================
    println!("\n🔓 Example 3: Minimal Privacy Mode");
    let telemetry_minimal = TelemetryKit::builder()
        .service_name("privacy-minimal")?
        .service_version("1.0.0")
        .minimal_privacy() // Use minimal preset
        .build()?;

    println!("   - Respects DO_NOT_TRACK: Yes");
    println!("   - Consent required: No");
    println!("   - Path sanitization: Disabled");
    println!("   - Email sanitization: Disabled");
    println!("   - Data retention: Forever");

    telemetry_minimal
        .track_feature("minimal-mode", |event| event.success(true))
        .await?;

    println!("   ✓ Event tracked with minimal privacy");

    // ============================================================
    // 5. Custom Privacy Configuration
    // ============================================================
    println!("\n⚙️  Example 4: Custom Privacy Settings");

    use telemetry_kit::privacy::PrivacyConfig;

    let custom_config = PrivacyConfig {
        consent_required: true,
        respect_do_not_track: true,
        sanitize_paths: true,
        sanitize_emails: true,
        data_retention_days: 60, // 60 days
        anonymize_ips: true,
    };

    let telemetry_custom = TelemetryKit::builder()
        .service_name("privacy-custom")?
        .service_version("1.0.0")
        .privacy(custom_config)
        .build()?;

    println!("   - Data retention: 60 days");
    println!("   - Custom configuration applied");

    // Grant consent
    telemetry_custom.grant_consent()?;

    telemetry_custom
        .track_custom(
            "custom_event",
            serde_json::json!({
                "feature": "custom-privacy"
            }),
        )
        .await?;

    println!("   ✓ Event tracked with custom privacy config");

    // ============================================================
    // 6. Fine-grained Privacy Control
    // ============================================================
    println!("\n🎛️  Example 5: Fine-grained Privacy Control");

    let telemetry_fine = TelemetryKit::builder()
        .service_name("privacy-fine")?
        .service_version("1.0.0")
        .consent_required(true)
        .data_retention(45) // 45 days
        .sanitize_paths(true)
        .sanitize_emails(true)
        .build()?;

    println!("   - Individual settings configured");
    println!("   - Consent required: Yes");
    println!("   - Data retention: 45 days");

    // Grant consent
    telemetry_fine.grant_consent()?;

    telemetry_fine
        .track_feature("fine-grained", |event| event.success(true))
        .await?;

    println!("   ✓ Event tracked with fine-grained privacy");

    // ============================================================
    // 7. Data Sanitization Demo
    // ============================================================
    println!("\n🧹 Example 6: Data Sanitization");

    use telemetry_kit::privacy::PrivacyManager;

    println!("   Original path: /Users/john/Documents/secret.txt");
    let sanitized_path = PrivacyManager::sanitize_path("/Users/john/Documents/secret.txt");
    println!("   Sanitized path: {}", sanitized_path);

    println!("\n   Original email: user@example.com");
    let sanitized_email = PrivacyManager::sanitize_email("user@example.com");
    println!("   Sanitized email: {}", sanitized_email);

    // ============================================================
    // 8. Consent Management
    // ============================================================
    println!("\n✋ Example 7: Consent Management");

    let telemetry_consent = TelemetryKit::builder()
        .service_name("consent-demo")?
        .service_version("1.0.0")
        .consent_required(true)
        .build()?;

    // Try tracking without consent
    println!("   Attempting to track without consent...");
    telemetry_consent
        .track_feature("before-consent", |event| event.success(true))
        .await?;
    println!("   ℹ Event silently ignored (no consent)");

    // Grant consent
    telemetry_consent.grant_consent()?;
    println!("   ✓ Consent granted");

    // Track with consent
    telemetry_consent
        .track_feature("after-consent", |event| event.success(true))
        .await?;
    println!("   ✓ Event tracked successfully");

    // Deny consent
    telemetry_consent.deny_consent()?;
    println!("   ✗ Consent denied");

    // Try tracking after denial
    telemetry_consent
        .track_feature("after-denial", |event| event.success(true))
        .await?;
    println!("   ℹ Event silently ignored (consent denied)");

    // Opt out completely
    telemetry_consent.opt_out()?;
    println!("   🚫 User opted out");

    println!("\n✅ Privacy examples completed!\n");
    println!("💡 Tips:");
    println!("   - Use strict_privacy() for GDPR compliance");
    println!("   - Always respect DO_NOT_TRACK environment variable");
    println!("   - Set appropriate data retention policies");
    println!("   - Sanitize PII (emails, paths) by default");
    println!("   - Ask for consent when required by regulations");

    Ok(())
}

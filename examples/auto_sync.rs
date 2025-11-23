//! Auto-sync example showing background synchronization
//!
//! This example demonstrates:
//! - Enabling auto-sync with credentials
//! - Configurable sync interval
//! - Background sync behavior
//! - Graceful shutdown with final sync
//!
//! Run with:
//! ```
//! cargo run --example auto_sync --features sync
//! ```

use std::time::Duration;
use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Auto-Sync Example ===\n");

    // Get credentials from environment (or use defaults for testing)
    let org_id = std::env::var("TELEMETRY_ORG_ID").unwrap_or_else(|_| "demo-org".to_string());
    let app_id = std::env::var("TELEMETRY_APP_ID").unwrap_or_else(|_| "demo-app".to_string());
    let token = std::env::var("TELEMETRY_TOKEN").unwrap_or_else(|_| "demo-token".to_string());
    let secret = std::env::var("TELEMETRY_SECRET").unwrap_or_else(|_| "demo-secret".to_string());

    println!("Credentials:");
    println!("  Org ID: {}", org_id);
    println!("  App ID: {}", app_id);
    println!("  Token: {}...", &token.chars().take(8).collect::<String>());
    println!();

    // Initialize telemetry with auto-sync
    let telemetry = TelemetryKit::builder()
        .service_name("auto-sync-example")?
        .service_version(env!("CARGO_PKG_VERSION"))
        .with_sync_credentials(&org_id, &app_id, &token, &secret)?
        .auto_sync(true) // Enable auto-sync (default: true)
        .sync_interval(5) // Sync every 5 seconds (default: 60)
        .sync_on_shutdown(true) // Sync before exit (default: true)
        .build()?;

    println!("✓ Telemetry initialized with auto-sync\n");
    println!("Auto-sync configuration:");
    println!("  Interval: 5 seconds");
    println!("  Sync on shutdown: enabled");
    println!();

    // Track multiple events
    for i in 1..=5 {
        println!("Tracking event {}...", i);
        telemetry
            .track_command("process", |event| {
                event
                    .flag(format!("--batch-{}", i))
                    .success(true)
                    .duration_ms(100 * i as u64)
            })
            .await?;

        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    println!("\n✓ All events tracked\n");

    // View stats
    let stats = telemetry.stats().await?;
    println!("Stats:");
    println!("  Total events: {}", stats.total_events);
    println!("  Unsynced: {}", stats.unsynced_events);
    println!("  Synced: {}", stats.synced_events);
    println!();

    // Wait for auto-sync to run
    println!("Waiting 6 seconds for auto-sync to run...");
    tokio::time::sleep(Duration::from_secs(6)).await;

    // Check stats again
    let stats = telemetry.stats().await?;
    println!("\nStats after auto-sync:");
    println!("  Total events: {}", stats.total_events);
    println!("  Unsynced: {}", stats.unsynced_events);
    println!("  Synced: {}", stats.synced_events);
    println!();

    // Graceful shutdown
    println!("Shutting down gracefully (with final sync)...");
    #[cfg(feature = "sync")]
    telemetry.shutdown().await?;

    println!("\n=== Example Complete ===");
    println!("\nNotes:");
    println!("- Auto-sync runs in the background every 5 seconds");
    println!("- Events are synced automatically without manual .sync() calls");
    println!("- Final sync occurs on shutdown (if sync_on_shutdown is enabled)");
    println!("- Events are stored locally until successfully synced");
    println!("- Failed syncs are retried with exponential backoff");

    Ok(())
}

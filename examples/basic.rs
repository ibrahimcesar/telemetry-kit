//! Basic example showing telemetry-kit usage with auto-sync
//!
//! This example demonstrates:
//! - Local-only tracking (no sync)
//! - Manual sync (commented out)
//! - Auto-sync with configurable interval
//!
//! Run with:
//! ```
//! cargo run --example basic
//! ```

use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Basic Telemetry Kit Example ===\n");

    // Initialize telemetry (local-only, no sync)
    let telemetry = TelemetryKit::builder()
        .service_name("basic-example")?
        .service_version(env!("CARGO_PKG_VERSION"))
        .build()?;

    println!("✓ Telemetry initialized (local-only)\n");

    // Track a command
    println!("Tracking command...");
    telemetry
        .track_command("greet", |event| {
            event
                .flag("--name")
                .flag("--enthusiastic")
                .success(true)
                .duration_ms(42)
        })
        .await?;

    println!("✓ Command tracked\n");

    // Track a feature
    println!("Tracking feature...");
    telemetry
        .track_feature("greeting", |event| {
            event.method("enthusiastic").success(true)
        })
        .await?;

    println!("✓ Feature tracked\n");

    // View stats
    let stats = telemetry.stats().await?;
    println!("Stats:");
    println!("  Total events: {}", stats.total_events);
    println!("  Unsynced: {}", stats.unsynced_events);
    println!("  Synced: {}", stats.synced_events);

    println!("\n=== Example Complete ===");
    println!("\nNote: Events are stored locally in ~/.telemetry-kit/basic-example.db");
    println!("To enable sync, set credentials with .with_sync_credentials()");
    println!("\nExample with auto-sync:");
    println!("  TelemetryKit::builder()");
    println!("    .service_name(\"my-app\")?");
    println!("    .with_sync_credentials(org_id, app_id, token, secret)?");
    println!("    .auto_sync(true)              // Enable auto-sync (default)");
    println!("    .sync_interval(60)            // Sync every 60 seconds (default)");
    println!("    .sync_on_shutdown(true)       // Sync before exit (default)");
    println!("    .build()?");

    Ok(())
}

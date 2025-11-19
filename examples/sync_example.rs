//! Example of using telemetry-kit with sync

use telemetry_kit::prelude::*;
use telemetry_kit::sync::SyncConfig;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Telemetry Kit Sync Example ===\n");

    // Configure sync settings
    let sync_config = SyncConfig::builder()
        .org_id("550e8400-e29b-41d4-a716-446655440000")?
        .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")?
        .token("tk_550e8400e29b41d4a716446655440000_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6")
        .secret("9f4b3c2a1e8d7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b6c5d4e3f2a1b0")
        .batch_size(100) // Sync in batches of 100
        .build()?;

    // Initialize telemetry with sync
    let telemetry = TelemetryKit::builder()
        .service_name("example-cli")?
        .service_version("1.0.0")
        .sync(sync_config)
        .build()?;

    println!("Telemetry initialized successfully!\n");

    // Track some events
    println!("Tracking command execution...");
    telemetry
        .track_command("build", |event| {
            event
                .flag("--release")
                .flag("--verbose")
                .duration_ms(1234)
                .success(true)
                .exit_code(0)
        })
        .await?;

    println!("Tracking feature usage...");
    telemetry
        .track_feature("authentication", |event| {
            event.method("oauth").success(true)
        })
        .await?;

    println!("Tracking custom event...");
    telemetry
        .track_custom(
            "deployment_started",
            serde_json::json!({
                "environment": "production",
                "region": "us-east-1"
            }),
        )
        .await?;

    // Check statistics
    let stats = telemetry.stats().await?;
    println!("\n=== Event Statistics ===");
    println!("Total events: {}", stats.total_events);
    println!("Unsynced events: {}", stats.unsynced_events);
    println!("Synced events: {}", stats.synced_events);

    // Manually trigger sync
    println!("\n=== Syncing Events ===");
    match telemetry.sync().await {
        Ok(()) => println!("✓ Events synced successfully!"),
        Err(e) => println!("✗ Sync failed: {}", e),
    }

    // Check statistics after sync
    let stats = telemetry.stats().await?;
    println!("\n=== Stats After Sync ===");
    println!("Total events: {}", stats.total_events);
    println!("Unsynced events: {}", stats.unsynced_events);
    println!("Synced events: {}", stats.synced_events);

    println!("\n=== Example Complete ===");

    Ok(())
}

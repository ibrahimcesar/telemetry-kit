//! End-to-end test demonstrating SDK syncing to local server
//!
//! This example shows how to use the telemetry SDK to track events
//! and sync them to a local server instance.
//!
//! ## Prerequisites
//!
//! 1. Start the server with Docker Compose:
//!    ```bash
//!    cd server
//!    docker compose up -d
//!    ```
//!
//! 2. The server will be running at http://localhost:3000
//!
//! 3. A test token is pre-seeded in the database:
//!    - Org ID: 550e8400-e29b-41d4-a716-446655440000
//!    - App ID: 7c9e6679-7425-40de-944b-e07fc1f90ae7
//!    - Token: tk_550e8400e29b41d4a716446655440000_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6
//!    - Secret: 9f4b3c2a1e8d7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b6c5d4e3f2a1b0
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example e2e_sync_test
//! ```

use telemetry_kit::prelude::*;
use telemetry_kit::sync::SyncConfig;

#[tokio::main]
async fn main() -> telemetry_kit::Result<()> {
    println!("🚀 Starting end-to-end sync test...\n");

    // Configure sync with local server
    let sync_config = SyncConfig::builder()
        .endpoint("http://localhost:3000")
        .org_id("550e8400-e29b-41d4-a716-446655440000")?
        .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")?
        .token("tk_550e8400e29b41d4a716446655440000_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6")
        .secret("9f4b3c2a1e8d7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b6c5d4e3f2a1b0")
        .build()?;

    println!("✅ Configured sync to: http://localhost:3000");
    println!("   Org ID: 550e8400-e29b-41d4-a716-446655440000");
    println!("   App ID: 7c9e6679-7425-40de-944b-e07fc1f90ae7\n");

    // Initialize telemetry with sync enabled
    let telemetry = TelemetryKit::builder()
        .service_name("e2e-test")?
        .service_version("1.0.0")
        .sync(sync_config)
        .build()?;

    println!("📊 Tracking test events...\n");

    // Track various events
    telemetry
        .track_command("init", |e| {
            e.success(true)
                .subcommand("e2e-test")
        })
        .await?;
    println!("   ✓ Tracked 'init' command");

    telemetry
        .track_feature("sync", |e| {
            e.success(true)
                .method("hmac-sha256")
                .data("batch_size", serde_json::json!(3))
        })
        .await?;
    println!("   ✓ Tracked 'sync' feature");

    telemetry
        .track_command("test", |e| {
            e.success(true)
                .duration_ms(150)
        })
        .await?;
    println!("   ✓ Tracked 'test' command\n");

    // Get stats before sync
    let stats_before = telemetry.stats().await?;
    println!("📈 Statistics before sync:");
    println!("   Total events: {}", stats_before.total_events);
    println!("   Unsynced events: {}", stats_before.unsynced_events);
    println!("   Successfully synced: {}\n", stats_before.synced_events);

    // Sync events to server
    println!("🔄 Syncing events to server...");
    match telemetry.sync().await {
        Ok(()) => {
            println!("✅ Sync completed successfully!\n");
        }
        Err(e) => {
            eprintln!("❌ Sync failed: {}\n", e);
            eprintln!("💡 Make sure the server is running:");
            eprintln!("   cd server");
            eprintln!("   docker compose up -d\n");
            return Err(e);
        }
    }

    // Get stats after sync
    let stats_after = telemetry.stats().await?;
    println!("📈 Statistics after sync:");
    println!("   Total events: {}", stats_after.total_events);
    println!("   Unsynced events: {}", stats_after.unsynced_events);
    println!("   Successfully synced: {}\n", stats_after.synced_events);

    // Verify sync worked
    if stats_after.synced_events > stats_before.synced_events {
        let synced = stats_after.synced_events - stats_before.synced_events;
        println!("🎉 Successfully synced {} events to the server!", synced);
    }

    println!("\n✨ End-to-end test complete!");
    println!("\n💡 You can now query the events in the PostgreSQL database:");
    println!("   docker compose -f server/docker-compose.yml exec postgres \\");
    println!("     psql -U telemetry telemetry_kit -c 'SELECT event_id, event_type, service_name, timestamp FROM events ORDER BY received_at DESC LIMIT 10;'");

    Ok(())
}

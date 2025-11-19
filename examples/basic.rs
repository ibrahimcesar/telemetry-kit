//! Basic example showing telemetry-kit usage
//!
//! Run with:
//! ```
//! cargo run --example basic
//! ```

use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Basic Telemetry Kit Example ===\n");

    // Initialize telemetry
    let telemetry = TelemetryKit::builder()
        .service_name("basic-example")?
        .service_version(env!("CARGO_PKG_VERSION"))
        .build()?;

    println!("✓ Telemetry initialized\n");

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

    // View stats
    let stats = telemetry.stats().await?;
    println!("Stats:");
    println!("  Total events: {}", stats.total_events);
    println!("  Unsynced: {}", stats.unsynced_events);

    println!("\n=== Example Complete ===");

    Ok(())
}

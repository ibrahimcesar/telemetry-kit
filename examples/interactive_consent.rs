//! Interactive Consent Example
//!
//! This example demonstrates the interactive consent prompt feature.
//! On the first run, it will show an interactive consent dialog.
//! On subsequent runs, it will use the stored consent preference.
//!
//! Run with:
//! ```
//! cargo run --example interactive_consent --features cli
//! ```
//!
//! To reset consent and see the prompt again:
//! ```
//! rm ~/.telemetry-kit/interactive-consent-demo-consent.json
//! cargo run --example interactive_consent --features cli
//! ```

use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Interactive Consent Example ===\n");

    // Method 1: Full consent prompt (detailed)
    println!("📋 Example 1: Full Consent Prompt");
    println!("Building telemetry with interactive consent...\n");

    let telemetry = TelemetryKit::builder()
        .service_name("interactive-consent-demo")?
        .service_version("1.0.0")
        .prompt_for_consent()? // Shows interactive prompt on first run
        .build()?;

    println!("✓ Telemetry initialized with consent\n");

    // Track some events
    telemetry
        .track_command("demo", |event| {
            event.flag("--interactive").success(true).duration_ms(42)
        })
        .await?;

    println!("✓ Event tracked\n");

    // Show stats
    let stats = telemetry.stats().await?;
    println!("Stats:");
    println!("  Total events: {}", stats.total_events);
    println!("  Unsynced: {}", stats.unsynced_events);
    println!("  Synced: {}", stats.synced_events);

    println!("\n=== Example Complete ===\n");
    println!("💡 What happened:");
    println!("  1. On first run, you saw an interactive consent prompt");
    println!("  2. Your choice was saved to ~/.telemetry-kit/interactive-consent-demo-consent.json");
    println!("  3. On next run, your saved preference will be used");
    println!();
    println!("To manage consent later:");
    println!("  telemetry-kit consent status   # Check current status");
    println!("  telemetry-kit consent grant    # Enable telemetry");
    println!("  telemetry-kit consent deny     # Disable telemetry");
    println!();
    println!("To reset and see the prompt again:");
    println!("  rm ~/.telemetry-kit/interactive-consent-demo-consent.json");

    Ok(())
}

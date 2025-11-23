//! Minimal Consent Prompt Example
//!
//! This example demonstrates the minimal (one-liner) consent prompt.
//! Use this when you want a less verbose consent experience.
//!
//! Run with:
//! ```
//! cargo run --example minimal_consent --features cli
//! ```

use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Minimal Consent Example ===\n");

    // Minimal consent prompt (one-liner)
    let telemetry = TelemetryKit::builder()
        .service_name("minimal-consent-demo")?
        .service_version("1.0.0")
        .prompt_minimal()? // Shows minimal one-line prompt
        .build()?;

    println!("\n✓ Telemetry initialized\n");

    // Track an event
    telemetry
        .track_feature("minimal-prompt", |event| event.success(true))
        .await?;

    println!("✓ Event tracked\n");

    println!("=== Example Complete ===\n");
    println!("The minimal prompt is perfect for:");
    println!("  • CLI tools with minimal output");
    println!("  • Quick interactive setups");
    println!("  • Applications where verbosity is unwanted");

    Ok(())
}

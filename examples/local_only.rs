//! Example of using telemetry-kit for local-only event tracking (no sync)

use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Telemetry Kit Local-Only Example ===\n");

    // Initialize telemetry without sync
    // Events will be buffered locally in SQLite
    let telemetry = TelemetryKit::builder()
        .service_name("my-awesome-cli")?
        .service_version("2.3.1")
        .build()?;

    println!("Telemetry initialized (local-only mode)\n");

    // Track a successful command
    println!("1. Tracking successful build command...");
    telemetry
        .track_command("build", |event| {
            event
                .flag("--release")
                .flag("--verbose")
                .duration_ms(5420)
                .success(true)
                .exit_code(0)
        })
        .await?;

    // Track a failed command
    println!("2. Tracking failed deploy command...");
    telemetry
        .track_command("deploy", |event| {
            event
                .subcommand("production")
                .flag("--force")
                .duration_ms(823)
                .success(false)
                .exit_code(1)
        })
        .await?;

    // Track feature usage
    println!("3. Tracking feature usage (GitHub integration)...");
    telemetry
        .track_feature("github_integration", |event| {
            event.method("clone_repo").success(true)
        })
        .await?;

    // Track another feature
    println!("4. Tracking feature usage (Docker build)...");
    telemetry
        .track_feature("docker_build", |event| {
            event
                .method("build_image")
                .success(true)
                .data("platform", serde_json::json!("linux/amd64"))
        })
        .await?;

    // Track custom events
    println!("5. Tracking custom configuration event...");
    telemetry
        .track_custom(
            "configuration_loaded",
            serde_json::json!({
                "config_file": ".config.toml",
                "plugins_enabled": 5
            }),
        )
        .await?;

    // Display statistics
    let stats = telemetry.stats().await?;
    println!("\n=== Event Statistics ===");
    println!("Total events tracked: {}", stats.total_events);
    println!("Unsynced events: {}", stats.unsynced_events);
    println!("Synced events: {}", stats.synced_events);

    // Cleanup old events
    println!("\n=== Cleanup ===");
    let cleaned = telemetry.cleanup().await?;
    println!("Cleaned up {} old synced events", cleaned);

    println!("\n=== Example Complete ===");
    println!("Events are stored locally in ~/.telemetry-kit/my-awesome-cli.db");

    Ok(())
}

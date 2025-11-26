# Sync Setup Guide

This guide shows you how to set up telemetry-kit to sync events with telemetry-kit.dev.

## Quick Start

```rust
use telemetry_kit::prelude::*;
use telemetry_kit::sync::SyncConfig;

#[tokio::main]
async fn main() -> telemetry_kit::Result<()> {
    // 1. Configure sync
    let sync_config = SyncConfig::builder()
        .org_id("YOUR_ORG_ID")?
        .app_id("YOUR_APP_ID")?
        .token("YOUR_API_TOKEN")
        .secret("YOUR_API_SECRET")
        .build()?;

    // 2. Initialize telemetry with sync
    let telemetry = TelemetryKit::builder()
        .service_name("my-app")?
        .service_version("1.0.0")
        .sync(sync_config)
        .build()?;

    // 3. Track events
    telemetry.track_command("build", |event| {
        event
            .flag("--release")
            .duration_ms(1234)
            .success(true)
    }).await?;

    // 4. Manually sync events
    telemetry.sync().await?;

    Ok(())
}
```

## Configuration Options

### SyncConfig

```rust
let sync_config = SyncConfig::builder()
    // Required: Your organization ID from telemetry-kit.dev dashboard
    .org_id("550e8400-e29b-41d4-a716-446655440000")?

    // Required: Your application ID from telemetry-kit.dev dashboard
    .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")?

    // Required: API token for authentication
    .token("tk_550e8400e29b41d4a716446655440000_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6")

    // Required: API secret for HMAC signing
    .secret("9f4b3c2a1e8d7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b6c5d4e3f2a1b0")

    // Optional: Batch size (default: 100, max: 1000)
    .batch_size(100)

    // Optional: Maximum retries (default: 5)
    .max_retries(5)

    // Optional: Enable/disable DNT check (default: true)
    .respect_dnt(true)

    .build()?;
```

### Using Staging Environment

For testing, you can use the staging endpoint:

```rust
let sync_config = SyncConfig::builder()
    .use_staging()  // Uses https://staging.telemetry-kit.dev
    .org_id("...")?
    .app_id("...")?
    .token("...")
    .secret("...")
    .build()?;
```

## Getting Credentials

1. **Sign up** at [telemetry-kit.dev](https://telemetry-kit.dev)
2. **Create an organization** in the dashboard
3. **Create an application** for your project
4. **Generate API credentials** (token + secret)
5. **Copy the credentials** to your configuration

## Syncing Strategies

### Manual Sync

Call `.sync()` manually when you want to upload events:

```rust
// Track events
telemetry.track_command("deploy", |e| e.success(true)).await?;
telemetry.track_command("test", |e| e.success(true)).await?;

// Sync all pending events
telemetry.sync().await?;
```

### Periodic Sync

Set up a background task to sync periodically:

```rust
let telemetry_clone = telemetry.clone();
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(3600));
    loop {
        interval.tick().await;
        if let Err(e) = telemetry_clone.sync().await {
            eprintln!("Sync failed: {}", e);
        }
    }
});
```

### Threshold-Based Sync

Sync when you reach a certain number of events:

```rust
let stats = telemetry.stats().await?;
if stats.unsynced_events >= 100 {
    telemetry.sync().await?;
}
```

## Event Storage

Events are buffered locally in SQLite before syncing:

- **Location**: `~/.telemetry-kit/<service-name>.db`
- **Retention**: Synced events are kept for 7 days, then automatically cleaned up
- **Offline Support**: Events are queued even when offline and synced when connection is restored

### Custom Database Path

```rust
let telemetry = TelemetryKit::builder()
    .service_name("my-app")?
    .db_path("/custom/path/telemetry.db")
    .build()?;
```

## Privacy & DNT

telemetry-kit respects the `DNT` (Do Not Track) environment variable:

```bash
# Disable telemetry
export DNT=1

# Enable telemetry
export DNT=0
# or unset DNT
```

When `DNT=1`, sync requests will be skipped automatically.

## Error Handling

```rust
match telemetry.sync().await {
    Ok(()) => println!("Sync successful"),
    Err(TelemetryError::RateLimitExceeded { retry_after }) => {
        println!("Rate limited, retry after {} seconds", retry_after);
    }
    Err(TelemetryError::ServerError { status, message }) => {
        eprintln!("Server error {}: {}", status, message);
    }
    Err(e) => eprintln!("Sync error: {}", e),
}
```

## Best Practices

1. **Sync Sparingly**: Don't sync after every event. Batch them and sync periodically or at application shutdown.

2. **Handle Errors Gracefully**: Don't let sync errors crash your application. Log them and continue.

3. **Use Meaningful Service Names**: Service names should be lowercase alphanumeric with dashes/underscores only.

4. **Track Selectively**: Only track events that provide value for your analytics.

5. **Cleanup Regularly**: Call `.cleanup()` periodically to remove old synced events.

## Complete Example

See [`examples/sync_example.rs`](examples/sync_example.rs) for a full working example.

## Troubleshooting

### Events not syncing

1. Check your credentials are correct
2. Verify you're calling `.sync()`
3. Check for errors in sync responses
4. Ensure DNT is not set to 1

### Rate limit errors

- Reduce sync frequency
- Increase batch size
- Upgrade your telemetry-kit.dev plan

### Authentication errors

- Verify token and secret are correct
- Check org_id and app_id match your dashboard
- Ensure HMAC signature is being calculated correctly

## API Reference

See the [full API documentation](https://docs.telemetry-kit.dev/api) for details on all configuration options and methods.

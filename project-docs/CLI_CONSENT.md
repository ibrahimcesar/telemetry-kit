# CLI Consent Management

The `telemetry-kit` CLI now includes comprehensive privacy consent management commands.

## Commands

### Grant Consent

Grant consent for telemetry tracking:

```bash
telemetry-kit consent grant
```

Output:
```
✓ Consent granted

Telemetry tracking is now enabled
Service: my-app

You can revoke consent at any time with:
  telemetry-kit consent deny
```

### Deny Consent

Deny consent for telemetry tracking:

```bash
telemetry-kit consent deny
```

Output:
```
✗ Consent denied

Telemetry tracking is now disabled
Service: my-app

You can grant consent at any time with:
  telemetry-kit consent grant
```

### Opt Out

Opt out of all telemetry (equivalent to `DO_NOT_TRACK=1`):

```bash
telemetry-kit consent opt-out
```

Output:
```
🚫 Opted out of telemetry

You have opted out of all telemetry tracking.
Service: my-app

This is equivalent to setting DO_NOT_TRACK=1

You can opt back in with:
  telemetry-kit consent grant
```

### Check Status

View current consent status:

```bash
telemetry-kit consent status
```

Output:
```
Privacy Consent Status

Service: my-app

Consent Status: Granted ✓
Last Updated: 2025-01-15 10:30:00 UTC

Current Behavior: Telemetry is ENABLED
  Events will be tracked and synced

Commands:
  telemetry-kit consent grant - Grant consent
  telemetry-kit consent deny - Deny consent
  telemetry-kit consent opt-out - Opt out completely
```

### Interactive Prompt

Interactively prompt for consent with full privacy information:

```bash
telemetry-kit consent prompt
```

Output:
```
Privacy Consent

Service: my-app

telemetry-kit collects anonymous usage statistics to help
improve the tool. We respect your privacy:

  • ✓ anonymous machine ID (no personal data)
  • ✓ command usage and success rates
  • ✓ errors and performance metrics
  • ✓ always respects DO_NOT_TRACK

We do NOT collect:
  • ✗ usernames or email addresses
  • ✗ file paths (sanitized to ~)
  • ✗ any personally identifiable information

? What would you like to do? ›
❯ Grant consent
  Deny consent
  Opt out (DO_NOT_TRACK)
```

## Service Name

By default, commands use the current directory name as the service name. You can override this:

```bash
telemetry-kit consent status --service my-custom-app
```

## DO_NOT_TRACK Detection

The CLI automatically detects the `DO_NOT_TRACK` environment variable:

```bash
export DO_NOT_TRACK=1
telemetry-kit consent status
```

Output will show:
```
⚠ DO_NOT_TRACK environment variable is set
  Telemetry is disabled regardless of consent
```

## Consent Storage

Consent is stored persistently in:
```
~/.telemetry-kit/{service-name}-consent.json
```

Format:
```json
{
  "status": "Granted",
  "timestamp": "2025-01-15T10:30:00Z",
  "service_name": "my-app"
}
```

## Integration with Code

The CLI consent commands integrate seamlessly with the SDK:

```rust
use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let telemetry = TelemetryKit::builder()
        .service_name("my-app")?
        .consent_required(true)  // Respect CLI consent
        .build()?;

    // This will be ignored if user ran: telemetry-kit consent deny
    telemetry.track_command("test", |e| e.success(true)).await?;

    Ok(())
}
```

## Best Practices

### 1. First-Run Consent

Prompt for consent on first run:

```rust
use telemetry_kit::privacy::PrivacyManager;

let manager = PrivacyManager::new(PrivacyConfig::default(), "my-app")?;
let consent = manager.load_consent()?;

if consent.status == ConsentStatus::Unknown {
    // Show consent prompt
    println!("Run: telemetry-kit consent prompt");
}
```

### 2. Respect User Choice

Always check consent before tracking:

```rust
let telemetry = TelemetryKit::builder()
    .service_name("my-app")?
    .consent_required(true)
    .build()?;

// Events automatically ignored if consent denied
```

### 3. Provide Clear Instructions

Include consent instructions in your README:

```markdown
## Telemetry

This tool collects anonymous usage statistics. You can manage this with:

```bash
# Grant consent
telemetry-kit consent grant

# Deny consent
telemetry-kit consent deny

# Check status
telemetry-kit consent status
```

Or set the `DO_NOT_TRACK` environment variable:
```bash
export DO_NOT_TRACK=1
```
```

## Examples

### Complete Flow

```bash
# 1. Check initial status
telemetry-kit consent status
# Output: Unknown (not set)

# 2. Run interactive prompt
telemetry-kit consent prompt
# User selects: Grant consent

# 3. Verify consent granted
telemetry-kit consent status
# Output: Granted ✓

# 4. Later, user wants to opt out
telemetry-kit consent opt-out
# Output: 🚫 Opted out

# 5. Check final status
telemetry-kit consent status
# Output: Opted Out 🚫
```

### Multiple Services

Manage consent for multiple services:

```bash
# Service A
telemetry-kit consent grant --service service-a

# Service B
telemetry-kit consent deny --service service-b

# Check both
telemetry-kit consent status --service service-a  # Granted
telemetry-kit consent status --service service-b  # Denied
```

### CI/CD Integration

Disable telemetry in CI/CD:

```yaml
# GitHub Actions
- name: Run tests
  env:
    DO_NOT_TRACK: 1
  run: cargo test
```

Or explicitly deny consent:

```bash
telemetry-kit consent deny --service my-app --yes
```

## Troubleshooting

### Consent Not Working

If consent seems to be ignored:

1. Check DO_NOT_TRACK:
   ```bash
   echo $DO_NOT_TRACK
   ```

2. Verify consent file exists:
   ```bash
   cat ~/.telemetry-kit/my-app-consent.json
   ```

3. Check SDK configuration:
   ```rust
   let telemetry = TelemetryKit::builder()
       .service_name("my-app")?
       .consent_required(true)  // Must be true!
       .build()?;
   ```

### Reset Consent

To reset consent:

```bash
rm ~/.telemetry-kit/my-app-consent.json
telemetry-kit consent status  # Shows Unknown
```

## See Also

- [Privacy Controls Documentation](docs/content/docs/privacy.mdx)
- [Privacy Implementation](PRIVACY_IMPLEMENTATION.md)
- [Privacy Example](examples/privacy.rs)
- [CLI Documentation](CLI.md)

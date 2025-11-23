# telemetry-kit CLI Documentation

The `telemetry-kit` command-line tool provides convenient commands for managing telemetry configuration and operations.

## Installation

Build the CLI with the `cli` feature:

```bash
cargo install telemetry-kit --features cli
```

Or build from source:

```bash
git clone https://github.com/ibrahimcesar/telemetry-kit
cd telemetry-kit
cargo build --release --features cli
```

The binary will be available at `target/release/telemetry-kit`.

## Commands

### `telemetry-kit init`

Interactive setup wizard for configuring telemetry in your project.

**Usage:**
```bash
telemetry-kit init [OPTIONS]
```

**Options:**
- `-y, --yes` - Skip interactive prompts and use defaults
- `-n, --service-name <NAME>` - Specify service name
- `-s, --service <NAME>` - Override service name globally

**Examples:**
```bash
# Interactive setup
telemetry-kit init

# Quick setup with defaults
telemetry-kit init --yes

# Setup with specific service name
telemetry-kit init --service-name my-app
```

**What it does:**
1. Prompts for service name (defaults to current directory)
2. Asks if you want to configure sync to telemetry-kit.dev
3. If yes, collects credentials:
   - Organization ID
   - Application ID
   - Auth Token
   - HMAC Secret
4. Tests the credentials (if sync configured)
5. Provides code snippet for your application

---

### `telemetry-kit test`

Test sync credentials and verify connectivity to telemetry-kit.dev.

**Usage:**
```bash
telemetry-kit test [OPTIONS]
```

**Options:**
- `-o, --org-id <ID>` - Organization ID
- `-a, --app-id <ID>` - Application ID
- `-t, --token <TOKEN>` - Authentication token
- `-s, --secret <SECRET>` - HMAC secret

**Examples:**
```bash
# Interactive (prompts for all credentials)
telemetry-kit test

# Provide credentials via flags
telemetry-kit test \
  --org-id demo-org \
  --app-id demo-app \
  --token demo-token \
  --secret demo-secret
```

**What it does:**
1. Collects credentials (via prompts or flags)
2. Creates a sync client with the credentials
3. Validates that the configuration is valid
4. Shows success/failure with colored output

---

###telemetry-kit stats`

View statistics about buffered events for a service.

**Usage:**
```bash
telemetry-kit stats [OPTIONS]
```

**Options:**
- `-d, --detailed` - Show detailed breakdown including file size
- `-s, --service <NAME>` - Service name (defaults to current directory)

**Examples:**
```bash
# View stats for current directory service
telemetry-kit stats

# View detailed stats
telemetry-kit stats --detailed

# View stats for specific service
telemetry-kit stats --service my-app
```

**What it shows:**
- Service name and database path
- Total events count
- Synced events count
- Unsynced events count
- Sync percentage
- Database file size (with `--detailed`)
- Helpful hints if unsynced events exist

**Example output:**
```
📊 Event Statistics

Service: my-app
Database: /Users/you/.telemetry-kit/my-app.db

Events:
  Total:      42
  Synced:     30
  Unsynced:   12

Sync rate: 71%

💡 You have 12 unsynced events
   Run telemetry-kit sync to sync now
```

---

### `telemetry-kit sync`

Manually trigger synchronization of events.

**Usage:**
```bash
telemetry-kit sync [OPTIONS]
```

**Options:**
- `-f, --force` - Force sync even if auto-sync is enabled
- `-s, --service <NAME>` - Service name

**Examples:**
```bash
# Sync current service
telemetry-kit sync

# Force sync
telemetry-kit sync --force
```

**Note:** Currently a placeholder. Full implementation requires stored credentials.
The command provides instructions for using the SDK's `.sync()` method or enabling auto-sync.

---

### `telemetry-kit validate`

Validate telemetry configuration for a service.

**Usage:**
```bash
telemetry-kit validate [OPTIONS]
```

**Options:**
- `-c, --config <PATH>` - Path to configuration file (future use)
- `-s, --service <NAME>` - Service name

**Examples:**
```bash
# Validate current service
telemetry-kit validate

# Validate specific service
telemetry-kit validate --service my-app
```

**What it checks:**
- Database file exists
- Database can be opened
- TelemetryKit can be initialized
- Configuration is valid

---

### `telemetry-kit clean`

Clear local event database for a service.

**Usage:**
```bash
telemetry-kit clean [OPTIONS]
```

**Options:**
- `-y, --yes` - Skip confirmation prompt
- `--all` - Also remove configuration files (future use)
- `-s, --service <NAME>` - Service name

**Examples:**
```bash
# Clean with confirmation
telemetry-kit clean

# Clean without confirmation
telemetry-kit clean --yes

# Clean everything including config
telemetry-kit clean --all
```

**What it does:**
1. Shows current event statistics
2. Warns if unsynced events will be lost
3. Prompts for confirmation (unless `-y`)
4. Deletes the SQLite database file
5. Optionally removes config files (with `--all`)

**Warning:** This operation is destructive. Unsynced events will be permanently lost.

---

## Global Options

Available on all commands:

- `-s, --service <NAME>` - Service name to operate on (defaults to current directory name)
- `-h, --help` - Show help for the command
- `-V, --version` - Show version information

## Examples

### Typical Workflow

```bash
# 1. Initialize telemetry in your project
cd my-app
telemetry-kit init

# 2. Add the generated code to your app
# (copy the code snippet from init output)

# 3. Run your app and generate some events
cargo run

# 4. View statistics
telemetry-kit stats

# 5. View detailed stats
telemetry-kit stats --detailed

# 6. Validate configuration
telemetry-kit validate

# 7. Clean up (optional)
telemetry-kit clean
```

### Testing Credentials

```bash
# Test credentials interactively
telemetry-kit test

# Test with environment variables
telemetry-kit test \
  --org-id "$TELEMETRY_ORG_ID" \
  --app-id "$TELEMETRY_APP_ID" \
  --token "$TELEMETRY_TOKEN" \
  --secret "$TELEMETRY_SECRET"
```

### Managing Multiple Services

```bash
# View stats for different services
telemetry-kit stats --service frontend
telemetry-kit stats --service backend
telemetry-kit stats --service worker

# Clean specific service
telemetry-kit clean --service old-service --yes
```

## Color Output

The CLI uses colors to improve readability:

- **Cyan** - Command names, values, highlights
- **Green** - Success messages, valid states
- **Yellow** - Warnings, info messages
- **Red** - Errors, failures
- **Dimmed** - Less important information
- **Bold** - Headers, important messages

Colors can be disabled by setting `NO_COLOR=1` environment variable.

## Database Location

Event databases are stored in:
```
~/.telemetry-kit/<service-name>.db
```

Each service has its own SQLite database file.

## Configuration

Configuration files are planned for future releases. Currently, all configuration is done via:
1. CLI flags
2. Interactive prompts
3. SDK builder API in your code

## Troubleshooting

### "No telemetry data found"

```bash
# Make sure you've initialized telemetry in your app
telemetry-kit init

# Check the database path
ls ~/.telemetry-kit/
```

### "Failed to initialize"

```bash
# Validate your configuration
telemetry-kit validate

# Check for database corruption
rm ~/.telemetry-kit/my-app.db
# Re-run your app
```

### "Credentials test failed"

```bash
# Double-check your credentials
telemetry-kit test

# Verify network connectivity
ping telemetry-kit.dev
```

## Future Enhancements

Planned features for future releases:

- **Configuration files** - Store credentials securely in config files
- **Full sync implementation** - CLI-based sync with stored credentials
- **Event filtering** - View and filter specific events
- **Export** - Export events to JSON/CSV
- **Import** - Import events from backup
- **Batch operations** - Operate on multiple services at once
- **Plugin system** - Extend CLI with custom commands

## See Also

- [README.md](README.md) - Main project documentation
- [PRODUCTION_PLAN.md](PRODUCTION_PLAN.md) - Development roadmap
- [examples/](examples/) - Usage examples
- SDK API documentation: `cargo doc --open`

---

**Version:** 0.2.0-alpha.1
**License:** MIT OR Apache-2.0
**Repository:** https://github.com/ibrahimcesar/telemetry-kit

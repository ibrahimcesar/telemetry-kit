# telemetry-kit VS Code Extension

Intelligent telemetry suggestions and code snippets for Rust open source projects using [telemetry-kit](https://github.com/ibrahimcesar/telemetry-kit).

**Perfect for open source maintainers** who want to understand how their CLI tools, libraries, and applications are used in the wild - without compromising user privacy.

## Features

### 🔍 Smart Analysis for OSS Projects

Automatically detects instrumentation opportunities in your Rust code to help you understand real-world usage:

- **Main Functions** - Track application lifecycle and startup
- **CLI Commands** - Monitor which commands users actually use
- **Async Functions** - Monitor async operations and performance
- **HTTP Handlers** - Track API endpoint usage in web frameworks
- **Error Paths** - Identify where users encounter issues
- **Library Functions** - Track feature adoption in your crates

![Analysis Demo](images/analysis-demo.gif)

### ⚡ Quick Fixes

One-click code actions to add telemetry tracking:

![Quick Fix Demo](images/quickfix-demo.gif)

### 📝 Rich Snippets Library

17+ code snippets for common telemetry patterns:

- `tk-init` - Initialize telemetry-kit
- `tk-command` - Track command execution
- `tk-feature` - Track feature usage
- `tk-error` - Track errors with context
- `tk-timing` - Track operations with timing
- And many more!

![Snippets Demo](images/snippets-demo.gif)

### 🎯 Inline Hints

Visual indicators show where telemetry could be added:

![Inline Hints Demo](images/hints-demo.png)

## Requirements

- **Rust 1.70+**
- **telemetry-kit CLI** - Install with: `cargo install telemetry-kit-cli`

## Installation

1. **From VS Code Marketplace:**
   - Search for "telemetry-kit" in the Extensions view
   - Click Install

2. **From VSIX:**
   ```bash
   code --install-extension telemetry-kit-0.1.0.vsix
   ```

## Quick Start

1. Open a Rust project with `Cargo.toml`
2. Run command: **"telemetry-kit: Analyze Project"** (`Cmd+Shift+P` / `Ctrl+Shift+P`)
3. View suggestions in the Problems panel
4. Click on suggestions for quick fixes
5. Use snippets by typing `tk-` and selecting from autocomplete

## Commands

| Command | Description | Shortcut |
|---------|-------------|----------|
| `telemetry-kit: Analyze Project` | Scan entire project for telemetry opportunities | - |
| `telemetry-kit: Add Telemetry Here` | Add telemetry at cursor position | - |
| `telemetry-kit: Initialize Project` | Run interactive telemetry setup | - |

## Snippets Reference

### Initialization

- `tk-init` - Basic initialization with privacy checks
- `tk-init-consent` - Initialize with interactive consent prompt
- `tk-init-sync` - Initialize with auto-sync enabled
- `tk-main` - Complete main function template

### Tracking

- `tk-command` - Track command execution
- `tk-feature` - Track feature usage
- `tk-event` - Track custom event
- `tk-error` - Track error with context
- `tk-timing` - Track operation with duration
- `tk-result` - Track operation result

### Privacy

- `tk-dnt` - DO_NOT_TRACK check
- `tk-privacy-strict` - Enable strict privacy mode
- `tk-privacy-custom` - Custom privacy configuration

### Integration

- `tk-axum` - Axum web handler with telemetry
- `tk-cli-subcommand` - CLI subcommand tracking
- `tk-library` - Library crate with optional telemetry
- `tk-instrument` - Macro-based instrumentation

### Utilities

- `tk-shutdown` - Graceful shutdown

## Configuration

Access settings via: **Preferences > Settings > telemetry-kit**

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `telemetry-kit.autoSuggest` | boolean | `true` | Auto-analyze files on save |
| `telemetry-kit.showInlineHints` | boolean | `true` | Show inline instrumentation hints |
| `telemetry-kit.diagnosticSeverity` | enum | `"Hint"` | Severity level for suggestions |
| `telemetry-kit.cliPath` | string | `"telemetry-kit"` | Path to CLI executable |

### Example Configuration

```json
{
  "telemetry-kit.autoSuggest": true,
  "telemetry-kit.showInlineHints": true,
  "telemetry-kit.diagnosticSeverity": "Information",
  "telemetry-kit.cliPath": "/usr/local/bin/telemetry-kit"
}
```

## Usage Examples

### Example 1: Analyzing a Project

1. Open Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`)
2. Type "telemetry-kit: Analyze Project"
3. Review suggestions in the Problems panel
4. Click on a suggestion to navigate to the code
5. Use the light bulb (💡) for quick fixes

### Example 2: Using Snippets

Type `tk-init` in a Rust file and press `Tab`:

```rust
use telemetry_kit::prelude::*;

// Check DO_NOT_TRACK
if TelemetryKit::is_do_not_track_enabled() {
    return Ok(());
}

let telemetry = TelemetryKit::builder()
    .service_name("my-app")?
    .service_version(env!("CARGO_PKG_VERSION"))
    // .strict_privacy()  // For GDPR compliance
    .build()?;
```

### Example 3: Quick Telemetry Addition

1. Place cursor on a function
2. Right-click → "telemetry-kit: Add Telemetry Here"
3. Extension inserts appropriate tracking code

## Troubleshooting

### CLI Not Found

**Error:** `telemetry-kit CLI not found`

**Solution:**
```bash
cargo install telemetry-kit-cli
```

Or set custom path in settings:
```json
{
  "telemetry-kit.cliPath": "/path/to/telemetry-kit"
}
```

### No Suggestions Appearing

1. Ensure you have a `Cargo.toml` in your workspace
2. Check that files are saved (auto-analyze runs on save)
3. Manually run "telemetry-kit: Analyze Project"
4. Check Output panel for errors: View → Output → telemetry-kit

### Analysis Takes Too Long

For large projects, disable auto-analyze:

```json
{
  "telemetry-kit.autoSuggest": false
}
```

Run analysis manually when needed.

## Best Practices

### 1. Start with Analysis

Run project analysis before adding telemetry:
```
Cmd+Shift+P → telemetry-kit: Analyze Project
```

### 2. Use Snippets for Common Patterns

Don't write telemetry code from scratch - use snippets:
- `tk-main` for entry points
- `tk-command` for CLI commands
- `tk-timing` for performance-critical code

### 3. Follow Priority Guidance

Focus on **High priority** suggestions first:
- Main functions
- Error handlers
- Critical user flows

### 4. Privacy First

Always use privacy snippets:
- `tk-dnt` at app start
- `tk-privacy-strict` for EU apps
- `tk-init-consent` for interactive consent

## Contributing

Found a bug or want a feature?

- [GitHub Issues](https://github.com/ibrahimcesar/telemetry-kit/issues)
- [Documentation](https://github.com/ibrahimcesar/telemetry-kit#readme)

## Privacy

This extension:
- ✅ Runs locally (no data sent to external services)
- ✅ Only executes the telemetry-kit CLI you install
- ✅ Does not collect any telemetry about your code

## License

MIT License - see [LICENSE](LICENSE) for details

## Links

- [telemetry-kit GitHub](https://github.com/ibrahimcesar/telemetry-kit)
- [Documentation](https://github.com/ibrahimcesar/telemetry-kit/tree/main/docs)
- [Examples](https://github.com/ibrahimcesar/telemetry-kit/tree/main/examples)
- [Report Issues](https://github.com/ibrahimcesar/telemetry-kit/issues)

---

**Made with ❤️ for the Rust community**

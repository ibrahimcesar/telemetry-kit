<div align="center">

![Logo](./logo.svg)

_Privacy-first usage analytics for Rust open source projects - understand how your CLI tools and libraries are used in the wild_

[![Crates.io](https://img.shields.io/crates/v/telemetry-kit.svg)](https://crates.io/crates/telemetry-kit)
[![Documentation](https://docs.rs/telemetry-kit/badge.svg)](https://docs.rs/telemetry-kit)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**telemetry-kit** helps open source maintainers understand how their tools are used in the wild. Add privacy-first telemetry in 3 lines of code. Self-host or use our managed service. Perfect for CLI tools, libraries, and Rust applications.

</div>


## 🎉 Currently Working (v0.2.0-alpha.1)

The following features are **fully implemented and tested**:

- ✅ **Interactive Consent Prompts**: First-run consent dialogs for privacy compliance (NEW!)
- ✅ **Smart Instrumentation Recommendations**: AI-powered code analysis suggesting where to add telemetry (NEW!)
- ✅ **Auto-Sync Background Task**: Automatic event synchronization in the background
- ✅ **SQLite → Service Sync Protocol**: Offline-first event buffering with HMAC-SHA256 authentication
- ✅ **Privacy-First User IDs**: Anonymous client identifiers with `client_` prefix (SHA-256 hashed machine IDs)
- ✅ **Event Tracking**: Command and feature event builders with fluent API
- ✅ **Ingestion Server**: Production-ready Rust server with PostgreSQL + Redis
- ✅ **Rate Limiting**: Token-based rate limits (Free/Pro/Business/Enterprise tiers)
- ✅ **Replay Protection**: Nonce-based duplicate request detection
- ✅ **Batch Ingestion**: 1-1000 events per request with partial success handling
- ✅ **Docker Deployment**: Complete docker-compose stack for local development

### Quick Start with Working Features

```bash
# 1. Start the server
cd server
docker compose up -d

# 2. Run the end-to-end test
cd ..
cargo run --example e2e_sync_test
```

See [DEPLOYMENT_GUIDE.md](project-docs/DEPLOYMENT_GUIDE.md) for complete setup instructions.

### Auto-Sync Background Task (NEW! 🎉)

Events are now automatically synced in the background - no manual `.sync()` calls required!

```rust
use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize with auto-sync (enabled by default)
    let telemetry = TelemetryKit::builder()
        .service_name("my-app")?
        .with_sync_credentials(org_id, app_id, token, secret)?
        .auto_sync(true)              // Enable auto-sync (default: true)
        .sync_interval(60)            // Sync every 60 seconds (default)
        .sync_on_shutdown(true)       // Sync before exit (default: true)
        .build()?;

    // Track events - they sync automatically in the background
    telemetry.track_command("build", |event| {
        event.success(true).duration_ms(1234)
    }).await?;

    // Graceful shutdown with final sync
    telemetry.shutdown().await?;
    Ok(())
}
```

**Features:**
- Background tokio task syncs events at configurable intervals
- Graceful shutdown with optional final sync
- Respects DO_NOT_TRACK environment variable
- Exponential backoff on sync failures
- Thread-safe implementation

See [examples/auto_sync.rs](examples/auto_sync.rs) for a complete example.

### CLI Tool (NEW! 🎉)

Manage telemetry configuration and operations from the command line.

```bash
# Install CLI
cargo install telemetry-kit --features cli

# Create a new project with telemetry pre-configured
telemetry-kit new my-app                    # CLI application
telemetry-kit new my-lib -p lib             # Library
telemetry-kit new my-api -p web --with-sync # Web service with sync

# Analyze code for instrumentation opportunities (NEW!)
telemetry-kit analyze                       # Analyze current directory
telemetry-kit analyze --detailed            # Show detailed recommendations with code snippets
telemetry-kit analyze --path src/           # Analyze specific directory
telemetry-kit analyze --format json         # Output as JSON

# Interactive setup (for existing projects)
telemetry-kit init

# View statistics
telemetry-kit stats

# Test sync credentials
telemetry-kit test

# Validate configuration
telemetry-kit validate

# Clean local events
telemetry-kit clean
```

**Available Commands:**
- `new` - Create new projects with telemetry pre-configured
- `analyze` - Get smart recommendations on where to add instrumentation (NEW!)
- `init` - Interactive project setup with credential configuration
- `test` - Validate sync credentials
- `stats` - View event statistics (total, synced, unsynced)
- `sync` - Manually trigger synchronization
- `validate` - Validate configuration
- `clean` - Clear local event database

See [CLI.md](project-docs/CLI.md) for complete CLI documentation.

---

## 🎯 The Problem

As an open source maintainer, you want to understand:
- Which features are actually used vs ignored
- Where users encounter errors or confusion
- How your CLI tool performs in real-world environments
- Whether your library is used correctly

But current telemetry solutions are:
- **Too complex**: Setting up OpenTelemetry requires understanding multiple crates and writing verbose boilerplate
- **Not opinionated**: You don't know *what* to track or *where* to add instrumentation
- **Privacy-blind**: Easy to accidentally log PII, risking your users' trust
- **CLI-unfriendly**: Most tools designed for long-running services, not CLI applications
- **Hard to self-host**: Commercial solutions or complex infrastructure required
- **Trust issues**: Users disable telemetry because they don't trust what's collected

## ✨ The Solution

```rust
use telemetry_kit::prelude::*;

#[tokio::main]
#[telemetry_kit::instrumented]  // 🎉 That's it!
async fn main() {
    telemetry_kit::init()
        .service_name("my-awesome-cli")
        .endpoint("https://telemetry.myapp.com")
        .anonymous()
        .init();
    
    // Your code here - automatically instrumented!
}
```

**What you get as an OSS maintainer:**
- ✅ **3 lines instead of 50+**: Sensible defaults, zero boilerplate
- 🎯 **Smart instrumentation**: Auto-detect CLI commands, errors, and performance bottlenecks
- 🔒 **Privacy-first**: Built-in anonymization, GDPR-compliant, earns user trust
- 🚀 **CLI-optimized**: Works with short-lived processes, offline-capable
- 📦 **Self-hostable**: Simple Docker-based collection server included
- 🤖 **AI-suggested**: Get recommendations on what to instrument
- 📊 **GitHub Badges**: Show usage metrics in your README
- 👥 **Public Dashboards**: Share anonymous analytics with your community

## 🌟 Key Features

### For Open Source Maintainers

**Understand Your Users Without Compromising Their Privacy:**

```rust
// Add to your CLI tool or library
let telemetry = TelemetryKit::builder()
    .service_name("my-awesome-cli")?
    .service_version(env!("CARGO_PKG_VERSION"))
    .prompt_for_consent()?       // Ask users first
    .build()?;

// Track what matters
telemetry.track_command("build", |event| {
    event.success(true).duration_ms(1234)
}).await?;
```

**What You Can Learn:**
- 📊 **Feature Usage**: Which commands/features are popular vs unused
- ⚠️ **Error Patterns**: Where users struggle or encounter bugs
- ⚡ **Performance**: Real-world execution times and bottlenecks
- 🌍 **Platform Distribution**: OS/architecture breakdown
- 📈 **Adoption Trends**: New vs returning users, version migration

**Build Trust With Your Community:**
- 🔒 **Transparent**: Users see exactly what you collect
- 🛑 **Respectful**: DO_NOT_TRACK honored automatically
- ✅ **Consent-First**: Optional interactive prompts
- 📖 **Open Source**: Inspect the code, self-host if preferred
- 🎯 **Anonymous**: No PII, just anonymous usage patterns

**Showcase Your Impact:**
- 🏆 Add usage badges to your README: ![Downloads](https://img.shields.io/badge/active_users-1.2K%2Fmo-blue)
- 📊 Share public dashboards with your community
- 📈 Show growth and adoption metrics to sponsors

### Zero-Config Telemetry
```rust
// Literally one line
telemetry_kit::init().auto_configure()?;
```

### Privacy Controls
```rust
telemetry_kit::init()
    .anonymous()                    // Anonymous user IDs
    .sanitize_emails()              // Hash email addresses
    .exclude_env_vars()             // Don't capture environment
    .gdpr_compliant()               // Full GDPR compliance
    .consent_prompt()               // Ask user on first run
    .init();
```

### Smart Instrumentation (NEW! 🎉)

Enable automatic function instrumentation with the `macros` feature:

```rust
use telemetry_kit::prelude::*;

#[instrument]  // Auto-track duration
async fn fetch_data(url: &str) -> Result<Data, Error> {
    // Function execution is automatically timed
    // Works with both sync and async functions
    // Supports Result types for success/failure tracking
    let response = reqwest::get(url).await?;
    let data = response.json().await?;
    Ok(data)
}
```

**Features:**
- Automatic execution timing for all instrumented functions
- Works with async and sync functions
- Supports functions with or without Result return types
- Zero-overhead when macros feature is disabled
- Compile-time code generation

Enable with:
```toml
[dependencies]
telemetry-kit = { version = "0.2", features = ["macros"] }
```

See [examples/instrument_macro.rs](examples/instrument_macro.rs) for a complete example.

**Note:** Currently the macro measures timing but doesn't send telemetry yet. Full telemetry integration coming soon!

### CLI-Specific Features
```rust
use telemetry_kit::cli::*;

#[derive(Parser)]
#[command(name = "my-cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Automatically tracks:
// - Which commands are used
// - Command duration
// - Success/failure rates
// - Anonymous usage patterns
```

## 🗺️ Roadmap

### Phase 1: Foundation (v0.1.0) - Q1 2025
- [ ] Core telemetry abstraction over OpenTelemetry
- [ ] Privacy-first defaults (anonymization, sanitization)
- [ ] Basic instrumentation macros
- [ ] Simple OTLP export
- [ ] Documentation and examples

### Phase 2: Developer Experience (v0.2.0) - Q2 2025
- [ ] CLI scaffolding tool (`telemetry-kit init`)
- [ ] Auto-detection of instrumentation points
- [ ] Smart suggestions (analyze code, suggest where to add tracking)
- [ ] Pre-built configuration templates
- [ ] VS Code extension for inline suggestions

### Phase 3: CLI Optimization (v0.3.0) - Q2 2025
- [ ] Short-lived process handling
- [ ] Offline caching with automatic sync
- [ ] User consent flow (first-run prompts)
- [ ] Minimal overhead (<1ms impact)
- [ ] Graceful degradation when server unavailable

### Phase 4: Self-Hosting (v0.4.0) - Q3 2025
- [ ] Simple collection server (Docker one-liner)
- [ ] Built-in dashboard for basic analytics
- [ ] SQLite/PostgreSQL storage backends
- [ ] REST API for custom integrations
- [ ] Pre-built dashboards for common patterns

### Phase 5: Advanced Features (v0.5.0+) - Q4 2025
- [ ] AI-powered insights (usage patterns, anomaly detection)
- [ ] Anonymous user cohorts
- [ ] A/B testing support
- [ ] Feature flag integration
- [ ] Custom metric definitions
- [ ] Multi-project dashboards

## 📊 Comparison with Existing Solutions

| Feature | telemetry-kit | OpenTelemetry | tracing | sentry-rust |
|---------|--------------|---------------|---------|-------------|
| **Setup Complexity** | ⭐ Low (3 lines) | ⚠️ High (50+ lines) | ⚠️ Medium | ⭐ Low |
| **Privacy Built-in** | ✅ Yes | ❌ No | ❌ No | ⚠️ Partial |
| **CLI Optimized** | ✅ Yes | ❌ No | ❌ No | ⚠️ Partial |
| **Auto-instrumentation** | ✅ Yes | ❌ No | ❌ No | ⚠️ Errors only |
| **Self-hostable** | ✅ Included | ⚠️ Complex | N/A | ❌ Commercial |
| **Smart Suggestions** | ✅ Yes | ❌ No | ❌ No | ❌ No |
| **Offline Support** | ✅ Yes | ❌ No | N/A | ⚠️ Limited |
| **GDPR Compliant** | ✅ Built-in | ⚠️ Manual | ⚠️ Manual | ⚠️ Manual |

## 🚀 Quick Start

### Installation

```toml
[dependencies]
telemetry-kit = "0.0.1"
```

### Basic Usage

```rust
use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize with defaults
    let _guard = telemetry_kit::init()
        .service_name("my-app")
        .init()?;
    
    // Your application code
    do_work().await?;
    
    Ok(())
}

#[instrument]
async fn do_work() -> Result<()> {
    // Automatically tracked!
    Ok(())
}
```

### CLI Application

```rust
use clap::Parser;
use telemetry_kit::cli::*;

#[derive(Parser)]
#[telemetry_kit::track_commands]  // Auto-track all commands!
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    telemetry_kit::init()
        .for_cli()                // CLI-specific optimizations
        .consent_on_first_run()   // Ask user permission
        .init()?;
    
    match cli.command {
        Commands::Build => build().await?,
        Commands::Deploy => deploy().await?,
    }
    
    Ok(())
}
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Your Application                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Macros     │  │  Middleware  │  │   Manual     │      │
│  │ #[instrument]│  │   Tracking   │  │   Tracking   │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         └──────────────────┴──────────────────┘              │
└─────────────────────────────┬───────────────────────────────┘
                              │
                   ┌──────────▼──────────┐
                   │  telemetry-kit Core │
                   │                     │
                   │ • Privacy filters   │
                   │ • Anonymization     │
                   │ • Batching          │
                   │ • Sampling          │
                   └──────────┬──────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼────────┐  ┌─────────▼────────┐  ┌────────▼────────┐
│ OpenTelemetry  │  │  Simple Server   │  │   Custom        │
│   (OTLP)       │  │  (Self-hosted)   │  │   Backends      │
└────────────────┘  └──────────────────┘  └─────────────────┘
```

## 🔒 Privacy & Compliance

telemetry-kit is **privacy-first by design**:

- **Anonymous by default**: No PII collected unless explicitly enabled
- **User consent**: Built-in consent flow for CLI applications
- **Data sanitization**: Automatic scrubbing of sensitive data
- **GDPR compliant**: Right to erasure, data portability, consent management
- **Transparent**: Users can see exactly what data is collected
- **Opt-out friendly**: Easy to disable, respects DO_NOT_TRACK

### Privacy Features

**Interactive Consent Prompts (NEW!):**

```rust
use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Show interactive consent dialog on first run
    let telemetry = TelemetryKit::builder()
        .service_name("my-app")?
        .service_version("1.0.0")
        .prompt_for_consent()?  // Full prompt with privacy details
        .build()?;

    // Or use minimal one-liner prompt
    let telemetry = TelemetryKit::builder()
        .service_name("my-app")?
        .prompt_minimal()?      // Shorter prompt
        .build()?;

    Ok(())
}
```

**Privacy Configuration:**

```rust
use telemetry_kit::prelude::*;
use telemetry_kit::privacy::PrivacyConfig;

// Strict privacy mode (GDPR-compliant)
let telemetry = TelemetryKit::builder()
    .service_name("my-app")?
    .strict_privacy()  // Requires consent, sanitizes data, 30-day retention
    .build()?;

// Or configure individually
let telemetry = TelemetryKit::builder()
    .service_name("my-app")?
    .consent_required(true)
    .sanitize_paths(true)
    .sanitize_emails(true)
    .data_retention(90)  // Days
    .build()?;
```

**Manage Consent via CLI:**

```bash
telemetry-kit consent grant    # Enable telemetry
telemetry-kit consent deny     # Disable telemetry
telemetry-kit consent status   # Check current status
```

See [examples/interactive_consent.rs](examples/interactive_consent.rs) and [examples/privacy.rs](examples/privacy.rs) for complete examples.

## 🤝 Contributing

We welcome contributions! This is a **new project** and we're building it in the open.

### How to Contribute

1. Check out our [Roadmap](#-roadmap) for planned features
2. Look at [GitHub Issues](https://github.com/ibrahimcesar/telemetry-kit/issues) for tasks
3. Read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines
4. Join discussions in [GitHub Discussions](https://github.com/ibrahimcesar/telemetry-kit/discussions)

### Areas We Need Help

- 🎨 **Design**: API design, ergonomics, developer experience
- 📝 **Documentation**: Examples, tutorials, guides
- 🔧 **Implementation**: Core features, integrations, tools
- 🧪 **Testing**: Unit tests, integration tests, real-world usage
- 🌍 **Community**: Blog posts, talks, spreading the word

## 📚 Documentation

### User Documentation
- [Quick Start Guide](docs/quick-start.md) *(coming soon)*
- [API Reference](https://docs.rs/telemetry-kit)
- [Privacy Guide](docs/privacy.md) *(coming soon)*
- [CLI Best Practices](docs/cli-best-practices.md) *(coming soon)*
- [Self-Hosting Guide](docs/self-hosting.md) *(coming soon)*

### Project Documentation
- [Roadmap](ROADMAP.md) - Feature roadmap and release plan
- [Security Policy](SECURITY.md) - Vulnerability disclosure and security practices
- [SLSA Compliance](SLSA.md) - Supply chain security documentation
- [Contributing Guide](CONTRIBUTING.md) - How to contribute to the project
- [Project Docs](project-docs/README.md) - Internal development documentation

## 💡 Inspiration

This project is inspired by:
- The simplicity of [sentry-rust](https://github.com/getsentry/sentry-rust)
- The power of [OpenTelemetry](https://opentelemetry.io/)
- The ergonomics of [tracing](https://github.com/tokio-rs/tracing)
- The privacy-focus of [Plausible Analytics](https://plausible.io/)
- The developer experience of [Next.js Analytics](https://nextjs.org/analytics)

## 📄 License

This project is dual-licensed under your choice of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

You may choose either license at your option.

### Why Dual License?

We follow the Rust community standard of dual licensing to give you flexibility:

- **Choose MIT** if you prefer simplicity and maximum compatibility (including GPL2)
- **Choose Apache 2.0** if you want explicit patent protection and contributor agreements

Both licenses are permissive and allow commercial use, modification, distribution, and private use.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## 🙏 Acknowledgments

Built with ❤️ by [Ibrahim Cesar](https://github.com/ibrahimcesar) and [contributors](https://github.com/ibrahimcesar/telemetry-kit/graphs/contributors).

Special thanks to:
- The [OpenTelemetry](https://opentelemetry.io/) project
- The [Rust tracing ecosystem](https://tokio.rs/tokio/topics/tracing)
- Everyone who provided feedback and ideas

### GNU Terry Pratchett

This project includes the `X-Clacks-Overhead: GNU Terry Pratchett` header in all HTTP requests to keep the memory of Sir Terry Pratchett alive in the overhead of the internet. Learn more at [gnuterrypratchett.com](http://www.gnuterrypratchett.com/).

> "A man is not dead while his name is still spoken." - Going Postal, Terry Pratchett

---

**Status**: 🚧 **Early Development** - API is unstable and will change

**Current Version**: 0.0.1 - Placeholder release for crate reservation

**First Usable Release**: v0.1.0 (Target: Q1 2025)

[⭐ Star us on GitHub](https://github.com/ibrahimcesar/telemetry-kit) | [📖 Read the Docs](https://docs.rs/telemetry-kit) | [💬 Discussions](https://github.com/ibrahimcesar/telemetry-kit/discussions)


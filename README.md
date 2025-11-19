<div align="center">
  
# 🔭 telemetry-kit

_Privacy-first, batteries-included telemetry for Rust applications_

[![Crates.io](https://img.shields.io/crates/v/telemetry-kit.svg)](https://crates.io/crates/telemetry-kit)
[![Documentation](https://docs.rs/telemetry-kit/badge.svg)](https://docs.rs/telemetry-kit)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**telemetry-kit** makes adding OpenTelemetry to your Rust applications effortless. No more 50+ lines of boilerplate, no more wondering what to instrument, no more privacy concerns.

</div>


## 🎉 Currently Working (v0.0.1)

The following features are **fully implemented and tested**:

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

See [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md) for complete setup instructions.

---

## 🎯 The Problem

Current Rust telemetry solutions are:
- **Too complex**: Setting up OpenTelemetry requires understanding multiple crates and writing verbose boilerplate
- **Not opinionated**: Developers don't know *what* to track or *where* to add instrumentation
- **Privacy-blind**: Easy to accidentally log PII, no built-in anonymization
- **CLI-unfriendly**: Most tools designed for long-running services, not CLI applications
- **Hard to self-host**: Commercial solutions or complex infrastructure required

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

**What you get:**
- ✅ **3 lines instead of 50+**: Sensible defaults, zero boilerplate
- 🎯 **Smart instrumentation**: Auto-detect CLI commands, errors, and performance bottlenecks
- 🔒 **Privacy-first**: Built-in anonymization, GDPR-compliant, opt-in by default
- 🚀 **CLI-optimized**: Works with short-lived processes, offline-capable
- 📦 **Self-hostable**: Simple Docker-based collection server included
- 🤖 **AI-suggested**: Get recommendations on what to instrument

## 🌟 Key Features

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

### Smart Instrumentation
```rust
#[instrument]                       // Auto-track duration & errors
async fn fetch_data(url: &str) -> Result<Data> {
    // Automatically captured:
    // - Function duration
    // - Success/failure
    // - Error messages (sanitized)
    // - Call frequency
}
```

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

```rust
telemetry_kit::init()
    .privacy(|privacy| {
        privacy
            .hash_user_ids()           // SHA-256 user identifiers
            .sanitize_emails()         // email@→hash@domain
            .exclude_ip_addresses()    // Never capture IPs
            .redact_env_vars(&["API_KEY", "TOKEN"])
            .max_string_length(100)    // Truncate long strings
            .exclude_fields(&["password", "secret"])
    })
    .init();
```

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

- [Quick Start Guide](docs/quick-start.md) *(coming soon)*
- [API Reference](https://docs.rs/telemetry-kit)
- [Privacy Guide](docs/privacy.md) *(coming soon)*
- [CLI Best Practices](docs/cli-best-practices.md) *(coming soon)*
- [Self-Hosting Guide](docs/self-hosting.md) *(coming soon)*

## 💡 Inspiration

This project is inspired by:
- The simplicity of [sentry-rust](https://github.com/getsentry/sentry-rust)
- The power of [OpenTelemetry](https://opentelemetry.io/)
- The ergonomics of [tracing](https://github.com/tokio-rs/tracing)
- The privacy-focus of [Plausible Analytics](https://plausible.io/)
- The developer experience of [Next.js Analytics](https://nextjs.org/analytics)

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

Built with ❤️ by [Ibrahim Cesar](https://github.com/ibrahimcesar) and [contributors](https://github.com/ibrahimcesar/telemetry-kit/graphs/contributors).

Special thanks to:
- The [OpenTelemetry](https://opentelemetry.io/) project
- The [Rust tracing ecosystem](https://tokio.rs/tokio/topics/tracing)
- Everyone who provided feedback and ideas

---

**Status**: 🚧 **Early Development** - API is unstable and will change

**Current Version**: 0.0.1 - Placeholder release for crate reservation

**First Usable Release**: v0.1.0 (Target: Q1 2025)

[⭐ Star us on GitHub](https://github.com/ibrahimcesar/telemetry-kit) | [📖 Read the Docs](https://docs.rs/telemetry-kit) | [💬 Discussions](https://github.com/ibrahimcesar/telemetry-kit/discussions)


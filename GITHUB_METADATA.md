# GitHub Repository Metadata

## Positioning Statement

**telemetry-kit** is privacy-first usage analytics for Rust open source maintainers. We help OSS developers understand how their CLI tools, libraries, and applications are used in the wild - without compromising user privacy.

## Target Audience

**Primary:** Open source maintainers of Rust projects
- CLI tool developers (cargo, ripgrep, bat, etc.)
- Library authors (tokio, serde, axum, etc.)
- Framework developers
- Developer tool creators

**Use Cases:**
- Understanding which features are popular vs unused
- Finding error patterns in real-world usage
- Tracking adoption across platforms
- Making data-driven decisions
- Demonstrating project health to sponsors

## GitHub Configuration

### Repository Tagline (One-liner)
```
Privacy-first usage analytics for Rust open source projects - understand how your CLI tools and libraries are used in the wild
```

### Short Description (for shields/badges)
```
Privacy-first telemetry for Rust OSS projects
```

### Full Description
```
Privacy-first telemetry for Rust OSS maintainers. Add usage analytics to CLI tools & libraries in 3 lines. GDPR-compliant, self-hostable, with DO_NOT_TRACK support. Perfect for understanding real-world usage without compromising user privacy.
```

### About Section (160 chars max)
```
Privacy-first telemetry for Rust OSS maintainers. Understand CLI tool & library usage without compromising user privacy. GDPR-compliant, self-hostable.
```

## GitHub Topics (Tags)

**Primary Tags:**
```
rust
telemetry
analytics
open-source
cli
privacy
```

**Secondary Tags:**
```
usage-analytics
rust-library
observability
gdpr
metrics
monitoring
developer-tools
```

**Specific Tags:**
```
oss-maintainers
rust-cli
self-hosted
privacy-first
rust-telemetry
do-not-track
offline-first
```

**Full List (alphabetical):**
```
analytics
cli
developer-tools
do-not-track
gdpr
metrics
monitoring
observability
offline-first
open-source
oss-maintainers
privacy
privacy-first
rust
rust-cli
rust-library
rust-telemetry
self-hosted
telemetry
usage-analytics
```

## Key Messaging

### Value Propositions

**For OSS Maintainers:**
1. **Understand Usage**: See which features users actually use vs ignore
2. **Find Issues**: Identify error patterns before users report them
3. **Make Decisions**: Data-driven feature planning and deprecation
4. **Build Trust**: Privacy-first approach earns user confidence
5. **Show Impact**: Demonstrate project health to sponsors and contributors

**Privacy-First Approach:**
1. **Transparent**: Users see exactly what you collect
2. **Respectful**: DO_NOT_TRACK honored automatically
3. **Consent-First**: Optional interactive prompts
4. **Open Source**: Inspect the code, self-host if preferred
5. **Anonymous**: No PII, just anonymous usage patterns

### Differentiators

**vs OpenTelemetry:**
- 3 lines instead of 50+ lines of boilerplate
- Privacy-first defaults
- CLI-optimized (offline-first, short-lived processes)
- Self-hosting included

**vs Commercial Analytics:**
- Open source and self-hostable
- No vendor lock-in
- Privacy-compliant by default
- Built for OSS, not corporate apps

**vs sentry-rust:**
- Not just errors - full usage analytics
- Lighter weight
- Better for CLI tools
- Privacy-focused from day one

## README Highlights

### Hero Section
```markdown
# telemetry-kit

_Privacy-first usage analytics for Open Source maintainers_

**telemetry-kit** helps open source maintainers understand how their tools
are used in the wild. Add privacy-first telemetry in 3 lines of code.
Self-host or use our managed service. Perfect for CLI tools, libraries,
and Rust applications.
```

### Key Features
- ✅ **3 lines instead of 50+**: Sensible defaults, zero boilerplate
- 🎯 **Smart instrumentation**: Auto-detect CLI commands, errors, bottlenecks
- 🔒 **Privacy-first**: Built-in anonymization, GDPR-compliant, earns user trust
- 🚀 **CLI-optimized**: Works with short-lived processes, offline-capable
- 📦 **Self-hostable**: Simple Docker-based collection server included
- 🤖 **AI-suggested**: Get recommendations on what to instrument
- 📊 **GitHub Badges**: Show usage metrics in your README
- 👥 **Public Dashboards**: Share anonymous analytics with your community

## Social Media

### Twitter/X Bio
```
Privacy-first telemetry for Rust OSS maintainers 🦀
Understand how your CLI tools & libraries are used 📊
3 lines of code, GDPR-compliant, self-hostable 🔒
```

### LinkedIn Description
```
telemetry-kit provides privacy-first usage analytics for Rust open source projects.
We help maintainers understand how their CLI tools and libraries are used in real-world
environments without compromising user privacy. GDPR-compliant, self-hostable, and
designed specifically for the needs of open source developers.
```

### Dev.to/Hashnode Tags
```
#rust #opensource #privacy #telemetry #analytics #cli #developers
```

## Elevator Pitch (30 seconds)

"telemetry-kit is privacy-first usage analytics for Rust open source maintainers.
If you maintain a CLI tool or library, you can add telemetry in just 3 lines of code
to understand which features users actually use, where they encounter errors, and how
your project performs in the wild. Unlike traditional analytics, we're GDPR-compliant
by default, respect DO_NOT_TRACK, and you can self-host everything. It's the guilt-free
way to make data-driven decisions for your open source project."

## FAQ for OSS Maintainers

**Q: Won't my users be upset about telemetry?**
A: Not when it's done right. We provide consent prompts, respect DO_NOT_TRACK, and
never collect PII. Many OSS projects (VS Code, Homebrew, Flutter) use telemetry
successfully because they're transparent about it.

**Q: Why not just use OpenTelemetry?**
A: OpenTelemetry requires 50+ lines of boilerplate and isn't designed for CLI tools.
We give you sensible defaults and handle offline scenarios automatically.

**Q: Can I self-host?**
A: Yes! The collection server is open source and deployable via Docker. You own your data.

**Q: What about GDPR?**
A: Built-in. We use anonymous user IDs, don't collect PII, and provide data sanitization
out of the box.

**Q: How does this help my project?**
A: You'll know which features to prioritize, where users struggle, and how to allocate
development time. Plus, you can show sponsors and contributors that your project is
actively used.

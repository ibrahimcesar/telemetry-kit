# Telemetry Approach: Usage Analytics vs Observability

## Problem Space Definition

**telemetry-kit is NOT an APM/observability tool.** It's a **usage analytics tool for library/tool authors.**

### What We're Building
- **Target Users:** CLI authors, component library creators, SDK maintainers
- **Goal:** Understand how developers use your tool/library
- **Questions to Answer:**
  - "Which commands do users run?"
  - "Which React components get adopted?"
  - "Which SDK methods are popular?"
  - "Should we deprecate feature X?"

### What We're NOT Building
- ❌ Application Performance Monitoring (APM)
- ❌ Error tracking (that's Sentry)
- ❌ Distributed tracing for microservices
- ❌ Real-time observability dashboards
- ❌ End-user behavior analytics (that's PostHog/Mixpanel)

## Similar Tools in the Wild

### Next.js Telemetry
```bash
# Tracks commands, build times, feature usage
next build  # Sends: command=build, duration=5s, version=14.0.0
```

### Homebrew Analytics
```bash
# Tracks which formulas are installed
brew install wget  # Sends: event=install, formula=wget, os=macos
```

### VS Code Telemetry
```typescript
// Tracks extension usage, commands, errors
vscode.commands.executeCommand('extension.myCommand');
// Sends: event=command, command=extension.myCommand
```

### npm (Anonymous Usage Statistics)
```bash
# Tracks npm command usage
npm install  # Sends: command=install, node_version=20.0.0
```

## OpenTelemetry: Wrong Tool for This Job

### Why OpenTelemetry Exists
OpenTelemetry was designed for:
- **Distributed tracing** across microservices
- **Performance monitoring** (latency, throughput)
- **Complex span relationships** (parent → child traces)
- **Integration with APM vendors** (Datadog, New Relic)

### Why It's Overkill for Usage Analytics

| OpenTelemetry | Usage Analytics Needs |
|---------------|----------------------|
| Traces & spans with parent/child | Simple events |
| Complex context propagation | Just count occurrences |
| Sub-millisecond timing | Daily/weekly aggregates |
| 50+ lines of setup | 3 lines of setup |
| Multiple npm packages | Single lightweight SDK |
| Designed for servers | Designed for CLIs/libraries |

### Example Complexity Comparison

**OpenTelemetry approach:**
```rust
use opentelemetry::{global, sdk::trace as sdktrace};
use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::{layer::SubscriberExt, Registry};

let tracer = opentelemetry_otlp::new_pipeline()
    .tracing()
    .with_exporter(
        opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint("http://localhost:4317"),
    )
    .with_trace_config(
        sdktrace::config()
            .with_resource(Resource::new(vec![
                KeyValue::new("service.name", "my-cli"),
            ])),
    )
    .install_batch(opentelemetry::runtime::Tokio)?;

// 30+ more lines for instrumentation...
```

**Simple usage analytics approach:**
```rust
// telemetry-kit for usage analytics
telemetry_kit::init()
    .service_name("my-cli")
    .track_commands()
    .init()?;

// That's it!
```

## Recommended Approach: Simple Event Tracking

### Core Concept
Send **simple JSON events** to a collection endpoint, not complex traces.

### Event Structure
```json
{
  "event_type": "command_executed",
  "timestamp": "2025-01-19T10:30:00Z",
  "user_id": "anonymous_hash_abc123",
  "session_id": "session_xyz789",
  "metadata": {
    "command": "build",
    "version": "1.2.0",
    "os": "macos",
    "success": true
  }
}
```

### Rust SDK API
```rust
use telemetry_kit::prelude::*;

// Initialize
let telemetry = TelemetryKit::init()
    .service_name("my-cli")
    .endpoint("https://telemetry.myapp.com")
    .anonymous()  // Hash user IDs
    .consent_prompt()  // Ask on first run
    .offline_mode()  // Queue if offline
    .init()?;

// Track a command
telemetry.track_command("build", |cmd| {
    cmd.metadata("target", "production")
});

// Track feature usage
telemetry.track_feature("feature_flags", true);

// Track method calls (for SDKs)
telemetry.track_method("Database::connect");
```

### JavaScript SDK API
```typescript
import { telemetryKit } from 'telemetry-kit';

// Initialize
const telemetry = await telemetryKit.init({
  serviceName: 'my-cli',
  endpoint: 'https://telemetry.myapp.com',
  anonymous: true,
  offlineMode: true,
});

// Track a command
telemetry.trackCommand('deploy', {
  target: 'production',
  region: 'us-east-1',
});

// React component tracking
import { useComponentTracking } from 'telemetry-kit/react';

export const Button = ({ variant }) => {
  useComponentTracking('Button', { variant });
  return <button className={variant}>Click</button>;
};
```

## Storage & Sync Strategy

### Local Storage (SQLite)
```
~/.telemetry-kit/my-cli.db

events table:
- id (primary key)
- event_type (string)
- timestamp (datetime)
- metadata (json)
- synced (boolean)
```

### Sync Strategy (Like VS Code)
1. **Offline:** Events stored in SQLite
2. **Online:** Batch sync every 1 hour or 100 events
3. **Retry:** Exponential backoff if sync fails
4. **Privacy:** Never send without consent

### HTTP Protocol
```http
POST /events HTTP/1.1
Host: telemetry.myapp.com
Content-Type: application/json
X-Telemetry-Kit-Version: 0.1.0

{
  "service_name": "my-cli",
  "service_version": "1.2.0",
  "events": [
    {
      "event_type": "command_executed",
      "timestamp": "2025-01-19T10:30:00Z",
      "metadata": { "command": "build" }
    }
  ]
}
```

## Collection Backend

### Simple Node.js Server
```typescript
// server/index.ts
import express from 'express';
import { TelemetryCollector } from './collector';

const app = express();
const collector = new TelemetryCollector({
  storage: 'postgres', // or 'sqlite'
});

app.post('/events', async (req, res) => {
  const { service_name, events } = req.body;

  // Validate & store
  await collector.store(service_name, events);

  res.json({ received: events.length });
});

app.listen(3000);
```

### Storage Schema (PostgreSQL)
```sql
CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  service_name VARCHAR(255),
  service_version VARCHAR(50),
  event_type VARCHAR(100),
  timestamp TIMESTAMPTZ,
  user_id VARCHAR(255),  -- Hashed
  session_id VARCHAR(255),
  metadata JSONB,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_service_event ON events(service_name, event_type);
CREATE INDEX idx_timestamp ON events(timestamp);
```

### Analytics Queries
```sql
-- Top commands
SELECT
  metadata->>'command' as command,
  COUNT(*) as usage_count
FROM events
WHERE service_name = 'my-cli'
  AND event_type = 'command_executed'
  AND timestamp > NOW() - INTERVAL '30 days'
GROUP BY metadata->>'command'
ORDER BY usage_count DESC;

-- Feature adoption over time
SELECT
  DATE(timestamp) as date,
  COUNT(DISTINCT user_id) as daily_active_users
FROM events
WHERE service_name = 'my-cli'
  AND event_type = 'feature_used'
  AND metadata->>'feature' = 'deploy'
GROUP BY DATE(timestamp)
ORDER BY date DESC;
```

## Privacy-First Design

### Anonymous by Default
- **User ID:** SHA-256 hash of machine ID + salt
- **No PII:** Never collect emails, IPs, usernames
- **Opt-in:** Require explicit consent on first run

### Consent Flow (CLI)
```
$ my-cli build

┌─────────────────────────────────────────────────────────────┐
│ 📊 Help us improve my-cli                                   │
│                                                              │
│ We collect anonymous usage data to understand which         │
│ features are used and prioritize development.               │
│                                                              │
│ Data collected:                                             │
│  • Commands executed (e.g., "build", "deploy")             │
│  • Success/failure rates                                    │
│  • Tool version and OS                                      │
│                                                              │
│ We DO NOT collect:                                          │
│  • File names, paths, or code                              │
│  • Personal information                                     │
│  • Environment variables                                    │
│                                                              │
│ Learn more: https://my-cli.com/telemetry                   │
│                                                              │
│ [Y] Yes, help improve  [N] No thanks  [?] Show me the data │
└─────────────────────────────────────────────────────────────┘
```

### DO_NOT_TRACK Support
```bash
# Respect user preference
export DO_NOT_TRACK=1
my-cli build  # No telemetry sent
```

## Comparison: OpenTelemetry vs Simple Events

| Aspect | OpenTelemetry | Simple Events (Recommended) |
|--------|---------------|---------------------------|
| **Complexity** | High (50+ lines) | Low (3 lines) |
| **Dependencies** | 5+ crates/packages | 1 lightweight crate |
| **Use Case** | APM, distributed tracing | Usage analytics |
| **Overhead** | ~5-10ms per operation | <1ms per event |
| **Storage** | Specialized backends | SQLite + PostgreSQL |
| **Protocol** | OTLP (gRPC/HTTP) | Simple HTTP JSON |
| **Learning Curve** | Steep | Minimal |
| **CLI-Friendly** | No | Yes (offline, batching) |
| **Privacy Built-in** | No | Yes (anonymous, opt-in) |

## When to Use OpenTelemetry

Use OpenTelemetry if you need:
- ✅ Distributed tracing across microservices
- ✅ APM integration (Datadog, New Relic)
- ✅ Sub-millisecond timing precision
- ✅ Complex span relationships

**Don't use OpenTelemetry for:**
- ❌ Simple usage analytics
- ❌ CLI command tracking
- ❌ Library adoption metrics
- ❌ Feature usage counting

## Recommendation

**Build telemetry-kit with simple event tracking, NOT OpenTelemetry.**

### Why?
1. **Better fit:** Usage analytics ≠ APM
2. **Simpler:** 3 lines vs 50+ lines
3. **Lighter:** Smaller binary, faster startup
4. **CLI-optimized:** Offline mode, batching built-in
5. **Privacy-first:** Easier to reason about what's sent

### Optional: OpenTelemetry Export
If users want to export to OpenTelemetry backends:
```rust
telemetry_kit::init()
    .export_to_otlp("http://localhost:4317")  // Optional!
    .init()?;
```

But the **default** should be simple events, not OTLP.

## Next Steps

1. Design simple event protocol (JSON schema)
2. Implement Rust SDK with SQLite + HTTP sync
3. Build minimal collection server (Node.js + PostgreSQL)
4. Create privacy-first consent flows
5. Add JavaScript/TypeScript SDK with same patterns
6. Build analytics dashboard for library authors

---

**Key Insight:** telemetry-kit is for **developers who want to understand their tool's usage**, not for **enterprises monitoring production systems**.

Simple event tracking is the right tool for this job.

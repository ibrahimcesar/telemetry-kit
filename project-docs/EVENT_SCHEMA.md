# Event Schema Specification

**Version:** 1.0.0
**Status:** Draft
**Language Support:** Rust, JavaScript, Python, Go, and more

---

## Design Principles

1. **Language-agnostic:** JSON format, works everywhere
2. **Versioned:** Schema version in every event
3. **Extensible:** Custom metadata without breaking changes
4. **Privacy-first:** No PII by default
5. **Compact:** Small payload size for offline batching
6. **Analyzable:** Easy to query in SQL/NoSQL

---

## Core Event Structure

### Base Event Format

Every event sent by any SDK follows this structure:

```json
{
  "schema_version": "1.0.0",
  "event_id": "uuid-v4-here",
  "timestamp": "2025-01-19T15:45:00.000Z",
  "service": {
    "name": "my-cli",
    "version": "1.2.0",
    "language": "rust",
    "language_version": "1.75.0"
  },
  "user": {
    "id": "client_abc123def456",
    "session_id": "sess_xyz789"
  },
  "environment": {
    "os": "macos",
    "os_version": "14.2.1",
    "arch": "aarch64",
    "ci": false
  },
  "event": {
    "type": "command_executed",
    "category": "cli",
    "data": {
      "command": "build",
      "subcommand": null,
      "flags": ["--production", "--verbose"],
      "success": true,
      "duration_ms": 5430,
      "exit_code": 0
    }
  },
  "metadata": {
    "sdk_version": "0.1.0",
    "sent_at": "2025-01-19T15:45:05.123Z",
    "nounce": "765323"
  }
}
```

---

## Field Specifications

### `schema_version` (required)
- **Type:** String (semver)
- **Purpose:** Allow schema evolution
- **Example:** `"1.0.0"`
- **Validation:** Must match `^\d+\.\d+\.\d+$`

### `event_id` (required)
- **Type:** String (UUID v4)
- **Purpose:** Deduplication, event tracking
- **Example:** `"550e8400-e29b-41d4-a716-446655440000"`
- **Generated:** Client-side, before sending

### `timestamp` (required)
- **Type:** String (ISO 8601 with milliseconds)
- **Purpose:** When event occurred
- **Example:** `"2025-01-19T15:45:00.000Z"`
- **Timezone:** Always UTC
- **Note:** Different from `sent_at` (transmission time)

---

## Service Block (required)

Information about the tool/library generating events.

```json
{
  "service": {
    "name": "my-cli",           // Required
    "version": "1.2.0",          // Required
    "language": "rust",          // Required: rust|javascript|python|go
    "language_version": "1.75.0" // Optional but recommended
  }
}
```

### `service.name` (required)
- **Type:** String
- **Purpose:** Identify which tool is sending events
- **Example:** `"my-cli"`, `"my-react-library"`, `"my-sdk"`
- **Validation:** `^[a-z0-9-_]+$` (lowercase, alphanumeric, dashes, underscores)

### `service.version` (required)
- **Type:** String (semver recommended)
- **Purpose:** Track usage by version
- **Example:** `"1.2.0"`, `"0.5.0-beta.1"`

### `service.language` (required)
- **Type:** String (enum)
- **Purpose:** Multi-language analytics
- **Values:** `rust`, `javascript`, `python`, `go`, `csharp`, `java`, `ruby`

### `service.language_version` (optional)
- **Type:** String
- **Purpose:** Debug compatibility issues
- **Example:** `"1.75.0"` (Rust), `"20.11.0"` (Node.js), `"3.12.0"` (Python)

---

## User Block (required)

Anonymous user identification.

```json
{
  "user": {
    "id": "client_abc123def456",
    "session_id": "sess_xyz789"
  }
}
```

### `user.id` (required)
- **Type:** String
- **Purpose:** Track unique users (anonymously)
- **Format:** SHA-256 hash of machine ID + salt
- **Example:** `"client_" + first_12_chars_of_hash`
- **Privacy:** NEVER send actual user IDs, emails, or names

**Generation (Rust):**
```rust
let machine_id = get_machine_id()?;
let salt = "telemetry-kit-v1";
let hash = sha256(format!("{}{}", machine_id, salt));
let user_id = format!("client_{}", &hash[..12]);
```

**Generation (JavaScript):**
```typescript
const machineId = getMachineId();
const salt = 'telemetry-kit-v1';
const hash = sha256(`${machineId}${salt}`);
const userId = `client_${hash.substring(0, 12)}`;
```

### `user.session_id` (optional)
- **Type:** String
- **Purpose:** Group events in a single run
- **Example:** `"sess_" + uuid_first_8_chars`
- **Lifetime:** Generated once per process/run

---

## Environment Block (optional but recommended)

Context about where the tool is running.

```json
{
  "environment": {
    "os": "macos",
    "os_version": "14.2.1",
    "arch": "aarch64",
    "ci": false,
    "terminal": "kitty",
    "shell": "zsh"
  }
}
```

### `environment.os` (required)
- **Type:** String (enum)
- **Values:** `linux`, `macos`, `windows`, `freebsd`, `unknown`
- **Purpose:** Platform-specific analytics

### `environment.os_version` (optional)
- **Type:** String
- **Example:** `"14.2.1"` (macOS), `"Ubuntu 22.04"` (Linux)

### `environment.arch` (optional)
- **Type:** String
- **Values:** `x86_64`, `aarch64`, `i686`, `arm`, `unknown`

### `environment.ci` (optional)
- **Type:** Boolean
- **Purpose:** Distinguish CI vs developer usage
- **Detection:** Check env vars: `CI`, `GITHUB_ACTIONS`, `JENKINS_HOME`, etc.

### `environment.terminal` (optional)
- **Type:** String
- **Example:** `"kitty"`, `"iterm2"`, `"wezterm"`, `"vscode"`

### `environment.shell` (optional)
- **Type:** String
- **Example:** `"zsh"`, `"bash"`, `"fish"`, `"powershell"`

---

## Event Block (required)

The actual event data.

```json
{
  "event": {
    "type": "command_executed",
    "category": "cli",
    "data": {
      // Event-specific fields
    }
  }
}
```

### `event.type` (required)
- **Type:** String
- **Purpose:** What happened
- **Examples:** `command_executed`, `feature_used`, `method_called`, `component_rendered`, `error_occurred`

### `event.category` (optional)
- **Type:** String (enum)
- **Purpose:** Group similar events
- **Values:** `cli`, `library`, `component`, `sdk`, `other`

### `event.data` (required)
- **Type:** Object
- **Purpose:** Event-specific payload
- **Schema:** Varies by `event.type` (see Event Types below)

---

## Event Types

### 1. Command Executed (CLI)

**Type:** `command_executed`

```json
{
  "event": {
    "type": "command_executed",
    "category": "cli",
    "data": {
      "command": "build",
      "subcommand": null,
      "flags": ["--production", "--verbose"],
      "success": true,
      "duration_ms": 5430,
      "exit_code": 0
    }
  }
}
```

**Fields:**
- `command` (string, required): Main command name
- `subcommand` (string|null): Nested command if any
- `flags` (array[string], optional): Flags used (sanitized, no values)
- `success` (boolean, required): Did it complete successfully?
- `duration_ms` (integer, optional): Execution time
- `exit_code` (integer, optional): Process exit code

---

### 2. Feature Used (Library/SDK)

**Type:** `feature_used`

```json
{
  "event": {
    "type": "feature_used",
    "category": "library",
    "data": {
      "feature": "authentication",
      "method": "email_password",
      "success": true
    }
  }
}
```

**Fields:**
- `feature` (string, required): Feature name
- `method` (string, optional): Variant or method used
- `success` (boolean, optional): Did it work?

---

### 3. Method Called (SDK)

**Type:** `method_called`

```json
{
  "event": {
    "type": "method_called",
    "category": "sdk",
    "data": {
      "class": "Database",
      "method": "connect",
      "async": true,
      "duration_ms": 234
    }
  }
}
```

**Fields:**
- `class` (string, optional): Class/module name
- `method` (string, required): Method/function name
- `async` (boolean, optional): Was it async?
- `duration_ms` (integer, optional): Execution time

---

### 4. Component Rendered (React/UI)

**Type:** `component_rendered`

```json
{
  "event": {
    "type": "component_rendered",
    "category": "component",
    "data": {
      "component": "Button",
      "variant": "ghost",
      "props_hash": "abc123",
      "render_count": 1
    }
  }
}
```

**Fields:**
- `component` (string, required): Component name
- `variant` (string, optional): Component variant
- `props_hash` (string, optional): Hash of props (privacy)
- `render_count` (integer, optional): How many times rendered in session

---

### 5. Error Occurred (Optional)

**Type:** `error_occurred`

```json
{
  "event": {
    "type": "error_occurred",
    "category": "cli",
    "data": {
      "error_type": "NetworkError",
      "command": "deploy",
      "fatal": true
    }
  }
}
```

**Fields:**
- `error_type` (string, required): Error classification (sanitized)
- `command` (string, optional): What command failed
- `fatal` (boolean, optional): Did it crash?

**Note:** For full error tracking, use Sentry. This is just for analytics.

---

## Metadata Block (required)

SDK metadata.

```json
{
  "metadata": {
    "sdk_version": "0.1.0",
    "sent_at": "2025-01-19T15:45:05.123Z",
    "batch_size": 1,
    "retry_count": 0
  }
}
```

### `metadata.sdk_version` (required)
- **Type:** String
- **Purpose:** Track SDK version for compatibility
- **Example:** `"0.1.0"`

### `metadata.sent_at` (required)
- **Type:** String (ISO 8601)
- **Purpose:** When event was transmitted (vs when it occurred)
- **Example:** `"2025-01-19T15:45:05.123Z"`

### `metadata.batch_size` (optional)
- **Type:** Integer
- **Purpose:** How many events in this batch
- **Example:** `10`

### `metadata.retry_count` (optional)
- **Type:** Integer
- **Purpose:** How many times retried
- **Example:** `0` (first attempt), `2` (third attempt)

---

## Batch Format

When sending multiple events at once:

```json
{
  "batch_id": "batch_uuid_here",
  "events": [
    { /* event 1 */ },
    { /* event 2 */ },
    { /* event 3 */ }
  ]
}
```

---

## Language-Specific Examples

### Rust SDK

```rust
use telemetry_kit::prelude::*;

let telemetry = TelemetryKit::init()
    .service_name("my-cli")
    .service_version(env!("CARGO_PKG_VERSION"))
    .init()?;

// Track command
telemetry.track_command("build", |event| {
    event
        .flag("--production")
        .flag("--verbose")
        .duration(Duration::from_millis(5430))
        .success(true)
});
```

**Generated Event:**
```json
{
  "schema_version": "1.0.0",
  "event_id": "...",
  "timestamp": "...",
  "service": {
    "name": "my-cli",
    "version": "1.2.0",
    "language": "rust",
    "language_version": "1.75.0"
  },
  "event": {
    "type": "command_executed",
    "category": "cli",
    "data": {
      "command": "build",
      "flags": ["--production", "--verbose"],
      "duration_ms": 5430,
      "success": true
    }
  }
}
```

---

### JavaScript SDK

```typescript
import { telemetryKit } from '@telemetry-kit/node';

const telemetry = await telemetryKit.init({
  serviceName: 'my-cli',
  serviceVersion: '1.2.0',
});

// Track command
telemetry.trackCommand('deploy', {
  flags: ['--target=production'],
  success: true,
  durationMs: 3200,
});
```

**Generated Event:**
```json
{
  "schema_version": "1.0.0",
  "event_id": "...",
  "timestamp": "...",
  "service": {
    "name": "my-cli",
    "version": "1.2.0",
    "language": "javascript",
    "language_version": "20.11.0"
  },
  "event": {
    "type": "command_executed",
    "category": "cli",
    "data": {
      "command": "deploy",
      "flags": ["--target=production"],
      "duration_ms": 3200,
      "success": true
    }
  }
}
```

---

### React Component

```typescript
import { useComponentTracking } from '@telemetry-kit/react';

export const Button = ({ variant, size }) => {
  useComponentTracking('Button', { variant, size });

  return <button className={variant}>{children}</button>;
};
```

**Generated Event:**
```json
{
  "schema_version": "1.0.0",
  "service": {
    "name": "my-design-system",
    "version": "2.1.0",
    "language": "javascript"
  },
  "event": {
    "type": "component_rendered",
    "category": "component",
    "data": {
      "component": "Button",
      "variant": "ghost",
      "props_hash": "abc123"
    }
  }
}
```

---

## Privacy Considerations

### ✅ Safe to Collect

- Command names (e.g., `"build"`, `"deploy"`)
- Feature usage (e.g., `"authentication used"`)
- Success/failure rates
- Duration metrics
- OS, architecture
- Tool version

### ❌ Never Collect (PII)

- File paths (e.g., `/Users/john/secret-project`)
- Email addresses
- API keys, tokens
- Environment variable values
- User names
- IP addresses (handled at server level)
- Git commit messages
- Code snippets

### 🛡️ Sanitization Rules

**Flag Values:**
```bash
# Input:  --api-key=sk_live_abc123
# Output: --api-key           (flag name only)

# Input:  --target=production
# Output: --target=production  (safe value, kept)
```

**Error Messages:**
```rust
// Input:  "File not found: /Users/john/app/config.json"
// Output: "File not found: [REDACTED]"

// Input:  "Invalid API key: sk_live_abc123"
// Output: "Invalid API key: [REDACTED]"
```

---

## Schema Versioning

### Version 1.0.0 (Current)
- Initial schema
- Support for: commands, features, methods, components

### Future: Version 1.1.0
- Add `custom_dimensions` field for arbitrary data
- Add `performance.memory_mb` field

### Future: Version 2.0.0
- Breaking: Change `event.data` structure
- Require migration for existing events

**Backward Compatibility:**
- Server must accept old schema versions
- SDKs should use latest schema
- Analytics queries handle multiple versions

---

## Wire Protocol

### HTTP POST to Collection Server

**Endpoint:** `POST /events`

**Headers:**
```
Content-Type: application/json
X-Telemetry-Kit-Version: 1.0.0
X-Service-Name: my-cli
X-Service-Version: 1.2.0
User-Agent: telemetry-kit-rust/0.1.0
```

**Body (Single Event):**
```json
{
  "schema_version": "1.0.0",
  // ... full event
}
```

**Body (Batch):**
```json
{
  "batch_id": "uuid-here",
  "events": [
    { /* event 1 */ },
    { /* event 2 */ }
  ]
}
```

**Response (Success):**
```json
{
  "status": "ok",
  "received": 2,
  "batch_id": "uuid-here"
}
```

**Response (Error):**
```json
{
  "status": "error",
  "error": "invalid_schema",
  "message": "Missing required field: event.type"
}
```

---

## Validation Rules

### Server-Side Validation

1. **Schema Version:** Must be valid semver
2. **Event ID:** Must be valid UUID v4
3. **Timestamp:** Must be ISO 8601, within 7 days
4. **Service Name:** Must match `^[a-z0-9-_]+$`
5. **User ID:** Must start with `client_`
6. **Event Type:** Must be known type or `custom`
7. **Max Size:** Event must be < 100KB

### Client-Side Validation

SDKs should validate before sending:
- All required fields present
- Types are correct
- No PII in free-text fields (basic check)

---

## Storage Schema (PostgreSQL)

```sql
CREATE TABLE events (
  id BIGSERIAL PRIMARY KEY,
  event_id UUID UNIQUE NOT NULL,
  schema_version VARCHAR(20) NOT NULL,
  timestamp TIMESTAMPTZ NOT NULL,

  -- Service
  service_name VARCHAR(255) NOT NULL,
  service_version VARCHAR(100) NOT NULL,
  service_language VARCHAR(50) NOT NULL,
  service_language_version VARCHAR(50),

  -- User (anonymous)
  user_id VARCHAR(255) NOT NULL,
  session_id VARCHAR(255),

  -- Environment
  os VARCHAR(50),
  os_version VARCHAR(100),
  arch VARCHAR(50),
  ci BOOLEAN,

  -- Event
  event_type VARCHAR(100) NOT NULL,
  event_category VARCHAR(50),
  event_data JSONB NOT NULL,

  -- Metadata
  sdk_version VARCHAR(50) NOT NULL,
  sent_at TIMESTAMPTZ NOT NULL,
  received_at TIMESTAMPTZ DEFAULT NOW(),

  -- Indexes
  INDEX idx_service_timestamp (service_name, timestamp DESC),
  INDEX idx_event_type (event_type),
  INDEX idx_user_id (user_id),
  INDEX idx_timestamp (timestamp DESC)
);
```

---

## Next Steps

1. **Implement schema validation** in Rust SDK
2. **Add examples** for each event type
3. **Create test fixtures** for multi-language testing
4. **Design custom event support** for advanced users
5. **Add schema migration guide** for future versions

---

**Last Updated:** 2025-01-19
**Status:** Draft (Open for feedback)
**Feedback:** Open GitHub issue or PR

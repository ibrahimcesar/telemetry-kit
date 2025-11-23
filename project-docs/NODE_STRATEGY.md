# Node.js Ecosystem Strategy

**Date:** 2025-01-23
**Status:** Planning Phase
**Target:** v0.6.0+ (Post-1.0.0)

---

## Executive Summary

This document outlines strategies for bringing telemetry-kit's privacy-first telemetry to the Node.js/TypeScript ecosystem, leveraging our Rust core while providing idiomatic JavaScript/TypeScript APIs.

**Goal:** Privacy-first telemetry that works seamlessly in Node.js, with the same principles as the Rust SDK.

---

## Strategic Options

### Option 1: Native Node Addon (Recommended) ⭐

**Approach:** Compile Rust code to native Node.js addon using napi-rs

**Pros:**
- ✅ Reuses 100% of Rust core logic (security, privacy, storage)
- ✅ Best performance (native speed)
- ✅ Consistent behavior across languages
- ✅ Single source of truth for security-critical code
- ✅ Mature tooling (napi-rs is production-ready)
- ✅ TypeScript support built-in

**Cons:**
- ⚠️ Requires native compilation (prebuilt binaries solve this)
- ⚠️ Platform-specific builds (but napi-rs handles this)
- ⚠️ Larger package size (~2-5MB per platform)

**Example API:**
```typescript
import { TelemetryKit } from '@telemetry-kit/node';

const telemetry = TelemetryKit.builder()
  .serviceName('my-node-app')
  .withSyncCredentials(orgId, appId, token, secret)
  .autoSync(true)
  .build();

await telemetry.trackCommand('deploy', {
  success: true,
  durationMs: 1234
});

await telemetry.shutdown();
```

**Implementation:**
```rust
// src/bindings/node.rs
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct TelemetryKitNode {
    inner: crate::TelemetryKit,
}

#[napi]
impl TelemetryKitNode {
    #[napi(constructor)]
    pub fn new(config: JsConfig) -> Result<Self> {
        let inner = crate::TelemetryKit::builder()
            .service_name(config.service_name)?
            .build()?;
        Ok(Self { inner })
    }

    #[napi]
    pub async fn track_command(&self, name: String, data: JsObject) -> Result<()> {
        // Convert JS object to Rust types
        // Call inner.track_command()
        Ok(())
    }
}
```

**Toolchain:**
- [napi-rs](https://napi.rs/) - Rust → Node.js bindings
- Automated CI for multiple platforms (Linux, macOS, Windows, ARM)
- npm package with prebuilt binaries

---

### Option 2: WebAssembly (WASM)

**Approach:** Compile Rust to WASM, use in Node.js via wasm-pack

**Pros:**
- ✅ Cross-platform (no native compilation needed)
- ✅ Smaller package size
- ✅ Works in browser too (bonus!)
- ✅ Reuses Rust core

**Cons:**
- ❌ Can't access native filesystem (SQLite storage problematic)
- ❌ Limited async support
- ❌ No threading (background sync harder)
- ❌ Performance overhead vs native
- ⚠️ Requires polyfills for Node.js APIs

**Use Case:** Better for browser than Node.js server-side

**Example:**
```typescript
import init, { TelemetryKit } from '@telemetry-kit/wasm';

await init(); // Load WASM module

const telemetry = new TelemetryKit({
  serviceName: 'my-app',
  // Storage would need to use IndexedDB or memory
});
```

---

### Option 3: Pure TypeScript Rewrite

**Approach:** Reimplement telemetry-kit in TypeScript

**Pros:**
- ✅ Native Node.js experience
- ✅ No build complexity
- ✅ Easy to contribute for JS developers
- ✅ Smaller package size

**Cons:**
- ❌ Duplicate code (maintenance burden)
- ❌ Security-critical code duplicated (HMAC, crypto)
- ❌ Behavior divergence risk
- ❌ Less performance than Rust
- ❌ More dependencies (crypto, SQLite bindings, etc.)

**Verdict:** Not recommended due to duplication and security concerns

---

### Option 4: Hybrid Approach

**Approach:** Pure TS for simple use cases, native addon for advanced features

**Architecture:**
```
@telemetry-kit/core (TypeScript)
  ├─ Basic event tracking
  ├─ Privacy helpers
  └─ Simple in-memory storage

@telemetry-kit/native (Rust → napi-rs)
  ├─ SQLite storage
  ├─ HMAC authentication
  ├─ Background sync
  └─ Advanced privacy features

@telemetry-kit/node (TypeScript wrapper)
  └─ Auto-selects core vs native based on needs
```

**Pros:**
- ✅ Fast install for simple cases (no native compilation)
- ✅ Full power when needed
- ✅ Best of both worlds

**Cons:**
- ⚠️ More complex architecture
- ⚠️ Two implementations to maintain

---

## Recommended Architecture

### Phase 1: Native Addon (napi-rs) ⭐

**Why:** Maximum code reuse, best security, consistent behavior

**Package Structure:**
```
packages/
├── rust-core/              # Existing Rust SDK
│   └── src/
│       └── bindings/
│           └── node.rs     # napi-rs bindings
│
├── node/                   # @telemetry-kit/node
│   ├── src/
│   │   ├── index.ts       # TypeScript wrapper
│   │   └── types.ts       # TypeScript definitions
│   ├── native/            # Prebuilt binaries (CI generated)
│   │   ├── linux-x64/
│   │   ├── darwin-arm64/
│   │   ├── win32-x64/
│   │   └── ...
│   └── package.json
│
└── examples/
    └── node-example/
```

**TypeScript API:**
```typescript
// index.ts
import { TelemetryKitNative } from './native';

export class TelemetryKit {
  private native: TelemetryKitNative;

  static builder(): TelemetryBuilder {
    return new TelemetryBuilder();
  }

  async trackCommand(name: string, options?: CommandOptions): Promise<void> {
    return this.native.trackCommand(name, options);
  }

  async trackFeature(name: string, options?: FeatureOptions): Promise<void> {
    return this.native.trackFeature(name, options);
  }

  async shutdown(): Promise<void> {
    return this.native.shutdown();
  }
}

export class TelemetryBuilder {
  private config: Config = {};

  serviceName(name: string): this {
    this.config.serviceName = name;
    return this;
  }

  withSyncCredentials(
    orgId: string,
    appId: string,
    token: string,
    secret: string
  ): this {
    this.config.sync = { orgId, appId, token, secret };
    return this;
  }

  autoSync(enabled: boolean): this {
    this.config.autoSync = enabled;
    return this;
  }

  build(): TelemetryKit {
    const native = new TelemetryKitNative(this.config);
    return new TelemetryKit(native);
  }
}
```

**package.json:**
```json
{
  "name": "@telemetry-kit/node",
  "version": "0.1.0",
  "description": "Privacy-first telemetry for Node.js",
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "napi": {
    "name": "telemetry-kit",
    "triples": {
      "defaults": true,
      "additional": [
        "aarch64-apple-darwin",
        "aarch64-linux-android",
        "aarch64-unknown-linux-gnu",
        "armv7-unknown-linux-gnueabihf",
        "x86_64-unknown-linux-musl"
      ]
    }
  },
  "dependencies": {},
  "devDependencies": {
    "@napi-rs/cli": "^2.18.0",
    "typescript": "^5.3.0"
  },
  "scripts": {
    "build": "napi build --platform --release",
    "artifacts": "napi artifacts"
  }
}
```

---

## Implementation Roadmap

### Phase 1: Proof of Concept (2 weeks)

**Goal:** Validate napi-rs approach with basic functionality

**Tasks:**
- [ ] Set up napi-rs in telemetry-kit repo
- [ ] Create basic Node.js bindings (builder, track_command)
- [ ] Implement TypeScript wrapper
- [ ] Build for single platform (local development)
- [ ] Create example Node.js app
- [ ] Validate privacy features work

**Deliverable:** Working proof-of-concept on macOS/Linux

---

### Phase 2: Production Bindings (4 weeks)

**Goal:** Complete API surface with proper error handling

**Tasks:**
- [ ] Implement all SDK methods (track_feature, privacy, consent)
- [ ] Proper error conversion (Rust → JS exceptions)
- [ ] Async/await support (all methods async in JS)
- [ ] TypeScript type definitions
- [ ] Event builder API
- [ ] Privacy controls API
- [ ] Consent management API

**Deliverable:** Feature-complete Node.js SDK

---

### Phase 3: CI/CD & Publishing (2 weeks)

**Goal:** Automated builds for all platforms

**Tasks:**
- [ ] GitHub Actions workflow for cross-compilation
- [ ] Build matrix (Linux x64/ARM, macOS x64/ARM, Windows x64/ARM)
- [ ] Prebuilt binary publishing
- [ ] npm package setup
- [ ] Versioning strategy (sync with Rust SDK)
- [ ] Test suite for Node.js bindings

**CI Platforms:**
```yaml
# .github/workflows/node-bindings.yml
strategy:
  matrix:
    settings:
      - host: macos-latest
        target: x86_64-apple-darwin
      - host: macos-latest
        target: aarch64-apple-darwin
      - host: ubuntu-latest
        target: x86_64-unknown-linux-gnu
      - host: ubuntu-latest
        target: aarch64-unknown-linux-gnu
      - host: windows-latest
        target: x86_64-pc-windows-msvc
```

**Deliverable:** Published to npm with prebuilt binaries

---

### Phase 4: Documentation & Examples (2 weeks)

**Goal:** Make it easy for Node.js developers to adopt

**Tasks:**
- [ ] Quick start guide (Node.js/TypeScript)
- [ ] API documentation (TypeDoc)
- [ ] Migration guide (from existing solutions)
- [ ] Example projects:
  - Express.js server
  - Next.js app
  - CLI tool (yargs/commander)
  - Lambda function
- [ ] Integration guides (Vercel, Netlify, Railway)

**Deliverable:** Comprehensive Node.js documentation

---

## API Design Principles

### 1. Idiomatic JavaScript

**Use async/await everywhere:**
```typescript
// Good - matches Node.js patterns
await telemetry.trackCommand('deploy', { success: true });

// Bad - sync in async language
telemetry.trackCommandSync('deploy', { success: true });
```

**Use builder pattern (matches Rust):**
```typescript
const telemetry = TelemetryKit.builder()
  .serviceName('my-app')
  .autoSync(true)
  .build();
```

### 2. TypeScript-First

**Strong typing:**
```typescript
interface CommandOptions {
  success?: boolean;
  durationMs?: number;
  metadata?: Record<string, string | number | boolean>;
}

interface FeatureOptions {
  enabled?: boolean;
  metadata?: Record<string, string | number | boolean>;
}
```

**Type inference:**
```typescript
// Infers CommandOptions
await telemetry.trackCommand('build', {
  success: true,
  durationMs: 1234
});
```

### 3. Privacy Controls

**Same API as Rust:**
```typescript
const telemetry = TelemetryKit.builder()
  .serviceName('my-app')
  .privacy({
    anonymizeEmails: true,
    sanitizePaths: true,
    respectDoNotTrack: true
  })
  .consent({
    required: true,
    defaultGranted: false
  })
  .build();
```

### 4. Error Handling

**JavaScript-style errors:**
```typescript
try {
  await telemetry.trackCommand('deploy', { success: true });
} catch (error) {
  if (error instanceof TelemetryError) {
    console.error('Telemetry error:', error.message);
    // Error has helpful context, not just Rust stack trace
  }
}
```

---

## Framework Integrations

### Express.js Middleware

```typescript
import { createTelemetryMiddleware } from '@telemetry-kit/express';

const app = express();

app.use(createTelemetryMiddleware({
  serviceName: 'my-api',
  trackRoutes: true,
  trackErrors: true
}));

// Automatically tracks:
// - Route hits
// - Response times
// - Error rates
// - Status codes
```

### Next.js Plugin

```typescript
// next.config.js
import { withTelemetry } from '@telemetry-kit/next';

export default withTelemetry({
  // Next.js config
}, {
  // Telemetry config
  serviceName: 'my-next-app',
  autoSync: true
});
```

### CLI Integration (Commander.js)

```typescript
import { program } from 'commander';
import { trackCommand } from '@telemetry-kit/cli';

program
  .command('deploy')
  .action(trackCommand(async () => {
    // Automatically tracked
    await deploy();
  }));
```

---

## Performance Considerations

### Bundle Size

**Native addon:**
- Core package: ~50KB (TypeScript wrapper)
- Native binary: ~2-5MB per platform
- Total (with prebuilt): ~50KB + one platform binary

**Comparison:**
- `@sentry/node`: ~500KB (pure JS)
- `better-sqlite3`: ~2MB (native SQLite)
- **telemetry-kit**: ~2-5MB (includes SQLite + crypto)

**Optimization:**
- Lazy-load native addon (only when needed)
- Provide "lite" version without sync (smaller)

### Startup Time

**Native addon overhead:**
- Loading native module: ~1-5ms
- Rust initialization: ~1-2ms
- Total: ~2-7ms (negligible)

**Comparison:**
- Pure JS implementation: ~5-10ms (crypto libs, etc.)
- **Advantage:** Rust is faster despite being native

### Runtime Performance

**Benchmark targets (same as Rust SDK):**
- Event creation: <10ms
- Privacy sanitization: <1ms
- Background sync: No blocking

---

## Distribution Strategy

### npm Packages

```
@telemetry-kit/node         # Main package (with native bindings)
@telemetry-kit/express      # Express.js middleware
@telemetry-kit/next         # Next.js plugin
@telemetry-kit/cli          # CLI helpers (commander, yargs)
@telemetry-kit/types        # TypeScript types only
```

### Prebuilt Binaries

**Platforms to support:**
- Linux: x64, ARM64 (glibc and musl)
- macOS: x64, ARM64 (M1/M2)
- Windows: x64, ARM64

**Fallback:**
- If no prebuilt binary, compile from source
- Requires Rust toolchain (detected automatically)

---

## Migration Path

### From Existing Solutions

**From `@sentry/node`:**
```typescript
// Before
import * as Sentry from '@sentry/node';
Sentry.init({ dsn: '...' });

// After
import { TelemetryKit } from '@telemetry-kit/node';
const telemetry = TelemetryKit.builder()
  .serviceName('my-app')
  .withSyncCredentials(...)
  .build();
```

**From custom solutions:**
```typescript
// Before
await fetch('https://analytics.example.com', {
  method: 'POST',
  body: JSON.stringify({ event: 'deploy' })
});

// After
await telemetry.trackCommand('deploy');
```

---

## Ecosystem Compatibility

### Works With

**Runtimes:**
- ✅ Node.js 16+ (LTS)
- ✅ Node.js 18+ (current)
- ✅ Node.js 20+ (latest)
- ⚠️ Deno (via npm: specifier)
- ⚠️ Bun (if napi-rs compatible)

**Frameworks:**
- ✅ Express.js
- ✅ Fastify
- ✅ Next.js
- ✅ Nest.js
- ✅ Remix
- ✅ AWS Lambda
- ✅ Vercel Functions

**Environments:**
- ✅ Local development
- ✅ Docker containers
- ✅ Kubernetes
- ✅ Serverless (AWS, Vercel, Netlify)
- ✅ Edge runtimes (if WASM fallback)

---

## Testing Strategy

### Unit Tests (TypeScript)

```typescript
import { TelemetryKit } from '@telemetry-kit/node';

describe('TelemetryKit', () => {
  it('should track commands', async () => {
    const telemetry = TelemetryKit.builder()
      .serviceName('test')
      .build();

    await telemetry.trackCommand('test', { success: true });

    // Verify event was created
  });
});
```

### Integration Tests (Rust → Node)

```rust
#[napi]
#[cfg(test)]
mod tests {
    use super::*;

    #[napi]
    fn test_track_command() {
        // Test Rust side of bindings
    }
}
```

### Cross-Platform CI

```yaml
- name: Test Node.js bindings
  run: npm test
  strategy:
    matrix:
      os: [ubuntu-latest, macos-latest, windows-latest]
      node: [16, 18, 20]
```

---

## Challenges & Solutions

### Challenge 1: Async Rust → Node.js

**Problem:** Rust async (tokio) ≠ Node.js async (libuv)

**Solution:** napi-rs handles this with `tokio_runtime`
```rust
#[napi]
impl TelemetryKitNode {
    #[napi]
    pub async fn track_command(&self, name: String) -> Result<()> {
        // napi-rs automatically bridges tokio ↔ libuv
        self.inner.track_command(&name, |_| {}).await?;
        Ok(())
    }
}
```

### Challenge 2: Error Conversion

**Problem:** Rust `Result<T, E>` → JavaScript exceptions

**Solution:** napi-rs `Error` type
```rust
use napi::Error;

#[napi]
pub fn may_fail() -> Result<String> {
    Err(Error::from_reason("Something went wrong"))
}
```

```typescript
try {
  mayFail();
} catch (e) {
  console.error(e.message); // "Something went wrong"
}
```

### Challenge 3: SQLite in Native Addon

**Problem:** rusqlite + napi-rs can conflict

**Solution:** Already solved (we use rusqlite successfully)

### Challenge 4: Background Threads

**Problem:** Node.js single-threaded, Rust has threads

**Solution:** Use tokio tasks (green threads), not OS threads
```rust
// Good - tokio task
tokio::spawn(async move {
    // Background work
});

// Bad - OS thread (blocks Node.js)
std::thread::spawn(|| {
    // Will deadlock!
});
```

---

## Timeline & Resources

### Estimated Timeline

- **Phase 1 (PoC):** 2 weeks (1 developer)
- **Phase 2 (Production):** 4 weeks (1-2 developers)
- **Phase 3 (CI/CD):** 2 weeks (1 developer)
- **Phase 4 (Docs):** 2 weeks (1 developer)

**Total:** ~10-12 weeks for initial release

### Required Skills

- Rust (intermediate - bindings code)
- TypeScript (advanced - wrapper API)
- Node.js ecosystem (package publishing, CI/CD)
- napi-rs experience (can be learned)

---

## Success Metrics

### Adoption
- 1,000+ npm downloads/month (first 3 months)
- 10,000+ downloads/month (first year)
- 100+ GitHub stars on node package

### Performance
- <10ms event creation latency
- <5MB package size (per platform)
- <10ms startup overhead

### Quality
- 90%+ test coverage
- Zero critical issues (first 6 months)
- <5% error rate in telemetry collection

---

## Alternatives Considered

### Just Use HTTP API

**Approach:** Provide HTTP client library (no native code)

**Pros:**
- Simple implementation
- No native compilation

**Cons:**
- No offline support
- No SQLite buffering
- Network required for every event
- Higher latency

**Verdict:** Not recommended (defeats purpose of SDK)

---

## Conclusion

**Recommended Approach:** Native addon via napi-rs

**Why:**
1. **Maximum code reuse** - Single source of truth for security
2. **Best performance** - Native speed, proper async
3. **Consistent behavior** - Rust and Node.js SDKs identical
4. **Proven technology** - napi-rs used by major projects (swc, Prisma, etc.)

**Next Steps:**
1. Create RFC for community feedback
2. Prototype basic napi-rs bindings
3. Validate approach with simple example
4. Execute full implementation roadmap

---

**Last Updated:** 2025-01-23
**Status:** Proposal
**Target Release:** v0.6.0+ (Post-1.0.0)

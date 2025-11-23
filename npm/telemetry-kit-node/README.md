# @telemetry-kit/node

Privacy-first telemetry for Node.js applications.

This package provides native Node.js bindings to the [telemetry-kit](https://github.com/ibrahimcesar/telemetry-kit) Rust SDK, offering high performance and strong privacy guarantees.

## Features

- 🔒 **Privacy-first** - Anonymous by default, GDPR compliant
- 🚀 **High performance** - Native Rust implementation
- 📦 **Offline-first** - SQLite buffering with automatic sync
- 🎯 **Simple API** - Easy to integrate, hard to misuse
- 💪 **TypeScript** - Full type definitions included

## Installation

```bash
npm install @telemetry-kit/node
```

**Note:** This package includes native bindings. Prebuilt binaries are available for:
- Linux (x64, ARM64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x64, ARM64)

## Quick Start

```typescript
import { TelemetryKit } from '@telemetry-kit/node';

// Create telemetry instance
const telemetry = new TelemetryKit({
  serviceName: 'my-node-app',
  autoSync: true,
  syncConfig: {
    organizationId: 'your-org-id',
    applicationId: 'your-app-id',
    token: 'your-token',
    secret: 'your-secret'
  }
});

// Track commands
await telemetry.trackCommand('deploy', {
  success: true,
  durationMs: 1234
});

// Track features
await telemetry.trackFeature('dark-mode', {
  enabled: true
});

// Graceful shutdown
await telemetry.shutdown();
```

## API

### Constructor

```typescript
const telemetry = new TelemetryKit(config);
```

**Config:**
- `serviceName` (required) - Your application name
- `autoSync` - Enable automatic background sync (default: true)
- `syncInterval` - Sync interval in seconds (default: 60)
- `syncOnShutdown` - Sync before shutdown (default: true)
- `syncConfig` - Sync credentials (optional for offline-only)
- `privacyConfig` - Privacy settings
- `consentConfig` - Consent management

### Tracking Events

```typescript
// Track command execution
await telemetry.trackCommand('build', {
  success: true,
  durationMs: 5000
});

// Track feature usage
await telemetry.trackFeature('premium-feature', {
  enabled: true
});
```

### Manual Sync

```typescript
// Trigger sync manually
await telemetry.sync();
```

### Statistics

```typescript
const stats = await telemetry.stats();
console.log(`Total: ${stats.total}, Synced: ${stats.synced}`);
```

### Shutdown

```typescript
// Clean shutdown (performs final sync if configured)
await telemetry.shutdown();
```

## Builder API

For more control, use the builder pattern:

```typescript
import { TelemetryKitBuilder } from '@telemetry-kit/node';

const telemetry = TelemetryKitBuilder.new('my-app')
  .withSyncCredentials(orgId, appId, token, secret)
  .endpoint('https://telemetry.example.com')
  .autoSync(true)
  .syncInterval(120)
  .build();
```

## Privacy

telemetry-kit is **privacy-first by design**:

- ✅ **Anonymous by default** - User IDs are hashed
- ✅ **No PII collection** - Personal data never tracked
- ✅ **DO_NOT_TRACK** - Respects environment variable
- ✅ **Consent management** - Built-in opt-in/opt-out
- ✅ **Data minimization** - Only essential data collected
- ✅ **GDPR compliant** - Right to erasure, data portability

## Express.js Integration

```typescript
import express from 'express';
import { TelemetryKit } from '@telemetry-kit/node';

const app = express();
const telemetry = new TelemetryKit({
  serviceName: 'my-api'
});

// Track requests
app.use(async (req, res, next) => {
  const start = Date.now();

  res.on('finish', async () => {
    await telemetry.trackCommand('request', {
      success: res.statusCode < 400,
      durationMs: Date.now() - start
    });
  });

  next();
});
```

## TypeScript

Full TypeScript support included:

```typescript
import {
  TelemetryKit,
  CommandOptions,
  FeatureOptions,
  EventStats
} from '@telemetry-kit/node';

const options: CommandOptions = {
  success: true,
  durationMs: 1000
};

await telemetry.trackCommand('deploy', options);
```

## Self-Hosting

You can self-host the telemetry server:

```bash
# Clone the repository
git clone https://github.com/ibrahimcesar/telemetry-kit
cd telemetry-kit/server

# Start with Docker
docker compose up -d
```

See the [self-hosting guide](https://github.com/ibrahimcesar/telemetry-kit/blob/main/project-docs/DEPLOYMENT_GUIDE.md) for details.

## Performance

Built on Rust for maximum performance:

- Event creation: ~10ms
- Privacy sanitization: <1ms
- Background sync: Non-blocking
- Minimal overhead

## Platform Support

Prebuilt binaries available for:

| Platform | Architectures |
|----------|---------------|
| Linux    | x64, ARM64, musl |
| macOS    | Intel (x64), Apple Silicon (ARM64) |
| Windows  | x64, ARM64 |

If your platform isn't supported, the package will compile from source (requires Rust toolchain).

## License

Dual licensed under MIT OR Apache-2.0.

## Links

- [Documentation](https://github.com/ibrahimcesar/telemetry-kit)
- [Repository](https://github.com/ibrahimcesar/telemetry-kit)
- [Security Policy](https://github.com/ibrahimcesar/telemetry-kit/blob/main/SECURITY.md)
- [Roadmap](https://github.com/ibrahimcesar/telemetry-kit/blob/main/ROADMAP.md)

## Support

- 📧 Email: email@ibrahimcesar.com
- 💬 [Discussions](https://github.com/ibrahimcesar/telemetry-kit/discussions)
- 🐛 [Issues](https://github.com/ibrahimcesar/telemetry-kit/issues)

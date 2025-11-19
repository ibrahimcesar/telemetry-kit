# Telemetry Kit Server

Ingestion server for telemetry-kit events with HMAC authentication, rate limiting, and PostgreSQL storage.

## Features

- ✅ HMAC-SHA256 request signing verification
- ✅ Nonce-based replay attack prevention (Redis)
- ✅ Per-token rate limiting with tier support
- ✅ PostgreSQL event storage with efficient indexes
- ✅ Batch ingestion (1-1000 events)
- ✅ Partial success handling (207 Multi-Status)
- ✅ DO_NOT_TRACK header support
- ✅ Comprehensive error responses
- ✅ Health check endpoint

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Start all services (PostgreSQL, Redis, Server)
docker-compose up -d

# View logs
docker-compose logs -f server

# Stop services
docker-compose down
```

The server will be available at `http://localhost:3000`.

### Manual Setup

1. **Install Dependencies**

```bash
# Install PostgreSQL 16+
# Install Redis 7+

# Or use Docker:
docker run -d --name postgres -p 5432:5432 \
  -e POSTGRES_USER=telemetry \
  -e POSTGRES_PASSWORD=telemetry \
  -e POSTGRES_DB=telemetry_kit \
  postgres:16-alpine

docker run -d --name redis -p 6379:6379 redis:7-alpine
```

2. **Configure Environment**

```bash
cp .env.example .env
# Edit .env with your database URLs
```

3. **Run Server**

```bash
cargo run --release
```

## Configuration

Configuration via environment variables with `TK__` prefix:

```bash
# Server
TK__SERVER__HOST=0.0.0.0
TK__SERVER__PORT=3000
TK__SERVER__LOG_LEVEL=info

# Database
TK__DATABASE__URL=postgresql://user:pass@localhost:5432/telemetry_kit
TK__DATABASE__MAX_CONNECTIONS=10

# Redis
TK__REDIS__URL=redis://localhost:6379

# Rate Limits
TK__RATE_LIMIT__FREE_RPM=10
TK__RATE_LIMIT__PRO_RPM=100
TK__RATE_LIMIT__BUSINESS_RPM=1000
```

## API Endpoints

### Health Check

```bash
GET /health
```

Response:
```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

### Ingest Events

```bash
POST /v1/ingest/:org_id/:app_id
```

Headers:
- `Authorization: Bearer <token>`
- `Content-Type: application/json`
- `X-Signature: <hmac_signature>`
- `X-Timestamp: <unix_timestamp>`
- `X-Nonce: <uuid>`
- `X-Batch-Size: <count>`
- `X-SDK-Version: <sdk_name/version>`
- `X-Schema-Version: <schema_version>`

Body:
```json
{
  "events": [...]
}
```

Responses:
- `200 OK` - All events accepted
- `207 Multi-Status` - Partial success
- `400 Bad Request` - Invalid request
- `401 Unauthorized` - Invalid HMAC
- `403 Forbidden` - Timestamp/token mismatch
- `409 Conflict` - Duplicate nonce
- `429 Too Many Requests` - Rate limit exceeded

## Testing with Client SDK

```rust
use telemetry_kit::prelude::*;
use telemetry_kit::sync::SyncConfig;

#[tokio::main]
async fn main() -> telemetry_kit::Result<()> {
    let sync_config = SyncConfig::builder()
        .endpoint("http://localhost:3000") // Use local server
        .org_id("550e8400-e29b-41d4-a716-446655440000")?
        .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")?
        .token("tk_550e8400e29b41d4a716446655440000_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6")
        .secret("9f4b3c2a1e8d7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b6c5d4e3f2a1b0")
        .build()?;

    let telemetry = TelemetryKit::builder()
        .service_name("test-app")?
        .sync(sync_config)
        .build()?;

    // Track event
    telemetry.track_command("test", |e| e.success(true)).await?;

    // Sync to local server
    telemetry.sync().await?;

    Ok(())
}
```

## Database Schema

```sql
-- API Tokens
CREATE TABLE api_tokens (
    id UUID PRIMARY KEY,
    org_id UUID NOT NULL,
    app_id UUID NOT NULL,
    token VARCHAR(255) UNIQUE NOT NULL,
    secret VARCHAR(255) NOT NULL,
    tier token_tier NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ
);

-- Events
CREATE TABLE events (
    id BIGSERIAL PRIMARY KEY,
    event_id UUID UNIQUE NOT NULL,
    org_id UUID NOT NULL,
    app_id UUID NOT NULL,
    schema_version VARCHAR(20) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    service_name VARCHAR(255) NOT NULL,
    service_version VARCHAR(100) NOT NULL,
    service_language VARCHAR(50) NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    session_id VARCHAR(255),
    event_type VARCHAR(100) NOT NULL,
    event_data JSONB NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    -- ... more fields
);
```

## Rate Limiting

Rate limits are enforced per token per minute:

| Tier | Requests/Minute | Events/Month |
|------|-----------------|--------------|
| Free | 10 | 10,000 |
| Pro | 100 | 100,000 |
| Business | 1,000 | 1,000,000 |
| Enterprise | Unlimited | Unlimited |

Rate limit headers in responses:
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 87
X-RateLimit-Reset: 1732003260
```

## Security

- **HMAC-SHA256** signatures prevent request tampering
- **Nonce caching** prevents replay attacks (10-minute window)
- **Timestamp validation** (±10 minutes tolerance)
- **Constant-time comparison** prevents timing attacks

## Development

```bash
# Run tests
cargo test

# Run with hot reload
cargo watch -x 'run'

# Check code
cargo clippy
cargo fmt
```

## Production Deployment

1. Use strong database credentials
2. Enable SSL for PostgreSQL
3. Use Redis persistence
4. Set appropriate rate limits
5. Enable structured logging
6. Monitor with metrics endpoint (TODO)

## License

MIT OR Apache-2.0

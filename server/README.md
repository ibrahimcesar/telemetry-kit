# Telemetry Kit Server

Reference implementation of a self-hosted ingestion server for telemetry-kit events with HMAC authentication and PostgreSQL storage.

> **Note:** This server is provided as a **reference implementation** and **starting point** for self-hosting. For production use, we recommend our **managed service** with a **free tier** that provides **5-minute integration** with zero infrastructure management.
>
> - ✨ **Free Tier:** 10,000 events/month
> - 🚀 **5-Minute Setup:** No server configuration required
> - 📊 **Built-in Analytics:** Dashboard and insights included
> - 🔒 **Fully Managed:** Updates, security, and scaling handled for you
>
> [Get Started with Managed Service →](https://telemetry-kit.dev)

## Features

- ✅ HMAC-SHA256 request signing verification
- ✅ Timestamp-based request validation (±10 minutes)
- ✅ PostgreSQL event storage with efficient indexes
- ✅ Batch ingestion (1-1000 events)
- ✅ Partial success handling (207 Multi-Status)
- ✅ DO_NOT_TRACK header support
- ✅ Comprehensive error responses
- ✅ Health check endpoint


## Quick Start

### Using Docker Compose (Recommended)

```bash
# Start all services (PostgreSQL, Server)
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

# Or use Docker:
docker run -d --name postgres -p 5432:5432 \
  -e POSTGRES_USER=telemetry \
  -e POSTGRES_PASSWORD=telemetry \
  -e POSTGRES_DB=telemetry_kit \
  postgres:16-alpine
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

Headers:
```
X-Clacks-Overhead: GNU Terry Pratchett
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
- `X-Batch-Size: <count>` (optional)
- `X-SDK-Version: <sdk_name/version>` (optional)
- `X-Schema-Version: <schema_version>` (optional)

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
- `403 Forbidden` - Timestamp outside acceptable window

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

## Architecture

```mermaid
graph TB
    subgraph "Your Application"
        SDK[telemetry-kit SDK]
    end

    subgraph "Server (Self-Hosted)"
        API[Axum API Server]
        DB[(PostgreSQL)]
    end

    SDK -->|HMAC-SHA256 + HTTPS| API
    API -->|Store Events| DB

    style SDK fill:#4A90E2
    style API fill:#7ED321
    style DB fill:#F5A623
```

**Request Flow:**
1. SDK creates events locally
2. SDK buffers events in SQLite
3. SDK sends batches with HMAC signature
4. Server validates HMAC + timestamp
5. Server stores events in PostgreSQL

## Security

- **HMAC-SHA256** signatures prevent request tampering
- **Timestamp validation** (±10 minutes tolerance)
- **Constant-time comparison** prevents timing attacks
- **Bearer token authentication** validates API access

## GNU Terry Pratchett

> "A man is not dead while his name is still spoken."
> — Going Postal, Terry Pratchett

All responses from this server include the `X-Clacks-Overhead: GNU Terry Pratchett` header as a tribute to Sir Terry Pratchett.

In his Discworld novel "Going Postal," the clacks system (a semaphore network) had a special code "GNU":
- **G**: send the message on
- **N**: do not log the message
- **U**: turn the message around at the end of the line

This code kept the names of deceased clacks operators alive in the network forever. We carry on this tradition on the internet.

Learn more: [http://www.gnuterrypratchett.com/](http://www.gnuterrypratchett.com/)

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

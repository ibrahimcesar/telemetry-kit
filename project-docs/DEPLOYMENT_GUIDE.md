# Telemetry Kit - Deployment & Testing Guide

Complete guide for deploying and testing the telemetry-kit server locally.

## 🚀 Quick Start

### 1. Start the Server Stack

```bash
cd server
docker compose up -d
```

This starts:
- **PostgreSQL 16** on port 5432
- **Redis 7** on port 6379
- **Telemetry Server** on port 3000

### 2. Verify Services are Running

```bash
# Check service health
docker compose ps

# Check server logs
docker compose logs -f server

# Test health endpoint
curl http://localhost:3000/health
```

Expected response:
```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

### 3. Run End-to-End Test

```bash
cd ..
cargo run --example e2e_sync_test
```

This will:
1. Track 3 test events locally
2. Sync them to the server via HMAC-authenticated requests
3. Verify successful ingestion
4. Display statistics

## 📊 Verifying Events in Database

### Query Recent Events

```bash
docker compose -f server/docker-compose.yml exec postgres \
  psql -U telemetry telemetry_kit -c \
  "SELECT event_id, event_type, service_name, user_id, timestamp
   FROM events
   ORDER BY received_at DESC
   LIMIT 10;"
```

### Count Events by Type

```bash
docker compose -f server/docker-compose.yml exec postgres \
  psql -U telemetry telemetry_kit -c \
  "SELECT event_type, COUNT(*) as count
   FROM events
   GROUP BY event_type
   ORDER BY count DESC;"
```

### View Event Details

```bash
docker compose -f server/docker-compose.yml exec postgres \
  psql -U telemetry telemetry_kit -c \
  "SELECT event_id, schema_version, service_name, service_version,
          user_id, event_type, event_data, received_at
   FROM events
   ORDER BY received_at DESC
   LIMIT 5;"
```

## 🔐 Test Credentials

A test token is pre-seeded in the database for local testing:

```
Org ID:  550e8400-e29b-41d4-a716-446655440000
App ID:  7c9e6679-7425-40de-944b-e07fc1f90ae7
Token:   tk_550e8400e29b41d4a716446655440000_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6
Secret:  9f4b3c2a1e8d7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b6c5d4e3f2a1b0
Tier:    pro (100 requests/minute)
```

## 🧪 Testing the API

### Manual Event Ingestion

```bash
# Generate test data
ORG_ID="550e8400-e29b-41d4-a716-446655440000"
APP_ID="7c9e6679-7425-40de-944b-e07fc1f90ae7"
TOKEN="tk_550e8400e29b41d4a716446655440000_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6"
SECRET="9f4b3c2a1e8d7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b6c5d4e3f2a1b0"

TIMESTAMP=$(date +%s)
NONCE=$(uuidgen)
EVENT_ID=$(uuidgen)

BODY=$(cat <<EOF
{
  "events": [{
    "schema_version": "1.0.0",
    "event_id": "$EVENT_ID",
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ)",
    "service": {
      "name": "test-cli",
      "version": "1.0.0",
      "language": "bash",
      "language_version": "5.0"
    },
    "user_id": "client_test123",
    "session_id": "$(uuidgen)",
    "environment": {
      "os": "linux",
      "os_version": "6.5.0",
      "arch": "x86_64",
      "ci": false,
      "shell": "bash"
    },
    "event": {
      "type": "command",
      "category": "test",
      "data": {
        "command": "test",
        "success": true
      }
    },
    "metadata": {
      "sdk_version": "0.1.0",
      "transmission_timestamp": "$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ)",
      "batch_size": 1,
      "retry_count": 0
    }
  }]
}
EOF
)

# Generate HMAC signature
MESSAGE="${TIMESTAMP}:${NONCE}:${BODY}"
SIGNATURE=$(echo -n "$MESSAGE" | openssl dgst -sha256 -hmac "$SECRET" -hex | cut -d' ' -f2)

# Send request
curl -X POST "http://localhost:3000/v1/ingest/$ORG_ID/$APP_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -H "X-Signature: $SIGNATURE" \
  -H "X-Timestamp: $TIMESTAMP" \
  -H "X-Nonce: $NONCE" \
  -H "X-Batch-Size: 1" \
  -H "X-SDK-Version: test/1.0.0" \
  -H "X-Schema-Version: 1.0.0" \
  -d "$BODY" \
  -v
```

### Expected Response (200 OK)

```json
{
  "status": "success",
  "accepted": 1,
  "rejected": 0,
  "message": "All events ingested successfully"
}
```

### Test Rate Limiting

```bash
# Send 11 requests quickly (free tier limit is 10/minute)
for i in {1..11}; do
  curl -X POST "http://localhost:3000/v1/ingest/$ORG_ID/$APP_ID" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -H "X-Signature: $SIGNATURE" \
    -H "X-Timestamp: $TIMESTAMP" \
    -H "X-Nonce: $(uuidgen)" \
    -d "$BODY" \
    -w "\nRequest $i: %{http_code}\n"
done
```

Expected: First 10 requests succeed (200), 11th request fails (429).

### Test Replay Protection

```bash
# Send same request twice with same nonce
NONCE=$(uuidgen)
SIGNATURE=$(echo -n "${TIMESTAMP}:${NONCE}:${BODY}" | openssl dgst -sha256 -hmac "$SECRET" -hex | cut -d' ' -f2)

# First request should succeed
curl -X POST "http://localhost:3000/v1/ingest/$ORG_ID/$APP_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "X-Signature: $SIGNATURE" \
  -H "X-Timestamp: $TIMESTAMP" \
  -H "X-Nonce: $NONCE" \
  -d "$BODY"

# Second request should fail (409 Conflict)
curl -X POST "http://localhost:3000/v1/ingest/$ORG_ID/$APP_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "X-Signature: $SIGNATURE" \
  -H "X-Timestamp: $TIMESTAMP" \
  -H "X-Nonce: $NONCE" \
  -d "$BODY"
```

## 🔍 Monitoring

### View Server Logs

```bash
docker compose -f server/docker-compose.yml logs -f server
```

### Check Redis Nonce Cache

```bash
docker compose -f server/docker-compose.yml exec redis redis-cli
> KEYS nonce:*
> GET nonce:550e8400-e29b-41d4-a716-446655440000
> TTL nonce:550e8400-e29b-41d4-a716-446655440000
```

### Check Rate Limit Counters

```bash
docker compose -f server/docker-compose.yml exec redis redis-cli
> KEYS ratelimit:*
> GET ratelimit:550e8400-e29b-41d4-a716-446655440000:*
```

## 🛠️ Development

### Rebuild Server After Code Changes

```bash
cd server
docker compose up -d --build server
```

### Run Server Locally (without Docker)

```bash
# Start PostgreSQL and Redis
docker compose up -d postgres redis

# Set environment variables
export TK__DATABASE__URL="postgresql://telemetry:telemetry@localhost:5432/telemetry_kit"
export TK__REDIS__URL="redis://localhost:6379"
export TK__SERVER__HOST="0.0.0.0"
export TK__SERVER__PORT="3000"
export TK__SERVER__LOG_LEVEL="debug"

# Run server
cargo run --release
```

### Run Server Tests

```bash
cd server
cargo test --all-targets
```

## 🧹 Cleanup

### Stop Services

```bash
docker compose -f server/docker-compose.yml down
```

### Remove Data Volumes

```bash
docker compose -f server/docker-compose.yml down -v
```

## 📋 Troubleshooting

### Server won't start

1. Check if ports are already in use:
   ```bash
   lsof -i :3000  # Server
   lsof -i :5432  # PostgreSQL
   lsof -i :6379  # Redis
   ```

2. Check service logs:
   ```bash
   docker compose logs postgres
   docker compose logs redis
   docker compose logs server
   ```

### Events not appearing in database

1. Check server logs for errors:
   ```bash
   docker compose logs -f server
   ```

2. Verify HMAC signature is correct:
   - Ensure timestamp is current (within ±10 minutes)
   - Ensure nonce is unique (UUID v4)
   - Ensure message format is: `timestamp:nonce:body`

3. Check authentication:
   ```bash
   docker compose exec postgres \
     psql -U telemetry telemetry_kit -c \
     "SELECT * FROM api_tokens WHERE token LIKE 'tk_%';"
   ```

### Rate limit issues

1. Check current rate limit counters:
   ```bash
   docker compose exec redis redis-cli KEYS "ratelimit:*"
   ```

2. Clear rate limit for testing:
   ```bash
   docker compose exec redis redis-cli FLUSHDB
   ```

### Database connection issues

1. Verify PostgreSQL is running:
   ```bash
   docker compose exec postgres pg_isready -U telemetry
   ```

2. Test connection:
   ```bash
   docker compose exec postgres \
     psql -U telemetry telemetry_kit -c "SELECT version();"
   ```

## 🎯 Next Steps

1. **Production Deployment**: See [server/README.md](server/README.md) for production configuration
2. **Custom Tokens**: Generate new API tokens with appropriate tiers
3. **Monitoring**: Set up Prometheus/Grafana for metrics
4. **Scaling**: Deploy with load balancer and multiple server instances

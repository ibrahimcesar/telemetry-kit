# Synchronization Strategy

## Overview

This document outlines the strategy for keeping two repositories in sync:

1. **telemetry-kit** (Public) - SDK library, self-hosting server, CLI tool
2. **telemetry-kit.dev** (Private) - Hosted service with hexagonal architecture

Both repos must maintain protocol compatibility while serving different purposes.

---

## 🏗️ Architecture Split

### Public Repo: `telemetry-kit`
**Purpose:** Open-source SDK and basic self-hosting
**Location:** `~/Dev/telemetry-kit`
**GitHub:** `github.com/ibrahimcesar/telemetry-kit`

**Contains:**
- Rust SDK library (event tracking, storage, sync client)
- Basic ingestion server (PostgreSQL + Redis)
- CLI tool
- Examples and documentation
- Self-hosting guides

**Audience:**
- Library users (integrate telemetry into their apps)
- Self-hosters (run their own infrastructure)
- Contributors (open source community)

### Private Repo: `telemetry-kit.dev`
**Purpose:** Managed cloud service (SaaS)
**Location:** `~/Dev/telemetry-kit.dev`
**GitHub:** `github.com/ibrahimcesar/telemetry-kit.dev` (private)
**Domain:** `telemetry-kit.dev` ✅

**Contains:**
- Hexagonal architecture (domain, ports, adapters)
- Multi-tenant infrastructure
- Web dashboard (Next.js)
- Billing integration (Stripe)
- Advanced analytics
- Cloud adapters (AWS, GCP)
- User authentication (GitHub OAuth, email)

**Audience:**
- Paying customers (hosted service users)
- Enterprise clients
- Internal team

---

## 🔄 What Must Stay in Sync

### 1. **Event Schema** ⚠️ CRITICAL

Both repos must use identical event schema versions.

**Current Schema:** v1.0.0

**Schema Location:**
- Public: `telemetry-kit/src/event.rs` (EventSchema struct)
- Private: `telemetry-kit.dev/crates/domain/src/entities/event.rs`

**Fields to Keep Identical:**
```rust
pub struct EventSchema {
    pub schema_version: String,        // "1.0.0"
    pub event_id: String,              // uuid
    pub event_type: EventType,         // Command, Feature, Custom
    pub timestamp: DateTime<Utc>,
    pub user_id: String,               // client_<hash>
    pub session_id: String,            // sess_<uuid>
    pub service_name: String,
    pub service_version: String,
    pub event_data: EventData,         // Command/Feature/Custom specific
    pub environment: Environment,      // os, arch, ci, shell
    pub metadata: Metadata,            // sdk version, transmission timestamp
}
```

**Sync Process:**
1. Make schema changes in `telemetry-kit` first (SDK is source of truth)
2. Test with public server implementation
3. Copy schema to `telemetry-kit.dev/crates/domain`
4. Update both CHANGELOG files with schema version bump
5. Coordinate release (SDK and server must support same schema)

### 2. **Sync Protocol** ⚠️ CRITICAL

The HMAC-SHA256 authentication protocol must be identical.

**Protocol Location:**
- Public: `telemetry-kit/src/sync/auth.rs`
- Private: `telemetry-kit.dev/crates/application/src/services/auth.rs`

**Components:**
- HMAC signature format: `HMAC-SHA256({timestamp}:{nonce}:{body})`
- Request headers: `X-Signature`, `X-Timestamp`, `X-Nonce`
- Timestamp validation: ±10 minutes
- Nonce replay protection: 10-minute window

**Sync Process:**
1. Protocol changes must be discussed and planned
2. Use feature flags for breaking changes
3. Support old and new protocol for transition period
4. Update SDK and both servers together
5. Announce deprecation timeline

### 3. **API Endpoints** ⚠️ IMPORTANT

Both servers must support the same ingestion endpoints.

**Endpoints:**
```
POST /v1/ingest/{org_id}/{app_id}
GET  /health
```

**Response Formats:**
```rust
// Success
200 OK: {"accepted": 100, "rejected": 0}

// Partial success
207 Multi-Status: {
  "accepted": 98,
  "rejected": 2,
  "errors": [{"event_id": "...", "reason": "..."}]
}

// Errors
401 Unauthorized: {"error": "Invalid signature"}
403 Forbidden: {"error": "Invalid token"}
429 Too Many Requests: {"error": "Rate limit exceeded", "retry_after": 60}
```

**Sync Process:**
1. Document endpoint changes in `docs/API.md` (both repos)
2. Use API versioning (`/v1/`, `/v2/`) for breaking changes
3. Test SDK against both servers
4. Update integration tests

### 4. **Rate Limiting Tiers** 🔄 ALIGN

Ensure tier names and limits are consistent.

**Tiers:**
- Free: 10/min (public server) → 10,000 events/month (cloud)
- Pro: 100/min → 100,000 events/month ($29/mo)
- Business: 1000/min → 1M events/month ($99/mo)
- Enterprise: Unlimited → Custom pricing

**Note:** Public server uses per-minute limits, cloud uses monthly quotas.

### 5. **Error Codes** 🔄 ALIGN

Use consistent error codes and messages.

**Error Location:**
- Public: `telemetry-kit/src/error.rs`
- Private: `telemetry-kit.dev/crates/domain/src/errors.rs`

**Common Errors:**
- `ERR_INVALID_SIGNATURE` - HMAC validation failed
- `ERR_EXPIRED_TIMESTAMP` - Timestamp outside window
- `ERR_REPLAY_ATTACK` - Nonce already used
- `ERR_RATE_LIMIT` - Too many requests
- `ERR_INVALID_TOKEN` - Token not found/expired
- `ERR_INVALID_SCHEMA` - Event schema version mismatch

---

## 🚫 What Can Diverge

### Public Repo Can Have:
- Simpler server implementation (no multi-tenancy)
- Basic dashboard (optional, HTML only)
- SQLite storage option
- Docker Compose only
- Flat file configuration
- No billing/authentication beyond API tokens

### Private Repo Can Have:
- Hexagonal architecture (overkill for public)
- Multiple cloud adapters (AWS, GCP, Azure)
- Advanced dashboard (Next.js, React)
- User management and OAuth
- Billing integration (Stripe)
- Email notifications
- Observer invites
- Public dashboard sharing
- Badge API
- Advanced analytics
- Parquet storage
- Multi-region support

---

## 📋 Sync Checklist

Use this checklist when making changes:

### Schema Changes
- [ ] Update `telemetry-kit/src/event.rs`
- [ ] Update `telemetry-kit.dev/crates/domain/src/entities/event.rs`
- [ ] Bump schema version in both
- [ ] Update tests in both repos
- [ ] Update CHANGELOG in both
- [ ] Verify SDK works with both servers

### Protocol Changes
- [ ] Document in `SYNC_PROTOCOL.md` (both repos)
- [ ] Update SDK client (`telemetry-kit/src/sync/`)
- [ ] Update public server handlers
- [ ] Update private server handlers
- [ ] Add integration tests
- [ ] Plan migration strategy if breaking

### API Changes
- [ ] Update `docs/API.md` (both repos)
- [ ] Update public server (`telemetry-kit/server/`)
- [ ] Update private server (`telemetry-kit.dev/crates/api/`)
- [ ] Update SDK client if needed
- [ ] Update examples
- [ ] Test E2E flow

### Error Code Changes
- [ ] Add to `telemetry-kit/src/error.rs`
- [ ] Add to `telemetry-kit.dev/crates/domain/src/errors.rs`
- [ ] Update error documentation
- [ ] Update SDK to handle new errors
- [ ] Update tests

---

## 🔧 Development Workflow

### Making Changes to Shared Components

1. **Start in Public Repo (`telemetry-kit`)**
   ```bash
   cd ~/Dev/telemetry-kit
   git checkout -b feature/your-feature

   # Make changes to event.rs, sync protocol, etc.
   # Test with public server
   cargo test
   cd server && docker compose up -d
   cargo run --example e2e_sync_test
   ```

2. **Copy to Private Repo (`telemetry-kit.dev`)**
   ```bash
   cd ~/Dev/telemetry-kit.dev
   git checkout -b feature/your-feature

   # Copy schema changes
   # Adapt to hexagonal architecture
   # Test with cloud server
   cargo test
   cargo run --bin api
   ```

3. **Test Cross-Compatibility**
   ```bash
   # SDK (public) → Public server
   cd ~/Dev/telemetry-kit
   cargo run --example e2e_sync_test

   # SDK (public) → Private server
   # Point SDK to private server endpoint
   # Verify events are received
   ```

4. **Merge and Tag**
   ```bash
   # Public repo
   cd ~/Dev/telemetry-kit
   git merge feature/your-feature
   git tag v0.2.0
   git push origin main --tags

   # Private repo
   cd ~/Dev/telemetry-kit.dev
   git merge feature/your-feature
   git tag v0.2.0
   git push origin main --tags
   ```

---

## 📦 Release Coordination

### Version Numbers

Both repos should maintain similar major.minor versions:
- `telemetry-kit` v1.0.0 ↔ `telemetry-kit.dev` v1.0.x
- Patch versions can diverge (bug fixes)
- Major/minor versions should align (breaking changes)

### Release Process

1. **Prepare Public Release**
   ```bash
   cd ~/Dev/telemetry-kit
   # Update Cargo.toml version
   # Update CHANGELOG.md
   # Tag release
   git tag v1.0.0
   cargo publish
   ```

2. **Prepare Private Release**
   ```bash
   cd ~/Dev/telemetry-kit.dev
   # Update Cargo.toml version
   # Update CHANGELOG.md
   # Tag release
   git tag v1.0.0
   # Deploy to telemetry-kit.dev
   ```

3. **Announce**
   - Blog post mentions both SDK and hosted service
   - Documentation links to telemetry-kit.dev
   - README shows both self-hosting and cloud options

---

## 🔍 Testing Cross-Compatibility

### Integration Test Matrix

| SDK Version | Public Server | Private Server | Status |
|-------------|---------------|----------------|--------|
| v0.2.0      | v0.2.0        | v0.2.0         | ✅      |
| v0.2.0      | v0.1.0        | v0.2.0         | ⚠️      |
| v0.3.0      | v0.2.0        | v0.3.0         | ❌      |

### Test Scenarios

1. **Schema Compatibility**
   ```bash
   # Old SDK → New server
   # New SDK → Old server
   # Ensure graceful degradation
   ```

2. **Protocol Compatibility**
   ```bash
   # Test HMAC signature verification
   # Test timestamp validation
   # Test nonce replay protection
   ```

3. **Rate Limiting**
   ```bash
   # Verify SDK respects 429 responses
   # Verify exponential backoff works
   # Test across both servers
   ```

---

## 📚 Documentation Requirements

### Public Repo (`telemetry-kit`)
- `README.md` - Mention both self-hosting and cloud
- `SYNC_PROTOCOL.md` - Document protocol in detail
- `docs/self-hosting.md` - Self-hosting guide
- `docs/cloud.md` - Link to telemetry-kit.dev

### Private Repo (`telemetry-kit.dev`)
- `README.md` - Focus on hosted service
- `docs/SYNC_PROTOCOL.md` - Same as public (copy)
- `docs/ALIGNMENT_ANALYSIS.md` - Keep updated
- `docs/SDK_INTEGRATION.md` - How to use SDK with cloud

### Shared Documents (Keep in Sync)
- `SYNC_PROTOCOL.md` - Copy between repos
- `API.md` - Endpoint documentation
- Schema version history

---

## 🎯 Key Principles

1. **SDK is Source of Truth** - Schema and protocol changes start in public repo
2. **Test Cross-Repo** - Always verify SDK works with both servers
3. **Version Coordination** - Major/minor versions should align
4. **Document Everything** - Protocol changes must be documented in both repos
5. **Backward Compatibility** - Support old clients during transitions
6. **Clear Communication** - CHANGELOG entries in both repos

---

## 🚨 Breaking Change Process

When making breaking changes:

1. **Announce in Advance** (1 month notice)
   - GitHub Discussions post
   - CHANGELOG entry
   - Email to hosted service users

2. **Support Transition Period** (3 months)
   - Old and new protocols both work
   - SDK supports both with feature flags
   - Server logs warnings for old clients

3. **Deprecate Gracefully**
   - Remove old protocol support
   - Bump major version
   - Update documentation

---

## 📧 Communication Channels

### For Schema/Protocol Changes
- GitHub Issues in both repos (link them)
- Private Slack/Discord for cloud planning
- RFC documents for major changes

### For Coordination
- Tag releases on same day when possible
- Coordinate blog posts and announcements
- Share test results between teams

---

## 🔐 Security Considerations

### Shared Security Requirements
- HMAC-SHA256 authentication
- Constant-time comparison for signatures
- Nonce-based replay protection
- Rate limiting

### Report Security Issues
- Public repo: SECURITY.md → email@ibrahimcesar.com
- Private repo: Direct team communication

---

## 📊 Success Metrics

Track alignment health:

- [ ] Schema version matches between repos
- [ ] Protocol version matches
- [ ] SDK passes E2E tests against both servers
- [ ] No breaking changes without migration path
- [ ] Documentation is in sync
- [ ] Release versions are coordinated

---

**Last Updated:** 2024-11-20
**Next Review:** Before v1.0.0 launch

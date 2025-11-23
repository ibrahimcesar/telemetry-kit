# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added (Phase 2 - In Progress)
- **Auto-Sync Background Task**
  - Background tokio task for automatic event synchronization
  - Configurable sync interval (default: 60 seconds)
  - Graceful shutdown with optional final sync
  - Thread-safe implementation using Arc and Mutex
  - Builder API methods: `.auto_sync()`, `.sync_interval()`, `.sync_on_shutdown()`
  - New example: `examples/auto_sync.rs`

- **CLI Tool** (`telemetry-kit`)
  - `init` - Interactive project setup with credential configuration
  - `test` - Validate sync credentials and test connectivity
  - `stats` - View event statistics (total, synced, unsynced)
  - `sync` - Manually trigger event synchronization (placeholder)
  - `validate` - Validate telemetry configuration
  - `clean` - Clear local event database with confirmation
  - Interactive prompts with `dialoguer`
  - Progress bars and spinners with `indicatif`
  - Colored output for better UX
  - Global `--service` flag to operate on specific services

### Planned for v1.0.0
- Privacy controls (consent flow, data sanitization)
- Query API endpoints
- Advanced analytics dashboard
- Full production deployment guides

## [0.2.0-alpha.1] - 2024-11-20

### Added
- **Core SDK Implementation** (2,189 lines of Rust)
  - Event tracking system with fluent builders (Command, Feature, Custom events)
  - SQLite-based local storage with proper schema and indexes
  - Anonymous user identification (SHA-256 hashed machine IDs)
  - Session tracking with unique session IDs
  - Environment detection (OS, architecture, CI, shell type)

- **Sync Protocol** (HMAC-SHA256 Authentication)
  - HTTP sync client with exponential backoff retry
  - HMAC-SHA256 request signing and verification
  - Timestamp validation (±10 minutes) for replay protection
  - Nonce-based duplicate request prevention
  - Multi-status response handling (200, 207, 401, 403, 429)
  - DNT (Do Not Track) header support

- **Production Server** (1,209 lines)
  - Axum-based ingestion API with PostgreSQL + Redis
  - Multi-tier rate limiting (Free/Pro/Business/Enterprise)
  - Batch ingestion (1-1000 events per request)
  - Replay protection with nonce caching
  - Partial success handling (207 Multi-Status responses)
  - Docker Compose deployment stack
  - Database migrations with proper indexes

- **Infrastructure**
  - CI/CD pipeline with GitHub Actions
  - Security scanning (cargo-audit, cargo-deny)
  - Code coverage reporting (cargo-tarpaulin)
  - Automated formatting and linting
  - Dependabot configuration
  - SECURITY.md with vulnerability disclosure policy

- **Documentation**
  - SYNC_STRATEGY.md - Repository synchronization guide
  - PRODUCTION_PLAN.md - 10-week roadmap to v1.0.0
  - DEPLOYMENT_GUIDE.md - Self-hosting instructions
  - SYNC_SETUP.md - Sync configuration guide
  - Comprehensive API documentation

- **Testing**
  - 40+ unit tests across modules
  - Integration test suite
  - 4 working examples (basic, local_only, sync, e2e_sync_test)
  - Test coverage for event tracking, storage, sync protocol

### Changed
- Version bumped from 0.0.1 (placeholder) to 0.2.0-alpha.1 (functional alpha)
- README updated with accurate "Currently Working" vs "Planned" sections
- Event schema stabilized at v1.0.0

### Fixed
- CI/CD pipeline moved to correct location (.github/workflows/)
- Build configuration optimized for development and release
- Proper error handling across all modules

### Security
- HMAC constant-time comparison to prevent timing attacks
- SQL injection protection via parameterized queries
- Input validation on all user-provided data
- Rate limiting to prevent abuse
- Anonymous user IDs (no PII collection)

### Known Limitations
- CLI tool is a stub (not functional)
- Privacy controls not configurable yet
- Dashboard not included

### Breaking Changes
- This is the first functional release, so no breaking changes from previous versions
- Event schema v1.0.0 is now the baseline for future compatibility

### Notes
This is the first alpha release with working functionality. The SDK and server can now:
- Track events locally in SQLite
- Sync events to a self-hosted server
- Handle authentication via HMAC-SHA256
- Enforce rate limits
- Prevent replay attacks

**Not yet suitable for production use.** Missing features:
- Auto-sync background task
- CLI tool
- Privacy configuration
- Web dashboard
- Cloud hosting

See [PRODUCTION_PLAN.md](PRODUCTION_PLAN.md) for the roadmap to v1.0.0.

## [0.0.1] - 2024-11-18

### Added
- Initial crate reservation on crates.io
- Project structure and documentation
- README with project vision and roadmap
- Minimal placeholder implementation
- MIT/Apache-2.0 dual licensing

### Notes
This is a placeholder release to reserve the crate name. The actual implementation
is under active development. See the [roadmap](README.md#-roadmap) for planned features.

[Unreleased]: https://github.com/ibrahimcesar/telemetry-kit/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/ibrahimcesar/telemetry-kit/releases/tag/v0.0.1

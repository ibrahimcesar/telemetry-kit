# telemetry-kit Project Documentation

This directory contains internal project documentation, development notes, and milestone records.

---

## 📋 Quick Navigation

### Core Documentation
- **[PRODUCTION_PLAN.md](PRODUCTION_PLAN.md)** - Original 8-week development plan (historical)
- **[CURRENT_STATUS.md](CURRENT_STATUS.md)** - Current project status snapshot
- **[../ROADMAP.md](../ROADMAP.md)** - Community-facing roadmap (at root)

### Weekly Milestones
- **[WEEK_5_COMPLETE.md](WEEK_5_COMPLETE.md)** - Auto-sync & CLI completion
- **[WEEK_7_COMPLETE.md](WEEK_7_COMPLETE.md)** - Testing & quality completion
- **Week 8** - Security hardening (in progress)

### Phase Completions
- **[PHASE_1_COMPLETE.md](PHASE_1_COMPLETE.md)** - Core SDK features
- **[PHASE_2_AUTO_SYNC.md](PHASE_2_AUTO_SYNC.md)** - Auto-sync implementation
- **[PHASE_2_CLI.md](PHASE_2_CLI.md)** - CLI implementation

---

## 🔧 Technical Documentation

### Performance & Testing
- **[BENCHMARKS.md](BENCHMARKS.md)** - Performance benchmark results and analysis
- **[TESTING_COMPLETE.md](TESTING_COMPLETE.md)** - Test suite completion summary
- **[PROPERTY_TESTS.md](PROPERTY_TESTS.md)** - Property-based testing documentation

### Error Handling
- **[ERROR_MESSAGES.md](ERROR_MESSAGES.md)** - Error message improvements and guidelines

### Features
- **[CLI.md](CLI.md)** - CLI tool documentation
- **[CLI_CONSENT.md](CLI_CONSENT.md)** - Consent flow in CLI
- **[PRIVACY_IMPLEMENTATION.md](PRIVACY_IMPLEMENTATION.md)** - Privacy features implementation
- **[EVENT_SCHEMA.md](EVENT_SCHEMA.md)** - Event data schema

### Sync Strategy
- **[SYNC_STRATEGY.md](SYNC_STRATEGY.md)** - Synchronization architecture
- **[SYNC_SETUP.md](SYNC_SETUP.md)** - Sync setup guide

---

## 🚀 Deployment & Operations

- **[DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)** - Deployment instructions and best practices
- **[MAKEFILE_SUMMARY.md](MAKEFILE_SUMMARY.md)** - Makefile command reference
- **[Makefile.md](Makefile.md)** - Detailed Makefile documentation

---

## 🎨 Design & Branding

- **[LOGO_UPDATES.md](LOGO_UPDATES.md)** - Logo design evolution and updates
- **[CLACKS.md](CLACKS.md)** - GNU Terry Pratchett tribute

---

## 📚 Documentation Site

- **[DOCS_SETUP.md](DOCS_SETUP.md)** - Fumadocs setup and configuration

---

## 🔒 Security

- **[UNMAINTAINED_DEPS.md](UNMAINTAINED_DEPS.md)** - Migration plan for unmaintained dependencies

For the main security documentation, see:
- **[../SECURITY.md](../SECURITY.md)** - Vulnerability disclosure policy (kept at root)
- **[../SLSA.md](../SLSA.md)** - Supply chain security compliance (kept at root)

---

## 📂 File Organization

### Files Kept at Root

These files remain at the project root for visibility and standard conventions:

1. **README.md** - Project overview and getting started
2. **CHANGELOG.md** - Version history
3. **CONTRIBUTING.md** - Contribution guidelines
4. **SECURITY.md** - Security policy (GitHub standard)
5. **SLSA.md** - Supply chain security
6. **ROADMAP.md** - Community-facing roadmap (public)

### Files in project-docs/

All internal development documentation, milestone records, technical notes, and historical documents are organized here.

---

## 🗂️ Document Categories

### 1. Milestone Records
Track completion of major development phases:
- PHASE_*_COMPLETE.md
- WEEK_*_COMPLETE.md

### 2. Technical Implementation
Detailed documentation of features and architecture:
- BENCHMARKS.md, TESTING_COMPLETE.md, PROPERTY_TESTS.md
- ERROR_MESSAGES.md
- PRIVACY_IMPLEMENTATION.md, EVENT_SCHEMA.md
- SYNC_STRATEGY.md, SYNC_SETUP.md

### 3. Tooling & Operations
Development and deployment tools:
- Makefile.md, MAKEFILE_SUMMARY.md
- DEPLOYMENT_GUIDE.md
- DOCS_SETUP.md

### 4. Feature Documentation
Individual feature deep-dives:
- CLI.md, CLI_CONSENT.md
- LOGO_UPDATES.md, CLACKS.md

### 5. Security & Dependencies
Security audits and dependency management:
- UNMAINTAINED_DEPS.md
- (SECURITY.md and SLSA.md kept at root)

---

## 📖 For Users vs Developers

### For End Users
Start here:
1. **[../README.md](../README.md)** - Getting started
2. **[../docs/](../docs/)** - User-facing documentation site

### For Contributors
Start here:
1. **[../CONTRIBUTING.md](../CONTRIBUTING.md)** - How to contribute
2. **[../PRODUCTION_PLAN.md](../PRODUCTION_PLAN.md)** - Development roadmap
3. **This directory** - Technical implementation details

### For Security Researchers
Start here:
1. **[../SECURITY.md](../SECURITY.md)** - Vulnerability disclosure
2. **[../SLSA.md](../SLSA.md)** - Supply chain security
3. **[UNMAINTAINED_DEPS.md](UNMAINTAINED_DEPS.md)** - Dependency audit results

---

## 🔄 Maintenance

This directory should be periodically reviewed and updated:

- **Weekly:** Update CURRENT_STATUS.md during active development
- **End of Week:** Create WEEK_*_COMPLETE.md milestone documents
- **End of Phase:** Create PHASE_*_COMPLETE.md summaries
- **Quarterly:** Archive old milestone documents if needed

---

## 📊 Quick Stats

**Project Start:** 2024
**Current Phase:** Week 8 - Security Hardening
**Total Milestones:** 7+ documented phases
**Documentation Files:** 23 files in this directory

---

**Last Updated:** 2025-01-23
**Maintained By:** telemetry-kit core team

# SLSA Compliance for telemetry-kit

**Status:** SLSA Level 2
**Target:** SLSA Level 3
**Last Updated:** 2025-01-22

---

## Table of Contents

- [What is SLSA?](#what-is-slsa)
- [Current Compliance](#current-compliance)
- [SLSA Level 2 (Current)](#slsa-level-2-current)
- [SLSA Level 3 (Planned)](#slsa-level-3-planned)
- [Build Provenance](#build-provenance)
- [Verification](#verification)
- [Supply Chain Threats](#supply-chain-threats)
- [Roadmap](#roadmap)

---

## What is SLSA?

SLSA (Supply-chain Levels for Software Artifacts) is a security framework to protect software supply chains from tampering, unauthorized changes, and malicious code injection.

**Goals:**
- Prevent unauthorized modifications to source code
- Ensure integrity of build process
- Provide verifiable provenance for artifacts
- Enable consumers to verify artifact authenticity

**Levels:**
- **Level 1**: Documentation of build process
- **Level 2**: Tamper-resistant build service
- **Level 3**: Hardened build platforms
- **Level 4**: Highest level of auditability (two-party review)

---

## Current Compliance

### SLSA Level 2 ✅

| Requirement | Status | Implementation |
|-------------|--------|----------------|
| Version controlled source | ✅ | GitHub repository |
| Generated provenance | ✅ | GitHub Actions attestations |
| Build service | ✅ | GitHub Actions |
| Automated build | ✅ | CI/CD pipeline |
| Provenance available | ✅ | Signed attestations |

### SLSA Level 3 (In Progress)

| Requirement | Status | Implementation |
|-------------|--------|----------------|
| Hardened build platform | 🔄 | GitHub Actions (trusted) |
| Non-falsifiable provenance | 🔄 | Sigstore integration planned |
| Isolated builds | ✅ | Ephemeral GitHub runners |
| Parameterless builds | 🔄 | Working towards |
| Hermetic builds | ❌ | Planned for v2.0 |

---

## SLSA Level 2 (Current)

### Requirements Met

#### 1. Version Controlled Source ✅

**Requirement:** Source code is version controlled with history.

**Implementation:**
- Git repository: https://github.com/ibrahimcesar/telemetry-kit
- All changes tracked in version control
- Signed commits encouraged (not required)
- Branch protection on `main`:
  - Pull request required
  - Status checks must pass
  - No direct pushes

**Verification:**
```bash
# Clone repository
git clone https://github.com/ibrahimcesar/telemetry-kit
cd telemetry-kit

# Verify history
git log --oneline

# Check for signed commits
git log --show-signature
```

#### 2. Generated Provenance ✅

**Requirement:** Provenance is automatically generated for each release.

**Implementation:**
- GitHub Actions workflows generate SLSA provenance
- Provenance includes:
  - Source commit SHA
  - Build parameters
  - Builder identity
  - Build timestamp
- Stored as GitHub attestations

**Verification:**
```bash
# Download release
gh release download v0.2.0

# Verify attestation (requires gh CLI v2.40+)
gh attestation verify telemetry-kit-*.tar.gz \
  --owner ibrahimcesar
```

#### 3. Build Service ✅

**Requirement:** Builds are performed by a hosted build service.

**Implementation:**
- **Service:** GitHub Actions
- **Runners:** GitHub-hosted (Ubuntu latest)
- **Isolation:** Each build runs in fresh VM
- **Logging:** Full build logs available
- **Reproducibility:** Same inputs → same outputs

**Build Configuration:** [.github/workflows/release.yml](.github/workflows/release.yml)

#### 4. Build as Code ✅

**Requirement:** Build process is defined as code in version control.

**Implementation:**
```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      contents: write
      id-token: write  # For SLSA provenance
      attestations: write

    steps:
      - uses: actions/checkout@v4

      - name: Build
        run: cargo build --release

      - name: Generate provenance
        uses: actions/attest-build-provenance@v1
        with:
          subject-path: 'target/release/telemetry-kit'
```

---

## SLSA Level 3 (Planned)

### Additional Requirements

#### 1. Hardened Build Platform 🔄

**Requirement:** Build platform prevents tampering by build process.

**Current State:**
- GitHub Actions provides isolation ✅
- Ephemeral runners prevent persistence ✅
- No access to secrets from forks ✅

**Planned Improvements:**
- Custom runners with enhanced security
- Restricted network access during builds
- Immutable build containers

#### 2. Non-falsifiable Provenance 🔄

**Requirement:** Provenance cannot be forged by build process itself.

**Current State:**
- GitHub Actions signs provenance ✅
- Attestations stored externally ✅

**Planned Improvements:**
- **Sigstore integration:** Sign with Rekor transparency log
- **Cosign signatures:** Verify artifacts with cosign
- **Public transparency:** All signatures publicly verifiable

**Example (planned):**
```bash
# Sign with cosign
cosign sign-blob \
  --bundle telemetry-kit.bundle \
  target/release/telemetry-kit

# Verify signature
cosign verify-blob \
  --bundle telemetry-kit.bundle \
  --certificate-identity=https://github.com/ibrahimcesar/telemetry-kit/.github/workflows/release.yml@refs/heads/main \
  --certificate-oidc-issuer=https://token.actions.githubusercontent.com \
  target/release/telemetry-kit
```

#### 3. Isolated Builds ✅

**Requirement:** Builds cannot access secrets or network.

**Current State:**
- Each build runs in fresh VM ✅
- Secrets not available to build steps ✅
- Network access restricted (partially)

**Verification:**
```yaml
# Build step has no secret access
- name: Build
  run: cargo build --release
  # No 'env:' with secrets
  # No 'with: secrets'
```

#### 4. Parameterless Builds 🔄

**Requirement:** Build doesn't accept user-controlled parameters.

**Current State:**
- Tag triggers build ✅
- No manual workflow_dispatch ✅
- Build parameters from git only ✅

**Improvement Needed:**
- Remove version from Cargo.toml
- Derive version from git tag
- Zero external parameters

#### 5. Hermetic Builds ❌

**Requirement:** All dependencies declared and fetched from known sources.

**Planned for v2.0:**
- Vendor all dependencies
- Cargo.lock pinning (already done ✅)
- Offline build capability
- Reproducible builds

---

## Build Provenance

### What is Provenance?

Provenance is metadata about how an artifact was built:

- **What:** Source repository and commit
- **When:** Build timestamp
- **Where:** Build platform (GitHub Actions)
- **Who:** Builder identity (GitHub)
- **How:** Build recipe (.github/workflows/release.yml)

### Provenance Format

We use the in-toto SLSA Provenance format (v1.0):

```json
{
  "_type": "https://in-toto.io/Statement/v1",
  "subject": [
    {
      "name": "telemetry-kit",
      "digest": {
        "sha256": "abc123..."
      }
    }
  ],
  "predicateType": "https://slsa.dev/provenance/v1",
  "predicate": {
    "buildDefinition": {
      "buildType": "https://github.com/actions/...",
      "externalParameters": {
        "workflow": {
          "ref": "refs/tags/v0.2.0",
          "repository": "https://github.com/ibrahimcesar/telemetry-kit"
        }
      }
    },
    "runDetails": {
      "builder": {
        "id": "https://github.com/actions/runner"
      },
      "metadata": {
        "invocationId": "https://github.com/ibrahimcesar/telemetry-kit/actions/runs/..."
      }
    }
  }
}
```

### Accessing Provenance

**Via GitHub CLI:**
```bash
# List attestations for a release
gh attestation list \
  --owner ibrahimcesar \
  --repo telemetry-kit \
  --limit 10

# Download specific attestation
gh attestation download \
  --repo ibrahimcesar/telemetry-kit \
  --digest-alg sha256 \
  --digest abc123...
```

**Via API:**
```bash
# Get attestations
curl -H "Authorization: Bearer $GITHUB_TOKEN" \
  https://api.github.com/repos/ibrahimcesar/telemetry-kit/attestations/sha256:abc123
```

---

## Verification

### For crates.io Users

```bash
# Install from crates.io
cargo install telemetry-kit

# Verify checksum matches GitHub release
sha256sum $(which telemetry-kit)

# Compare with GitHub release checksum
curl -sL https://github.com/ibrahimcesar/telemetry-kit/releases/download/v0.2.0/checksums.txt
```

### For Binary Users

```bash
# Download release
wget https://github.com/ibrahimcesar/telemetry-kit/releases/download/v0.2.0/telemetry-kit-linux-x86_64.tar.gz

# Verify checksum
sha256sum telemetry-kit-linux-x86_64.tar.gz

# Verify attestation
gh attestation verify telemetry-kit-linux-x86_64.tar.gz \
  --owner ibrahimcesar \
  --repo telemetry-kit
```

### For Source Users

```bash
# Clone repository
git clone https://github.com/ibrahimcesar/telemetry-kit
cd telemetry-kit

# Verify tag signature (if signed)
git tag -v v0.2.0

# Build from source
cargo build --release

# Verify build is reproducible
cargo clean
cargo build --release
sha256sum target/release/telemetry-kit
```

---

## Supply Chain Threats

### Threats We Mitigate

| Threat | Mitigation | SLSA Level |
|--------|-----------|------------|
| Compromised source repo | Branch protection, 2FA required | L2 |
| Malicious dependencies | cargo-deny, cargo-audit | L2 |
| Tampered build process | Immutable workflow files | L2 |
| Man-in-the-middle attacks | HTTPS, TLS certificate pinning | L1 |
| Compromised build environment | Ephemeral GitHub runners | L3 |
| Forged provenance | GitHub-signed attestations | L3 (partial) |

### Threats We're Working On

| Threat | Status | Target |
|--------|--------|--------|
| Dependency confusion | 🔄 Planning | L3 |
| Typosquatting | 🔄 Monitoring | L2 |
| Compromised maintainer account | ✅ 2FA enforced | L2 |
| Build parameter injection | 🔄 Hardening | L3 |
| Time-of-check to time-of-use (TOCTOU) | ❌ Future | L4 |

### Known Limitations

❌ **Not Yet Addressed:**
- Hermetic builds (dependencies fetched at build time)
- Reproducible builds (timestamps in binaries)
- Post-build tampering (waiting for Sigstore)
- Supply chain attacks on build tools (cargo, rustc)

---

## Roadmap

### Q1 2025 - SLSA Level 2 Complete ✅

- [x] GitHub Actions for builds
- [x] Provenance generation
- [x] Branch protection
- [x] Automated releases
- [x] Checksums published

### Q2 2025 - SLSA Level 3

- [ ] Sigstore integration
  - [ ] Sign releases with cosign
  - [ ] Upload to Rekor transparency log
  - [ ] Verify in CI/CD
- [ ] Hardened builds
  - [ ] Network isolation
  - [ ] Read-only filesystems
  - [ ] Minimal base images
- [ ] Parameterless builds
  - [ ] Version from git tags only
  - [ ] No manual workflow triggers

### Q3 2025 - Enhanced Security

- [ ] Reproducible builds
  - [ ] Fixed timestamps
  - [ ] Hermetic dependencies
  - [ ] Vendored crates
- [ ] SBOM generation
  - [ ] CycloneDX format
  - [ ] Dependency graph
  - [ ] License compliance
- [ ] Vulnerability scanning
  - [ ] Automated Dependabot PRs
  - [ ] Trivy container scanning
  - [ ] OSV.dev integration

### Q4 2025 - SLSA Level 4 (Aspirational)

- [ ] Two-party review
- [ ] Mandatory code review
- [ ] Signed commits required
- [ ] Build reproducibility verified
- [ ] Complete audit trail

---

## Dependencies

### Trusted Publishers

We trust these crates.io publishers:

- `tokio` - Tokio team (verified)
- `serde` - Serde team (verified)
- `reqwest` - Sean McArthur (verified)
- `rusqlite` - John Gallagher (verified)

### Dependency Verification

```bash
# Audit dependencies for vulnerabilities
cargo audit

# Check dependency licenses
cargo deny check licenses

# Review dependency authors
cargo supply-chain publishers

# Check for unmaintained crates
cargo outdated
```

### Cargo.lock

We commit `Cargo.lock` to ensure deterministic builds:

```bash
# Verify dependencies match lock
cargo verify-project

# Check for inconsistencies
cargo tree
```

---

## Contributing to SLSA Compliance

### For Contributors

**Before Submitting PRs:**
- Sign your commits: `git config --global commit.gpgsign true`
- Run security checks: `cargo audit && cargo deny check`
- No secrets in code: `git-secrets --scan`

**In Pull Requests:**
- Add new dependencies to `deny.toml` allowlist
- Update SBOM if dependencies change
- Note security implications in PR description

### For Maintainers

**Before Merging:**
- Review dependency changes carefully
- Check for suspicious code patterns
- Verify CI passes all security checks
- Ensure provenance will be generated

**Release Process:**
- Tag with GPG-signed tag
- Wait for automated build
- Verify attestation is generated
- Test installation from crates.io

---

## References

### SLSA Documentation

- [SLSA Levels](https://slsa.dev/spec/v1.0/levels)
- [SLSA Requirements](https://slsa.dev/spec/v1.0/requirements)
- [Provenance Format](https://slsa.dev/provenance/v1)

### Tools & Services

- [GitHub Attestations](https://docs.github.com/en/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds)
- [Sigstore](https://www.sigstore.dev/)
- [in-toto](https://in-toto.io/)
- [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)

### Standards

- [SLSA Specification v1.0](https://slsa.dev/spec/v1.0/)
- [in-toto Specification](https://github.com/in-toto/docs)
- [The Update Framework (TUF)](https://theupdateframework.io/)

---

## Contact

Questions about SLSA compliance?

- **Email:** security@ibrahimcesar.com
- **GitHub:** https://github.com/ibrahimcesar/telemetry-kit/security
- **Discussions:** https://github.com/ibrahimcesar/telemetry-kit/discussions

---

**Document Version:** 1.0
**Last Updated:** 2025-01-22
**SLSA Level:** 2 (working towards 3)

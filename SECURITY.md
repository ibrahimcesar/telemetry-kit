# Security Policy

## Supported Versions

We actively support the following versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| 0.2.x   | :white_check_mark: |
| < 0.2   | :x:                |

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities through public GitHub issues.**

### How to Report

Send security vulnerability reports to: **security@ibrahimcesar.com**

Include the following information:

- **Type of vulnerability** (e.g., authentication bypass, SQL injection, XSS, etc.)
- **Full paths of affected source files**
- **Location of the affected source code** (tag/branch/commit or direct URL)
- **Step-by-step instructions to reproduce the issue**
- **Proof-of-concept or exploit code** (if possible)
- **Impact of the vulnerability** (what an attacker could achieve)
- **Suggested fix** (if you have one)

### What to Expect

- **Initial Response:** Within 48 hours
- **Status Update:** Within 7 days
- **Fix Timeline:** Depends on severity
  - Critical: 1-7 days
  - High: 7-14 days
  - Medium: 14-30 days
  - Low: 30-90 days

### Disclosure Policy

- We will acknowledge your email within 48 hours
- We will confirm the vulnerability and determine its severity
- We will work on a fix and prepare a security advisory
- We will notify you when the fix is ready for review
- We will publicly disclose the vulnerability after the fix is released

### Bug Bounty

Currently, we do not have a paid bug bounty program. However:

- We will publicly credit you for responsible disclosure (if desired)
- We will add you to our `SECURITY_HALL_OF_FAME.md`
- We may send you telemetry-kit swag as a thank you

## Security Best Practices

### For SDK Users

1. **Keep Dependencies Updated**
   ```bash
   cargo update
   cargo audit
   ```

2. **Secure Your API Tokens**
   - Never commit tokens to version control
   - Use environment variables or secure vaults
   - Rotate tokens regularly
   - Use separate tokens for dev/staging/production

3. **Review Telemetry Data**
   - Ensure no PII is being tracked
   - Use privacy controls to sanitize data
   - Respect DO_NOT_TRACK headers
   - Obtain user consent where required

4. **Self-Hosting Security**
   - Use TLS/SSL for all connections
   - Keep PostgreSQL and Redis updated
   - Implement network isolation
   - Regular security audits
   - Monitor for suspicious activity

### For Self-Hosters

1. **Network Security**
   ```yaml
   # docker-compose.yml
   services:
     postgres:
       networks:
         - internal  # Not exposed to public
     redis:
       networks:
         - internal  # Not exposed to public
     api:
       networks:
         - internal
         - public    # Only API exposed
   ```

2. **Authentication**
   - Generate strong API tokens (32+ characters)
   - Use HMAC-SHA256 for all requests
   - Implement rate limiting
   - Monitor for brute force attempts

3. **Database Security**
   - Use strong passwords (generated, not human-chosen)
   - Enable SSL/TLS for database connections
   - Regular backups (encrypted)
   - Principle of least privilege for database users

4. **Monitoring**
   - Enable audit logging
   - Monitor failed authentication attempts
   - Set up alerts for unusual activity
   - Regular security scans

## Known Security Considerations

### HMAC Authentication

- **Constant-time comparison:** We use constant-time comparison to prevent timing attacks ✅
- **Timestamp validation:** ±10 minute window prevents replay attacks ✅
- **Nonce checking:** Prevents duplicate requests within window ✅

### SQLite Storage

- **Local file:** SDK stores events in `~/.telemetry-kit/`
- **Permissions:** File permissions should be 600 (user read/write only)
- **Encryption:** Data is not encrypted at rest (consider disk encryption)

### Network Communication

- **TLS Required:** Always use HTTPS endpoints in production
- **Certificate Validation:** SDK validates TLS certificates
- **No Sensitive Data:** Never include passwords, API keys, or tokens in events

### Privacy

- **Anonymous by Default:** User IDs are SHA-256 hashed machine IDs
- **No PII Collection:** SDK does not collect personally identifiable information
- **Data Minimization:** Only essential telemetry data is collected
- **User Control:** Consent flows and opt-out mechanisms available

## Security Features

### Implemented

- [x] HMAC-SHA256 request signing
- [x] Constant-time signature comparison
- [x] Timestamp-based replay protection
- [x] Nonce-based deduplication
- [x] Rate limiting (server-side)
- [x] Input validation
- [x] SQL injection protection (parameterized queries)
- [x] Anonymous user IDs (SHA-256 hashed)

### Planned

- [ ] Token rotation automation
- [ ] Encryption at rest (SQLite)
- [ ] Advanced rate limiting (client-side)
- [ ] Anomaly detection
- [ ] Security headers (CSP, HSTS, etc.)
- [ ] Dependency vulnerability scanning (automated PRs)

## Compliance

### GDPR

- **Data Minimization:** ✅ Only collect necessary data
- **Anonymization:** ✅ User IDs are hashed
- **Right to Erasure:** ✅ API available to delete user data
- **Data Portability:** ✅ Export APIs available
- **Consent Management:** 🚧 In progress

### Privacy Regulations

- **DO_NOT_TRACK:** ✅ Respected by SDK
- **Opt-Out:** ✅ Users can disable telemetry
- **Transparency:** ✅ Open source code, clear documentation

## Security Tooling

### Automated Checks

We use the following tools in CI/CD:

- **cargo-audit:** Dependency vulnerability scanning
- **cargo-deny:** Supply chain security
- **clippy:** Code quality and security lints
- **rustfmt:** Consistent code style

### Manual Reviews

- Code reviews required for all PRs
- Security-focused reviews for auth/crypto changes
- Annual third-party security audits (planned)

## Contact

- **Security Issues:** security@ibrahimcesar.com
- **General Questions:** email@ibrahimcesar.com
- **GitHub:** https://github.com/ibrahimcesar/telemetry-kit

## Hall of Fame

We thank the following researchers for responsible disclosure:

*(No reports yet - be the first!)*

## SLSA Compliance

We follow SLSA (Supply-chain Levels for Software Artifacts) best practices:

- **SLSA Level 2** (current): ✅ Version control, build service, provenance
- **SLSA Level 3** (planned): 🔄 Hardened builds, non-falsifiable provenance

See [SLSA.md](SLSA.md) for detailed compliance information.

## Resources

### Documentation

- [Security Best Practices](https://telemetry-kit.dev/docs/security)
- [Privacy Controls](https://telemetry-kit.dev/docs/privacy)
- [Error Handling](https://telemetry-kit.dev/docs/error-handling)
- [Self-Hosting Guide](https://telemetry-kit.dev/docs/self-hosting)

### Standards & References

- [CVSS 3.1](https://www.first.org/cvss/) - Vulnerability severity scoring
- [CWE](https://cwe.mitre.org/) - Common weakness enumeration
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [SLSA](https://slsa.dev/) - Supply chain security framework

---

**Last Updated:** 2025-01-22
**Version:** 1.1

# Unmaintained Dependencies - Migration Plan

**Date:** 2025-01-23
**Status:** Identified during Week 8 Security Audit
**Severity:** Medium (supply chain risk)

---

## Overview

During our security audit using `cargo-audit`, we identified 4 unmaintained dependencies. This document provides migration strategies for each.

**Important:** All 4 unmaintained crates are used by **server code only** - the telemetry-kit SDK itself is clean.

---

## 1. json5 (RUSTSEC-2025-0120)

**Current Usage:** Server (via `config` crate for configuration files)
**Unmaintained Since:** 2024
**Risk Level:** Medium

### Alternatives

#### Option 1: json-five (Recommended)
- **Crate:** `json_five` (json-five)
- **Status:** Actively maintained (updated Feb 2025)
- **Performance:** 3-4x faster than json5, up to 20x in synthetic tests
- **Compatibility:** Full serde support
- **Migration Effort:** Low (drop-in replacement)

```toml
# Before
json5 = "0.4"

# After
json_five = "0.6"
```

#### Option 2: Switch to TOML
- **Rationale:** TOML is Rust's standard config format
- **Crate:** `toml` (very actively maintained)
- **Migration Effort:** Medium (config file format change)

```toml
toml = "0.8"
```

**Recommendation:** Use `json_five` for backward compatibility, or migrate to TOML for better Rust ecosystem alignment.

**Sources:**
- [json-five on Lib.rs](https://lib.rs/crates/json-five)
- [json-five GitHub](https://github.com/spyoungtech/json-five-rs)

---

## 2. number_prefix (RUSTSEC-2025-0119)

**Current Usage:** SDK (via `indicatif` crate for progress bars in CLI)
**Unmaintained Since:** 2024
**Risk Level:** Low (display-only, no security impact)

### Alternatives

#### Option 1: format_num (Recommended)
- **Status:** Actively maintained
- **Features:** SI prefix formatting, Python-style format specs
- **Example:** `42e6` → `"42M"`

```toml
format_num = "0.1"
```

```rust
// Before (via indicatif)
use number_prefix::NumberPrefix;

// After
use format_num::NumberFormat;
let formatted = NumberFormat::default().format(".2s", 42_000_000); // "42M"
```

#### Option 2: human_format
- **Status:** Actively maintained
- **Features:** Customizable magnitude-based formatting
- **Node.js Port:** Based on human-format from Node

```toml
human_format = "1.1"
```

#### Option 3: numfmt
- **Status:** Actively maintained
- **Features:** Fast formatting, std-compatible
- **Example:** `12345.6789` → `"12.345 K"`

```toml
numfmt = "1.2"
```

**Recommendation:**
- **Short term:** Monitor indicatif updates (they may migrate internally)
- **Long term:** If indicatif doesn't update, fork or switch to `format_num`

**Note:** Since this is a transitive dependency via `indicatif`, we should:
1. Check if newer `indicatif` versions use a maintained alternative
2. Open issue with indicatif maintainers
3. Only migrate if necessary

**Sources:**
- [format_num on crates.io](https://crates.io/crates/format_num)
- [human_format on crates.io](https://crates.io/crates/human_format)
- [numfmt on crates.io](https://crates.io/crates/numfmt)

---

## 3. paste (RUSTSEC-2024-0436)

**Current Usage:** Server (via `sqlx` for SQL macro magic)
**Unmaintained Since:** October 2024
**Risk Level:** Low (compile-time only, no runtime risk)

### Alternatives

#### Option 1: pastey (Recommended)
- **Status:** Actively maintained (2025)
- **Compatibility:** Drop-in replacement for `paste`
- **Features:** Additional raw identifier support, replace modifier
- **Migration Effort:** Minimal

```toml
# Before
paste = "1.0"

# After
pastey = "0.3"
```

**Migration:**
```rust
// No code changes needed - just update Cargo.toml
```

#### Option 2: paste2
- **Status:** Maintained fork
- **Goal:** Drop-in replacement for dtolnay/paste

```toml
paste2 = "1.0"
```

#### Option 3: Wait for sqlx Update
- sqlx may migrate to pastey in future versions
- Monitor sqlx releases for paste removal

**Recommendation:**
- **Short term:** Monitor sqlx releases
- **Long term:** If sqlx doesn't migrate, consider `pastey` as drop-in replacement

**Note:** Since this is transitive via `sqlx`, migration should happen in server repository, not SDK.

**Sources:**
- [pastey on Lib.rs](https://lib.rs/crates/pastey)
- [pastey on crates.io](https://crates.io/crates/pastey)
- [paste2 on crates.io](https://crates.io/crates/paste2)
- [RUSTSEC-2024-0436 Analysis](https://cmcgeemedia.com/blog/rustsec-2024-0436-paste-crate)

---

## 4. proc-macro-error (RUSTSEC-2024-0370)

**Current Usage:** Server (via `validator` crate for validation macros)
**Unmaintained Since:** 2024
**Risk Level:** Low (compile-time only)

### Alternatives

#### Option 1: syn::Error (Recommended - Modern Standard)
- **Status:** Part of `syn` (very actively maintained)
- **Approach:** Use `syn::Error` directly for proc-macro error handling
- **Migration Effort:** Low if updating validator

```rust
// Modern approach (2025)
use syn::Error;

// Generate compile_error! automatically
return Err(Error::new(span, "validation failed"));
```

**Modern Pattern (2025):**
```rust
// Return Result with optional errors
Result<(T, Option<syn::Error>), syn::Error>

// Ok((T, Some(err))) = error that didn't block code generation
// Ok((T, None)) = success
// Err(err) = blocking error
```

#### Option 2: proc-macro-error2
- **Status:** Maintained fork/continuation
- **Compatibility:** Similar API to original

```toml
proc-macro-error2 = "2.0"
```

#### Option 3: compile_error! Trick
- **Approach:** Manually emit `compile_error!` in generated code
- **Stability:** Works on stable Rust

```rust
quote! {
    compile_error!("validation failed");
}
```

**Recommendation:**
- **Best Practice (2025):** Use `syn::Error` directly
- **Quick Fix:** Use `proc-macro-error2` as drop-in replacement
- **Long Term:** Migrate to `syn::Error` for better IDE support

**Note:** Like `paste`, this is transitive via `validator`. Check if newer validator versions migrate away from proc-macro-error.

**Sources:**
- [syn::Error Documentation](https://docs.rs/syn/latest/syn/struct.Error.html)
- [Procedural Macros: Error handling](https://blog.turbo.fish/proc-macro-error-handling/)
- [proc-macro-error2 on crates.io](https://crates.io/crates/proc-macro-error2)

---

## Migration Priority

| Crate | Priority | Reason | Timeline |
|-------|----------|--------|----------|
| json5 | **HIGH** | Direct server dependency | Q1 2025 |
| number_prefix | LOW | Transitive (via indicatif) | Monitor |
| paste | LOW | Transitive (via sqlx) | Monitor |
| proc-macro-error | LOW | Transitive (via validator) | Monitor |

---

## Action Plan

### Immediate (This Week)

1. **json5 → json_five**
   - Update server Cargo.toml
   - Test configuration loading
   - No code changes needed (serde compatible)

### Short Term (Q1 2025)

2. **Monitor Transitive Dependencies**
   - Track indicatif releases for number_prefix migration
   - Track sqlx releases for paste migration
   - Track validator releases for proc-macro-error migration

3. **Open Upstream Issues**
   - Report unmaintained deps to indicatif maintainers
   - Report to sqlx maintainers
   - Report to validator maintainers

### Long Term (Q2 2025)

4. **Evaluate Direct Migration**
   - If upstreams don't migrate, consider:
     - Forking with maintained alternatives
     - Switching libraries entirely
     - Direct implementation (for simple cases like number_prefix)

---

## Testing Strategy

For each migration:

1. **Unit Tests:** Ensure functionality unchanged
2. **Integration Tests:** Test full workflow
3. **Benchmark:** Verify no performance regression
4. **Security Audit:** Run `cargo audit` after migration

---

## Documentation Updates

After migration:
- [ ] Update DEPENDENCIES.md
- [ ] Update SECURITY.md (remove from unmaintained list)
- [ ] Update CHANGELOG.md
- [ ] Update server README with migration notes

---

## References

### Security Advisories
- [RUSTSEC-2025-0120: json5 unmaintained](https://rustsec.org/)
- [RUSTSEC-2025-0119: number_prefix unmaintained](https://rustsec.org/)
- [RUSTSEC-2024-0436: paste unmaintained](https://rustsec.org/)
- [RUSTSEC-2024-0370: proc-macro-error unmaintained](https://rustsec.org/)

### Alternative Crates
- [json-five (json_five)](https://lib.rs/crates/json-five)
- [format_num](https://crates.io/crates/format_num)
- [pastey](https://lib.rs/crates/pastey)
- [syn::Error](https://docs.rs/syn/latest/syn/struct.Error.html)

---

**Last Updated:** 2025-01-23
**Next Review:** Q1 2025 (after monitoring upstream migrations)

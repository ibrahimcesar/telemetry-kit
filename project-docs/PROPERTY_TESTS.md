# Property-Based Testing with Proptest

## Overview

Property-based tests have been added to the privacy module using [proptest](https://github.com/proptest-rs/proptest). These tests verify that privacy functions maintain their invariants across a wide range of inputs.

## What is Property-Based Testing?

Instead of testing with specific examples, property-based testing:
- Generates hundreds of random test cases automatically
- Verifies that properties (invariants) hold for ALL inputs
- Finds edge cases you might not think of manually
- Provides better coverage than example-based tests

## Privacy Module Properties

### 11 Property Tests Added

#### 1. **Email Sanitization Always Valid**
```rust
fn sanitize_email_always_valid(email in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}")
```
**Property**: Sanitized emails always:
- Start with `email_`
- Never contain `@`
- Never contain the original email

**Why**: Ensures PII is never leaked through sanitization

---

#### 2. **Email Sanitization is Deterministic**
```rust
fn sanitize_email_deterministic(email in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}")
```
**Property**: Same email always produces same hash

**Why**: Ensures consistent behavior and allows correlation without exposing PII

---

#### 3. **Email Sanitization is Unique**
```rust
fn sanitize_email_unique(email1, email2)
```
**Property**: Different emails produce different hashes

**Why**: Prevents hash collisions that could conflate different users

---

#### 4. **Path Sanitization Hides Home**
```rust
fn sanitize_path_hides_home(suffix in "[a-zA-Z0-9/_.-]+")
```
**Property**: Paths under home directory:
- Always start with `~`
- Never contain actual home path

**Why**: Prevents username leakage through file paths

---

#### 5. **Path Sanitization is Idempotent**
```rust
fn sanitize_path_idempotent(suffix in "[a-zA-Z0-9/_.-]+")
```
**Property**: Sanitizing a sanitized path produces same result

**Why**: Ensures sanitization can be applied multiple times safely

---

#### 6. **Non-Home Paths Preserved**
```rust
fn sanitize_path_preserves_non_home(path in "/tmp/[a-zA-Z0-9/_.-]+")
```
**Property**: Paths outside home directory remain unchanged

**Why**: Prevents over-sanitization that could break functionality

---

#### 7. **Sanitized Data Never Contains Raw Emails**
```rust
fn sanitize_data_removes_emails(email in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}")
```
**Property**: After sanitization, JSON data:
- Never contains original email
- Always contains `email_` prefix

**Why**: Ensures recursive sanitization works correctly

---

#### 8. **ConsentStatus Serialization Roundtrip**
```rust
fn consent_status_serde_roundtrip(status in [Unknown, Granted, Denied, OptedOut])
```
**Property**: Serialize → Deserialize preserves consent status

**Why**: Ensures consent storage doesn't corrupt data

---

#### 9. **PrivacyConfig Cloning Preserves Fields**
```rust
fn privacy_config_clone_preserves(consent_required, sanitize_paths, ...)
```
**Property**: Cloning PrivacyConfig preserves all 6 fields

**Why**: Ensures config can be safely cloned without data loss

---

#### 10. **Email Hash Format is Consistent**
```rust
fn sanitize_email_hash_format(email in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}")
```
**Property**: Email hash is always:
- Exactly 16 hex characters
- All lowercase hex digits

**Why**: Ensures consistent hash format for parsing/validation

---

#### 11. **Recursive Sanitization Handles Nesting**
```rust
fn sanitize_data_handles_nesting(email, depth in 1..=5)
```
**Property**: Deeply nested emails (up to 5 levels) are all sanitized

**Why**: Ensures sanitization works on complex JSON structures

---

## Running Property Tests

### Run Only Property Tests
```bash
cargo test --lib privacy::tests::proptests
```

### Run All Privacy Tests
```bash
cargo test --lib privacy
```

### Run with Verbose Output
```bash
cargo test --lib privacy::tests::proptests -- --nocapture
```

### Increase Test Cases (Default: 256)
```bash
PROPTEST_CASES=10000 cargo test --lib privacy::tests::proptests
```

## Test Results

```
running 11 tests
test privacy::tests::proptests::consent_status_serde_roundtrip ... ok
test privacy::tests::proptests::privacy_config_clone_preserves ... ok
test privacy::tests::proptests::sanitize_data_handles_nesting ... ok
test privacy::tests::proptests::sanitize_data_removes_emails ... ok
test privacy::tests::proptests::sanitize_email_always_valid ... ok
test privacy::tests::proptests::sanitize_email_deterministic ... ok
test privacy::tests::proptests::sanitize_email_hash_format ... ok
test privacy::tests::proptests::sanitize_email_unique ... ok
test privacy::tests::proptests::sanitize_path_hides_home ... ok
test privacy::tests::proptests::sanitize_path_idempotent ... ok
test privacy::tests::proptests::sanitize_path_preserves_non_home ... ok

test result: ok. 11 passed; 0 failed
```

**Total Privacy Tests**: 20 (9 example-based + 11 property-based)

## Benefits

1. **Edge Case Discovery**: Proptest automatically finds edge cases like:
   - Very long email addresses
   - Unusual characters in paths
   - Deeply nested JSON structures
   - Minimal valid inputs (e.g., `a@b.co`)

2. **Regression Prevention**: If a change breaks a property, proptest will:
   - Find the minimal failing case
   - Save it for regression testing
   - Report exactly which property failed

3. **Documentation**: Properties serve as executable documentation of invariants

4. **Confidence**: 11 properties × 256 test cases = 2,816 test cases per run

## Future Property Tests

### Potential Additions

1. **Event Serialization**
   ```rust
   proptest! {
       fn event_serde_roundtrip(event: Event) {
           let json = serde_json::to_string(&event)?;
           let deserialized = serde_json::from_str(&json)?;
           prop_assert_eq!(event, deserialized);
       }
   }
   ```

2. **HMAC Signature Determinism**
   ```rust
   proptest! {
       fn hmac_signature_deterministic(data: Vec<u8>, key: String) {
           let sig1 = generate_hmac(&data, &key);
           let sig2 = generate_hmac(&data, &key);
           prop_assert_eq!(sig1, sig2);
       }
   }
   ```

3. **Storage Roundtrip**
   ```rust
   proptest! {
       fn storage_roundtrip(events: Vec<Event>) {
           storage.store_batch(&events)?;
           let loaded = storage.load_all()?;
           prop_assert_eq!(events, loaded);
       }
   }
   ```

## Configuration

Property tests can be configured via `proptest! { #![proptest_config(...)] }`:

```rust
proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1000,           // Run 1000 test cases instead of 256
        max_shrink_iters: 10000, // Spend more time finding minimal failing case
        .. ProptestConfig::default()
    })]

    #[test]
    fn my_property_test(...) { ... }
}
```

## See Also

- [Proptest Book](https://proptest-rs.github.io/proptest/intro.html)
- [Privacy Implementation](PRIVACY_IMPLEMENTATION.md)
- [Week 5 Complete](WEEK_5_COMPLETE.md)
- [Testing Guide](docs/content/docs/testing.mdx) (future)

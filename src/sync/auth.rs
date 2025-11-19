//! HMAC-SHA256 authentication for sync requests

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// HMAC authentication helper
pub struct HmacAuth {
    secret: String,
}

impl HmacAuth {
    /// Create a new HMAC authenticator with the given secret
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),
        }
    }

    /// Calculate HMAC signature for a message
    ///
    /// Message format: `{timestamp}:{nonce}:{body}`
    ///
    /// # Arguments
    /// * `timestamp` - Unix timestamp in seconds
    /// * `nonce` - Unique nonce (UUID v4)
    /// * `body` - JSON request body
    ///
    /// # Returns
    /// Hex-encoded HMAC-SHA256 signature
    pub fn sign(&self, timestamp: &str, nonce: &str, body: &str) -> String {
        let message = format!("{}:{}:{}", timestamp, nonce, body);

        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");

        mac.update(message.as_bytes());

        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    /// Verify an HMAC signature
    ///
    /// # Arguments
    /// * `timestamp` - Unix timestamp in seconds
    /// * `nonce` - Unique nonce
    /// * `body` - JSON request body
    /// * `signature` - Signature to verify
    ///
    /// # Returns
    /// `true` if signature is valid, `false` otherwise
    pub fn verify(&self, timestamp: &str, nonce: &str, body: &str, signature: &str) -> bool {
        let expected = self.sign(timestamp, nonce, body);
        constant_time_eq(signature.as_bytes(), expected.as_bytes())
    }
}

/// Constant-time comparison to prevent timing attacks
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut diff = 0u8;
    for (a_byte, b_byte) in a.iter().zip(b.iter()) {
        diff |= a_byte ^ b_byte;
    }

    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_signing() {
        let auth = HmacAuth::new("test_secret_key");
        let timestamp = "1732003200";
        let nonce = "550e8400-e29b-41d4-a716-446655440000";
        let body = r#"{"events":[]}"#;

        let signature = auth.sign(timestamp, nonce, body);

        // Signature should be a 64-character hex string
        assert_eq!(signature.len(), 64);
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_hmac_verification() {
        let auth = HmacAuth::new("test_secret_key");
        let timestamp = "1732003200";
        let nonce = "550e8400-e29b-41d4-a716-446655440000";
        let body = r#"{"events":[]}"#;

        let signature = auth.sign(timestamp, nonce, body);

        // Verify the signature
        assert!(auth.verify(timestamp, nonce, body, &signature));

        // Tampered body should fail verification
        let tampered_body = r#"{"events":[{"tampered":true}]}"#;
        assert!(!auth.verify(timestamp, nonce, tampered_body, &signature));
    }

    #[test]
    fn test_signature_determinism() {
        let auth = HmacAuth::new("test_secret_key");
        let timestamp = "1732003200";
        let nonce = "550e8400-e29b-41d4-a716-446655440000";
        let body = r#"{"events":[]}"#;

        let sig1 = auth.sign(timestamp, nonce, body);
        let sig2 = auth.sign(timestamp, nonce, body);

        // Same inputs should produce same signature
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"hello", b"hello!"));
    }
}

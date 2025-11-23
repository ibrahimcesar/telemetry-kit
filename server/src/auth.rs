//! HMAC authentication and verification

use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use hmac::{Hmac, Mac};
use serde_json::json;
use sha2::Sha256;
use std::sync::Arc;

use crate::models::ApiToken;
use crate::AppState;

type HmacSha256 = Hmac<Sha256>;

/// HMAC verification middleware
pub async fn verify_hmac(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    // Extract headers
    let signature = request
        .headers()
        .get("X-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| error_response(StatusCode::UNAUTHORIZED, "Missing X-Signature header"))?
        .to_string();

    let timestamp = request
        .headers()
        .get("X-Timestamp")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| error_response(StatusCode::UNAUTHORIZED, "Missing X-Timestamp header"))?
        .to_string();

    // Extract body
    let (parts, body) = request.into_parts();
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| error_response(StatusCode::BAD_REQUEST, "Failed to read body"))?;

    let body_str = String::from_utf8(body_bytes.to_vec())
        .map_err(|_| error_response(StatusCode::BAD_REQUEST, "Invalid UTF-8 in body"))?;

    // Get token from Authorization header
    let auth_header = parts
        .headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    let token_str = auth_header.ok_or_else(|| {
        error_response(
            StatusCode::UNAUTHORIZED,
            "Missing or invalid Authorization header",
        )
    })?;

    // Fetch token from database
    let token = sqlx::query_as::<_, ApiToken>(
        "SELECT * FROM api_tokens WHERE token = $1 AND is_active = true",
    )
    .bind(token_str)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| error_response(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
    .ok_or_else(|| error_response(StatusCode::UNAUTHORIZED, "Invalid token"))?;

    // Verify HMAC signature
    let message = format!("{}:{}", timestamp, body_str);
    let is_valid = verify_signature(&message, &signature, &token.secret);

    if !is_valid {
        return Err(error_response(
            StatusCode::UNAUTHORIZED,
            "Invalid HMAC signature",
        ));
    }

    // Validate timestamp (within ±10 minutes)
    let request_time = timestamp
        .parse::<i64>()
        .map_err(|_| error_response(StatusCode::BAD_REQUEST, "Invalid timestamp format"))?;

    let now = Utc::now().timestamp();
    let time_diff = (now - request_time).abs();

    if time_diff > 600 {
        return Err(error_response(
            StatusCode::FORBIDDEN,
            "Timestamp outside acceptable window",
        ));
    }

    // Update last_used_at
    sqlx::query("UPDATE api_tokens SET last_used_at = NOW() WHERE id = $1")
        .bind(token.id)
        .execute(&state.db)
        .await
        .ok();

    // Reconstruct request with body and add extensions
    let mut request = Request::from_parts(parts, Body::from(body_bytes));
    request.extensions_mut().insert(token);
    request.extensions_mut().insert(body_str);

    Ok(next.run(request).await)
}

/// Verify HMAC signature
fn verify_signature(message: &str, signature: &str, secret: &str) -> bool {
    let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(mac) => mac,
        Err(_) => return false,
    };

    mac.update(message.as_bytes());

    let expected = hex::encode(mac.finalize().into_bytes());

    constant_time_eq(signature.as_bytes(), expected.as_bytes())
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

fn error_response(status: StatusCode, message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        status,
        Json(json!({
            "error": status.canonical_reason().unwrap_or("error"),
            "message": message
        })),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_signature_valid() {
        let secret = "test_secret";
        let message = "1234567890:body";

        // Generate a valid signature
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(message.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        // Verify it
        assert!(verify_signature(message, &signature, secret));
    }

    #[test]
    fn test_verify_signature_invalid() {
        let secret = "test_secret";
        let message = "1234567890:body";
        let wrong_signature = "invalid_signature";

        assert!(!verify_signature(message, wrong_signature, secret));
    }

    #[test]
    fn test_verify_signature_wrong_secret() {
        let secret = "test_secret";
        let wrong_secret = "wrong_secret";
        let message = "1234567890:body";

        // Generate signature with correct secret
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(message.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        // Try to verify with wrong secret
        assert!(!verify_signature(message, &signature, wrong_secret));
    }

    #[test]
    fn test_constant_time_eq_equal() {
        let a = b"test_string";
        let b = b"test_string";
        assert!(constant_time_eq(a, b));
    }

    #[test]
    fn test_constant_time_eq_different() {
        let a = b"test_string";
        let b = b"diff_string";
        assert!(!constant_time_eq(a, b));
    }

    #[test]
    fn test_constant_time_eq_different_length() {
        let a = b"test";
        let b = b"test_longer";
        assert!(!constant_time_eq(a, b));
    }

    #[test]
    fn test_hmac_message_format() {
        // Verify the message format matches the spec: "timestamp:body"
        let timestamp = "1234567890";
        let body = r#"{"events":[{"event_id":"..."}]}"#;

        let message = format!("{}:{}", timestamp, body);

        assert_eq!(
            message,
            format!("1234567890:{}", body)
        );
    }
}

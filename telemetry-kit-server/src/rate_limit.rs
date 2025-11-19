//! Rate limiting middleware

use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde_json::json;
use std::sync::Arc;

use crate::{models::{ApiToken, TokenTier}, AppState};

/// Rate limit middleware
pub async fn rate_limit(
    state: Arc<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    // Get token from request extensions (set by HMAC middleware)
    let token = request
        .extensions()
        .get::<ApiToken>()
        .cloned()
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "unauthorized",
                    "message": "Missing authentication"
                })),
            )
        })?;

    // Get rate limit for this tier
    let limit = match token.tier {
        TokenTier::Free => state.config.rate_limit.free_rpm,
        TokenTier::Pro => state.config.rate_limit.pro_rpm,
        TokenTier::Business => state.config.rate_limit.business_rpm,
        TokenTier::Enterprise => return Ok(next.run(request).await), // No limit for enterprise
    };

    // Rate limit key: token_id:minute_window
    let now = Utc::now();
    let minute_window = now.timestamp() / 60;
    let key = format!("ratelimit:{}:{}", token.id, minute_window);

    // Increment counter with 60 second expiry
    let count = state
        .redis
        .incr_with_expiry(&key, 60)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "internal_error",
                    "message": "Rate limit check failed"
                })),
            )
        })?;

    // Check if over limit
    if count > limit as i64 {
        let reset_at = (minute_window + 1) * 60;
        let retry_after = reset_at - now.timestamp();

        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            [
                ("X-RateLimit-Limit", limit.to_string()),
                ("X-RateLimit-Remaining", "0".to_string()),
                ("X-RateLimit-Reset", reset_at.to_string()),
                ("Retry-After", retry_after.to_string()),
            ],
            Json(json!({
                "error": "rate_limit_exceeded",
                "message": "Rate limit exceeded for this token",
                "retry_after": retry_after
            })),
        ));
    }

    // Add rate limit headers to response
    let remaining = limit.saturating_sub(count as u32);
    let reset_at = (minute_window + 1) * 60;

    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("X-RateLimit-Limit", limit.to_string().parse().unwrap());
    headers.insert(
        "X-RateLimit-Remaining",
        remaining.to_string().parse().unwrap(),
    );
    headers.insert("X-RateLimit-Reset", reset_at.to_string().parse().unwrap());

    Ok(response)
}

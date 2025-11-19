//! Rate limiting middleware

use axum::{
    extract::{Request, State},
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde_json::json;
use std::sync::Arc;

use crate::{
    models::{ApiToken, TokenTier},
    AppState,
};

/// Rate limit middleware
pub async fn rate_limit(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    // Get token from request extensions (set by HMAC middleware)
    let token = match request.extensions().get::<ApiToken>().cloned() {
        Some(token) => token,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "unauthorized",
                    "message": "Missing authentication"
                })),
            )
                .into_response();
        }
    };

    // Get rate limit for this tier
    let limit = match token.tier {
        TokenTier::Free => state.config.rate_limit.free_rpm,
        TokenTier::Pro => state.config.rate_limit.pro_rpm,
        TokenTier::Business => state.config.rate_limit.business_rpm,
        TokenTier::Enterprise => return next.run(request).await, // No limit for enterprise
    };

    // Rate limit key: token_id:minute_window
    let now = Utc::now();
    let minute_window = now.timestamp() / 60;
    let key = format!("ratelimit:{}:{}", token.id, minute_window);

    // Increment counter with 60 second expiry
    let count = match state.redis.incr_with_expiry(&key, 60).await {
        Ok(count) => count,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "internal_error",
                    "message": "Rate limit check failed"
                })),
            )
                .into_response();
        }
    };

    // Check if over limit
    if count > limit as i64 {
        let reset_at = (minute_window + 1) * 60;
        let retry_after = reset_at - now.timestamp();

        let mut response = (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({
                "error": "rate_limit_exceeded",
                "message": "Rate limit exceeded for this token",
                "retry_after": retry_after
            })),
        )
            .into_response();

        // Add rate limit headers
        let headers = response.headers_mut();
        headers.insert(
            "X-RateLimit-Limit",
            HeaderValue::from_str(&limit.to_string()).unwrap(),
        );
        headers.insert("X-RateLimit-Remaining", HeaderValue::from_static("0"));
        headers.insert(
            "X-RateLimit-Reset",
            HeaderValue::from_str(&reset_at.to_string()).unwrap(),
        );
        headers.insert(
            "Retry-After",
            HeaderValue::from_str(&retry_after.to_string()).unwrap(),
        );

        return response;
    }

    // Add rate limit headers to response
    let remaining = limit.saturating_sub(count as u32);
    let reset_at = (minute_window + 1) * 60;

    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        "X-RateLimit-Limit",
        HeaderValue::from_str(&limit.to_string()).unwrap(),
    );
    headers.insert(
        "X-RateLimit-Remaining",
        HeaderValue::from_str(&remaining.to_string()).unwrap(),
    );
    headers.insert(
        "X-RateLimit-Reset",
        HeaderValue::from_str(&reset_at.to_string()).unwrap(),
    );

    response
}

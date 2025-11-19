//! Request handlers

use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    models::{ApiToken, EventBatch, IncomingEvent},
    AppState,
};

/// Ingestion endpoint path parameters
#[derive(Debug, Deserialize)]
pub struct IngestPath {
    pub org_id: Uuid,
    pub app_id: Uuid,
}

/// Success response
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    status: String,
    accepted: usize,
    rejected: usize,
    message: String,
}

/// Partial success response
#[derive(Debug, Serialize)]
pub struct PartialResponse {
    status: String,
    accepted: usize,
    rejected: usize,
    errors: Vec<EventError>,
}

/// Event error detail
#[derive(Debug, Serialize)]
pub struct EventError {
    event_id: Uuid,
    error: String,
    message: String,
}

/// Health check endpoint
pub async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Ingest events endpoint
pub async fn ingest(
    State(state): State<Arc<AppState>>,
    Path(params): Path<IngestPath>,
    request: Request,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // Get token from extensions (set by auth middleware)
    let token = request
        .extensions()
        .get::<ApiToken>()
        .cloned()
        .ok_or_else(|| error_response(StatusCode::UNAUTHORIZED, "Missing authentication"))?;

    // Verify org_id and app_id match the token
    if token.org_id != params.org_id || token.app_id != params.app_id {
        return Err(error_response(
            StatusCode::FORBIDDEN,
            "Token does not match org_id/app_id",
        ));
    }

    // Get body from extensions (set by auth middleware)
    let body = request
        .extensions()
        .get::<String>()
        .cloned()
        .ok_or_else(|| error_response(StatusCode::INTERNAL_SERVER_ERROR, "Missing body"))?;

    // Parse batch
    let batch: EventBatch = serde_json::from_str(&body)
        .map_err(|e| error_response(StatusCode::BAD_REQUEST, &format!("Invalid JSON: {}", e)))?;

    // Validate batch size
    if batch.events.is_empty() {
        return Err(error_response(
            StatusCode::BAD_REQUEST,
            "Batch must contain at least one event",
        ));
    }

    if batch.events.len() > 1000 {
        return Err(error_response(
            StatusCode::BAD_REQUEST,
            "Batch size exceeds maximum of 1000 events",
        ));
    }

    // Validate headers
    let batch_size_header = request
        .headers()
        .get("X-Batch-Size")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<usize>().ok());

    if let Some(header_size) = batch_size_header {
        if header_size != batch.events.len() {
            return Err(error_response(
                StatusCode::BAD_REQUEST,
                "X-Batch-Size header does not match actual batch size",
            ));
        }
    }

    // Check DNT header
    if let Some(dnt) = request.headers().get("DNT") {
        if dnt == "1" {
            // Silent rejection for DNT=1
            return Ok((StatusCode::NO_CONTENT, ()).into_response());
        }
    }

    // Process events
    let mut accepted = 0;
    let mut errors = Vec::new();

    for event in batch.events {
        match process_event(&state, &event, params.org_id, params.app_id).await {
            Ok(()) => accepted += 1,
            Err(e) => errors.push(EventError {
                event_id: event.event_id,
                error: e.0.clone(),
                message: e.1,
            }),
        }
    }

    let rejected = errors.len();

    // Return appropriate response
    if errors.is_empty() {
        Ok(Json(SuccessResponse {
            status: "success".to_string(),
            accepted,
            rejected,
            message: "All events ingested successfully".to_string(),
        })
        .into_response())
    } else if accepted > 0 {
        Ok((
            StatusCode::MULTI_STATUS,
            Json(PartialResponse {
                status: "partial".to_string(),
                accepted,
                rejected,
                errors,
            }),
        )
            .into_response())
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "bad_request",
                "message": "All events rejected",
                "errors": errors
            })),
        ))
    }
}

/// Process a single event
async fn process_event(
    state: &AppState,
    event: &IncomingEvent,
    org_id: Uuid,
    app_id: Uuid,
) -> Result<(), (String, String)> {
    // Validate schema version
    if !is_supported_schema(&event.schema_version) {
        return Err((
            "unsupported_schema".to_string(),
            format!("Unsupported schema version: {}", event.schema_version),
        ));
    }

    // Validate required fields
    if event.service.name.is_empty() {
        return Err((
            "invalid_schema".to_string(),
            "Missing required field: service.name".to_string(),
        ));
    }

    if event.service.version.is_empty() {
        return Err((
            "invalid_schema".to_string(),
            "Missing required field: service.version".to_string(),
        ));
    }

    if !event.user_id.starts_with("client_") {
        return Err((
            "invalid_schema".to_string(),
            "User ID must start with 'client_'".to_string(),
        ));
    }

    // Check for duplicate event_id
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM events WHERE event_id = $1)")
        .bind(event.event_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            (
                "database_error".to_string(),
                format!("Database error: {}", e),
            )
        })?;

    if exists {
        return Err((
            "duplicate".to_string(),
            format!("Event already ingested (duplicate event_id: {})", event.event_id),
        ));
    }

    // Convert to stored event
    let stored = event.to_stored(org_id, app_id);

    // Insert into database
    sqlx::query(
        r#"
        INSERT INTO events (
            event_id, org_id, app_id, schema_version, timestamp,
            service_name, service_version, service_language, service_language_version,
            user_id, session_id,
            os, os_version, arch, ci, shell,
            event_type, event_category, event_data,
            sdk_version, transmission_timestamp, batch_size, retry_count,
            received_at
        ) VALUES (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9,
            $10, $11,
            $12, $13, $14, $15, $16,
            $17, $18, $19,
            $20, $21, $22, $23,
            $24
        )
        "#,
    )
    .bind(stored.event_id)
    .bind(stored.org_id)
    .bind(stored.app_id)
    .bind(stored.schema_version)
    .bind(stored.timestamp)
    .bind(stored.service_name)
    .bind(stored.service_version)
    .bind(stored.service_language)
    .bind(stored.service_language_version)
    .bind(stored.user_id)
    .bind(stored.session_id)
    .bind(stored.os)
    .bind(stored.os_version)
    .bind(stored.arch)
    .bind(stored.ci)
    .bind(stored.shell)
    .bind(stored.event_type)
    .bind(stored.event_category)
    .bind(stored.event_data)
    .bind(stored.sdk_version)
    .bind(stored.transmission_timestamp)
    .bind(stored.batch_size)
    .bind(stored.retry_count)
    .bind(stored.received_at)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            "database_error".to_string(),
            format!("Failed to insert event: {}", e),
        )
    })?;

    Ok(())
}

/// Check if schema version is supported
fn is_supported_schema(version: &str) -> bool {
    // Currently only support 1.x.x
    version.starts_with("1.")
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
    use chrono::Utc;

    #[test]
    fn test_is_supported_schema_valid() {
        assert!(is_supported_schema("1.0.0"));
        assert!(is_supported_schema("1.1.0"));
        assert!(is_supported_schema("1.99.99"));
    }

    #[test]
    fn test_is_supported_schema_invalid() {
        assert!(!is_supported_schema("2.0.0"));
        assert!(!is_supported_schema("0.9.0"));
        assert!(!is_supported_schema("invalid"));
    }

    #[test]
    fn test_event_batch_validation() {
        // Valid batch with one event
        let event = create_test_event();
        let batch = EventBatch {
            events: vec![event],
        };
        assert_eq!(batch.events.len(), 1);

        // Empty batch (should be rejected by handler)
        let empty_batch = EventBatch { events: vec![] };
        assert_eq!(empty_batch.events.len(), 0);
    }

    #[test]
    fn test_user_id_validation() {
        // Valid user IDs
        assert!("client_abc123".starts_with("client_"));
        assert!("client_550e8400e29b41d4a716446655440000a1b2c3d4e5f6".starts_with("client_"));

        // Invalid user IDs
        assert!(!"anon_abc123".starts_with("client_"));
        assert!(!"user_abc123".starts_with("client_"));
        assert!(!"abc123".starts_with("client_"));
    }

    fn create_test_event() -> IncomingEvent {
        use crate::models::*;

        IncomingEvent {
            schema_version: "1.0.0".to_string(),
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            service: ServiceInfo {
                name: "test-service".to_string(),
                version: "1.0.0".to_string(),
                language: "rust".to_string(),
                language_version: Some("1.75.0".to_string()),
            },
            user_id: "client_test123".to_string(),
            session_id: Some(Uuid::new_v4().to_string()),
            environment: Environment {
                os: "linux".to_string(),
                os_version: Some("6.5.0".to_string()),
                arch: Some("x86_64".to_string()),
                ci: Some(false),
                shell: Some("bash".to_string()),
            },
            event: EventData {
                event_type: "command".to_string(),
                category: Some("test".to_string()),
                data: json!({"success": true}),
            },
            metadata: Metadata {
                sdk_version: "0.1.0".to_string(),
                transmission_timestamp: Utc::now(),
                batch_size: 1,
                retry_count: 0,
            },
        }
    }
}

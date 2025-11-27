//! Sync client for pushing events to telemetry-kit.dev

use super::{
    auth::HmacAuth, config::SyncConfig, retry::RetryStrategy, ErrorResponse, SyncResponse,
};
use crate::error::{Result, TelemetryError};
use crate::event::EventBatch;
use chrono::Utc;
use reqwest::{header::HeaderMap, Client as HttpClient, StatusCode};
use std::time::Duration;
use uuid::Uuid;

const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");
const SCHEMA_VERSION: &str = "1.0.0";

/// Sync client for pushing events to the server
pub struct SyncClient {
    config: SyncConfig,
    auth: HmacAuth,
    http_client: HttpClient,
    retry_strategy: RetryStrategy,
}

impl SyncClient {
    /// Create a new sync client
    pub fn new(config: SyncConfig) -> Result<Self> {
        let auth = HmacAuth::new(config.secret.clone());
        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        let retry_strategy = RetryStrategy::new(config.max_retries, 1000);

        Ok(Self {
            config,
            auth,
            http_client,
            retry_strategy,
        })
    }

    /// Sync a batch of events to the server
    ///
    /// This method handles:
    /// - HMAC signature generation
    /// - DNT (Do Not Track) checking
    /// - Retry logic with exponential backoff
    /// - Rate limit handling
    pub async fn sync(&self, batch: EventBatch) -> Result<SyncResponse> {
        // Check DNT header if enabled
        if self.config.respect_dnt && is_dnt_enabled() {
            return Err(TelemetryError::Other(
                "DNT (Do Not Track) is enabled, skipping sync".to_string(),
            ));
        }

        let mut retry_count = 0;

        loop {
            match self.try_sync(&batch, retry_count).await {
                Ok(response) => return Ok(response),
                Err(e) if e.is_retryable() && self.retry_strategy.should_retry(retry_count) => {
                    let delay = self.retry_strategy.delay_for(retry_count);
                    tokio::time::sleep(delay).await;
                    retry_count += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Attempt to sync events (single attempt, no retry)
    async fn try_sync(&self, batch: &EventBatch, _retry_count: u32) -> Result<SyncResponse> {
        if batch.is_empty() {
            return Ok(SyncResponse::Success {
                accepted: 0,
                rejected: 0,
                message: "No events to sync".to_string(),
            });
        }

        // Generate timestamp and nonce
        let timestamp = Utc::now().timestamp().to_string();
        let nonce = Uuid::new_v4().to_string();

        // Serialize body
        let body = serde_json::to_string(batch)?;

        // Calculate HMAC signature
        let signature = self.auth.sign(&timestamp, &nonce, &body);

        // Build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("X-Signature", signature.parse().unwrap());
        headers.insert("X-Timestamp", timestamp.parse().unwrap());
        headers.insert("X-Nonce", nonce.parse().unwrap());
        headers.insert("X-Batch-Size", batch.size().to_string().parse().unwrap());
        headers.insert(
            "X-SDK-Version",
            format!("telemetry-kit-rust/{}", SDK_VERSION)
                .parse()
                .unwrap(),
        );
        headers.insert("X-Schema-Version", SCHEMA_VERSION.parse().unwrap());

        // GNU Terry Pratchett - keeping his memory alive in the overhead
        // See: http://www.gnuterrypratchett.com/
        headers.insert("X-Clacks-Overhead", "GNU Terry Pratchett".parse().unwrap());

        // Send request
        let url = self.config.ingestion_url();
        let response = self
            .http_client
            .post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();

        // Handle response based on status code
        match status {
            StatusCode::OK => {
                let sync_response: SyncResponse = response.json().await?;
                Ok(sync_response)
            }

            StatusCode::MULTI_STATUS => {
                let sync_response: SyncResponse = response.json().await?;
                Ok(sync_response)
            }

            StatusCode::TOO_MANY_REQUESTS => {
                let error_response: ErrorResponse = response.json().await?;
                let retry_after = error_response.retry_after.unwrap_or(60);
                Err(TelemetryError::RateLimitExceeded { retry_after })
            }

            StatusCode::BAD_REQUEST
            | StatusCode::UNAUTHORIZED
            | StatusCode::FORBIDDEN
            | StatusCode::CONFLICT
            | StatusCode::PAYLOAD_TOO_LARGE
            | StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response: ErrorResponse = response.json().await?;
                Err(TelemetryError::ServerError {
                    status: status.as_u16(),
                    message: format!("{}: {}", error_response.error.code, error_response.error.message),
                })
            }

            _ if status.is_server_error() => {
                let error_text = response.text().await.unwrap_or_default();
                Err(TelemetryError::ServerError {
                    status: status.as_u16(),
                    message: error_text,
                })
            }

            _ => {
                let error_text = response.text().await.unwrap_or_default();
                Err(TelemetryError::Other(format!(
                    "Unexpected status code {}: {}",
                    status, error_text
                )))
            }
        }
    }

    /// Get the sync configuration
    pub fn config(&self) -> &SyncConfig {
        &self.config
    }
}

/// Check if DNT (Do Not Track) is enabled
///
/// Checks the DNT environment variable
fn is_dnt_enabled() -> bool {
    std::env::var("DNT")
        .ok()
        .and_then(|v| v.parse::<u8>().ok())
        .map(|v| v == 1)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> SyncConfig {
        SyncConfig::builder()
            .org_id("550e8400-e29b-41d4-a716-446655440000")
            .unwrap()
            .app_id("7c9e6679-7425-40de-944b-e07fc1f90ae7")
            .unwrap()
            .token("tk_test_token")
            .secret("test_secret")
            .build()
            .unwrap()
    }

    #[test]
    fn test_client_creation() {
        let config = create_test_config();
        let client = SyncClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_empty_batch() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let config = create_test_config();
            let client = SyncClient::new(config).unwrap();

            let batch = EventBatch::new(vec![]);
            let result = client.try_sync(&batch, 0).await;

            assert!(result.is_ok());
            let response = result.unwrap();
            assert_eq!(response.accepted(), 0);
        });
    }

    #[test]
    fn test_dnt_detection() {
        // DNT not set
        std::env::remove_var("DNT");
        assert!(!is_dnt_enabled());

        // DNT = 0
        std::env::set_var("DNT", "0");
        assert!(!is_dnt_enabled());

        // DNT = 1
        std::env::set_var("DNT", "1");
        assert!(is_dnt_enabled());

        // Cleanup
        std::env::remove_var("DNT");
    }
}

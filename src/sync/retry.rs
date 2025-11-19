//! Retry strategy with exponential backoff

use rand::Rng;
use std::time::Duration;

/// Retry strategy for sync operations
#[derive(Debug, Clone)]
pub struct RetryStrategy {
    max_retries: u32,
    base_delay_ms: u64,
}

impl RetryStrategy {
    /// Create a new retry strategy
    ///
    /// # Arguments
    /// * `max_retries` - Maximum number of retry attempts
    /// * `base_delay_ms` - Base delay in milliseconds (default: 1000)
    pub fn new(max_retries: u32, base_delay_ms: u64) -> Self {
        Self {
            max_retries,
            base_delay_ms,
        }
    }

    /// Create default retry strategy (5 retries, 1 second base delay)
    pub fn default_strategy() -> Self {
        Self::new(5, 1000)
    }

    /// Calculate delay for a given retry attempt
    ///
    /// Uses exponential backoff with jitter:
    /// `delay = base_delay * (2 ^ retry_count) + jitter`
    ///
    /// # Arguments
    /// * `retry_count` - Current retry attempt (0-indexed)
    ///
    /// # Returns
    /// Duration to wait before retrying
    pub fn delay_for(&self, retry_count: u32) -> Duration {
        let exponential_delay = self.base_delay_ms * 2_u64.pow(retry_count);
        let jitter = rand::thread_rng().gen_range(0..1000); // 0-1000ms jitter

        Duration::from_millis(exponential_delay + jitter)
    }

    /// Check if we should retry
    pub fn should_retry(&self, retry_count: u32) -> bool {
        retry_count < self.max_retries
    }

    /// Get maximum retries
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self::default_strategy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_delay() {
        let strategy = RetryStrategy::new(5, 1000);

        // First retry: ~1-2 seconds (1000ms base + jitter)
        let delay0 = strategy.delay_for(0);
        assert!(delay0.as_millis() >= 1000 && delay0.as_millis() < 2000);

        // Second retry: ~2-3 seconds (2000ms base + jitter)
        let delay1 = strategy.delay_for(1);
        assert!(delay1.as_millis() >= 2000 && delay1.as_millis() < 3000);

        // Third retry: ~4-5 seconds (4000ms base + jitter)
        let delay2 = strategy.delay_for(2);
        assert!(delay2.as_millis() >= 4000 && delay2.as_millis() < 5000);
    }

    #[test]
    fn test_should_retry() {
        let strategy = RetryStrategy::new(3, 1000);

        assert!(strategy.should_retry(0));
        assert!(strategy.should_retry(1));
        assert!(strategy.should_retry(2));
        assert!(!strategy.should_retry(3));
        assert!(!strategy.should_retry(4));
    }

    #[test]
    fn test_default_strategy() {
        let strategy = RetryStrategy::default();
        assert_eq!(strategy.max_retries(), 5);
    }
}

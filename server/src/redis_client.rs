//! Redis client wrapper

use redis::aio::ConnectionManager;
use redis::{AsyncCommands, RedisError};

#[derive(Clone)]
pub struct RedisClient {
    conn: ConnectionManager,
}

impl RedisClient {
    pub async fn new(url: &str) -> Result<Self, RedisError> {
        let client = redis::Client::open(url)?;
        let conn = ConnectionManager::new(client).await?;

        Ok(Self { conn })
    }

    /// Get a value from Redis
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, RedisError>
    where
        T: redis::FromRedisValue,
    {
        let mut conn = self.conn.clone();
        conn.get(key).await
    }

    /// Set a value in Redis with expiry
    pub async fn set_ex<T>(&self, key: &str, value: T, seconds: u64) -> Result<(), RedisError>
    where
        T: redis::ToRedisArgs + Send + Sync,
    {
        let mut conn = self.conn.clone();
        conn.set_ex(key, value, seconds).await
    }

    /// Increment a counter and set expiry if it doesn't exist
    pub async fn incr_with_expiry(
        &self,
        key: &str,
        expiry_seconds: u64,
    ) -> Result<i64, RedisError> {
        let mut conn = self.conn.clone();

        // Use a pipeline for atomic operation
        let count: i64 = redis::pipe()
            .atomic()
            .incr(key, 1)
            .expire(key, expiry_seconds as i64)
            .ignore()
            .query_async(&mut conn)
            .await?;

        Ok(count)
    }

    /// Get current count for rate limiting
    #[allow(dead_code)]
    pub async fn get_count(&self, key: &str) -> Result<i64, RedisError> {
        let mut conn = self.conn.clone();
        let count: Option<i64> = conn.get(key).await?;
        Ok(count.unwrap_or(0))
    }
}

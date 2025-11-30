use sqlx::{PgPool};
use redis::{Client};
use anyhow::Result;
use super::error::UrlShortenerError;
use redis::AsyncCommands;

pub struct UrlShortener {
    pub pool: PgPool,
    pub redis: Client,
}

impl UrlShortener {
    pub async fn new(pool: PgPool, redis: Client) -> Result<Self, UrlShortenerError> {
        Ok(Self { pool, redis })
    }
    pub async fn create_short_url(&self, long_url: &str) -> Result<String, UrlShortenerError> {
        let id = sqlx::query!(
            r#"INSERT INTO urls (long_url) VALUES ($1) RETURNING id"#,
            long_url
        )
        .fetch_one(&self.pool)
        .await?.id as u64;
        let short = base62::encode(id);
        let result = sqlx::query!(
            r#"UPDATE urls SET short_code = $1 WHERE id = $2"#,
            &short,
            id as i64
        )
        .execute(&self.pool)
        .await?;
        if result.rows_affected() == 0 {
            return Err(UrlShortenerError::SQLXError(sqlx::Error::RowNotFound));
        }
        let mut conn = self.redis.get_multiplexed_async_connection().await?;
        let _: () = conn.set_ex(short.clone(), long_url, 60 * 60).await?;
        Ok(short)
    }
    pub async fn resolve_short_url(&self, short_url: &str) -> Result<String, UrlShortenerError> {
        let mut conn = self.redis.get_multiplexed_async_connection().await?;
        let redis_long_url: Option<String> = conn.get(short_url).await?;
        if let Some(url) = redis_long_url {
            return Ok(url);
        }
        let sqlx_long_url = sqlx::query!(
            r#"SELECT long_url FROM urls WHERE short_code = $1"#,
            short_url
        )
        .fetch_one(&self.pool)
        .await?;
        let _: () = conn.set_ex(short_url, sqlx_long_url.long_url.clone(), 60 * 60).await?;
        Ok(sqlx_long_url.long_url)
    }
}
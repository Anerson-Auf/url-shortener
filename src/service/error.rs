use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceInitError {
    #[error("DATABASE_URL is not set")]
    DatabaseUrlNotSet,
    #[error("REDIS_URL is not set")]
    RedisUrlNotSet,
    #[error("Failed to connect to database")]
    DatabaseConnectionError(#[from] sqlx::Error),
    #[error("Failed to connect to Redis")]
    RedisConnectionError(#[from] redis::RedisError),
    #[error("Failed to get environment variable: {0}")]
    GetEnvError(#[from] std::env::VarError),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Failed from UrlShortenerError")]
    UrlShortenerError(#[from] UrlShortenerError),
}

#[derive(Error, Debug)]
pub enum UrlShortenerError {
    #[error("Failed to encode URL: {0}")]
    EncodeURL(base62::EncodeError),
    #[error("Failed to decode URL: {0}")]
    DecodeURL(base62::DecodeError),
    #[error("Database error: {0}")]
    SQLXError(#[from] sqlx::Error),
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
}

use sqlx::PgPool;
use redis::Client;
use chrono::{DateTime, Utc};
use serde::{Deserialize};

use super::url_shortener::UrlShortener;

pub struct Service {
    pub pool: PgPool,
    pub redis: Client,
    pub url_shortener: UrlShortener,
}

#[derive(Deserialize)]
pub struct ShortUrlPayload {
    pub url: String,
}

pub struct ShortUrlResponse {
    pub short_url: String,
    pub date: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}
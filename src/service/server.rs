use super::{
    types::Service,
    error::ServiceInitError,
    url_shortener::UrlShortener,
};
use axum_server;
use anyhow::Result;
use sqlx::PgPool;
use redis::Client;
use axum::{
    Router,
};
use std::{str::FromStr, sync::Arc};
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use super::handlers::router;

impl Service {
    pub async fn new() -> Result<Arc<Self>, ServiceInitError> {
        dotenvy::dotenv().ok();
        let database = std::env::var("DATABASE_URL")?;
        let redis_url = std::env::var("REDIS_URL")?;
        let pool = PgPool::connect(&database).await?;
        let redis = Client::open(redis_url)?;

        let url_shortener = UrlShortener::new(pool.clone(), redis.clone()).await?;
        Ok(Arc::new(Self {
            pool,
            redis,
            url_shortener,
        }))
    }
    pub fn router(self: &Arc<Self>) -> Router {
        router().with_state(self.clone())
    }
    pub async fn start(self: Arc<Self>) -> Result<(), ServiceInitError> {
        dotenvy::dotenv().ok();
        let ip = std::env::var("IP")?;
        let port = std::env::var("PORT")?;
        let ip_addr = IpAddr::V4(Ipv4Addr::from_str(ip.as_str()).unwrap_or(Ipv4Addr::LOCALHOST));
        tracing::info!("Starting server on {}:{}", ip_addr, port);
        axum_server::bind(SocketAddr::new(ip_addr, port.parse::<u16>().unwrap_or(8080))).serve(self.router().into_make_service()).await?;
        Ok(())
    }
}
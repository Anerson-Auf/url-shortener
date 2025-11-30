use axum::{Json, http::StatusCode, extract::Path, response::Redirect};
use super::types::{ShortUrlPayload, Service};
use axum::extract::State;
use std::sync::Arc;
use axum::routing::{get, post};
use axum::Router;
use tower_http::services::ServeDir;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer, key_extractor::KeyExtractor, GovernorError};
use std::time::Duration;
use axum::http::Request;
use std::net::SocketAddr;

#[allow(unused)]
pub async fn get_config() -> (StatusCode, Json<serde_json::Value>) {
    dotenvy::dotenv().ok();
    let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    let config = serde_json::json!({
        "base_url": base_url,
    });
    (StatusCode::OK, Json(config))
}

pub async fn short_url(
    State(service): State<Arc<Service>>,
    Json(payload): Json<ShortUrlPayload>,
) -> (StatusCode, Json<String>) {
    match service.url_shortener.create_short_url(&payload.url).await {
        Ok(short) => (StatusCode::OK, Json(short)),
        Err(e) => {
            tracing::error!("Error creating short URL: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Error: {}", e)))
        },
    }
}

pub async fn redirect(
    State(service): State<Arc<Service>>,
    Path(short_url): Path<String>,
) -> Redirect {
    tracing::info!("Looking up short_code: {}", short_url);
    let url = service.url_shortener.resolve_short_url(&short_url).await;
    match url {
        Ok(url) => {
            let full_url = if url.starts_with("http://") || url.starts_with("https://") {
                url
            } else {
                format!("https://{}", url)
            };
            tracing::info!("Redirecting to: {}", full_url);  
            Redirect::to(&full_url)
        },
        Err(e) => {
            tracing::error!("Error redirecting to: {} with error: {}", short_url, e);
            Redirect::to("/")
        },
    }
}

#[derive(Clone)]
struct AxumIpKeyExtractor;

impl KeyExtractor for AxumIpKeyExtractor {
    type Key = String;

    fn extract<T>(&self, req: &Request<T>) -> Result<Self::Key, tower_governor::GovernorError> {
        if let Some(addr) = req.extensions().get::<SocketAddr>() {
            Ok(addr.ip().to_string())
        } else if let Some(forwarded) = req.headers().get("x-forwarded-for") {
            forwarded.to_str()
                .ok()
                .and_then(|s| s.split(',').next().map(|ip| ip.trim().to_string()))
                .ok_or(GovernorError::UnableToExtractKey)
        } else if let Some(real_ip) = req.headers().get("x-real-ip") {
            real_ip.to_str()
                .ok()
                .map(|s| s.to_string())
                .ok_or(GovernorError::UnableToExtractKey)
        } else {
            Ok("unknown".to_string())
        }
    }
}

pub fn router() -> Router<std::sync::Arc<Service>> {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(10)
            .burst_size(20)
            .key_extractor(AxumIpKeyExtractor)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(interval);
            tracing::info!("rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    let api_routes = Router::new()
        .route("/api/config", get(get_config))
        .route("/api/short-url", post(short_url))
        .route("/r/{code}", get(redirect))
        .layer(GovernorLayer::new(governor_conf));

    Router::new()
        .merge(api_routes)
        .nest_service("/static", ServeDir::new("static"))
        .fallback_service(ServeDir::new("static"))
}
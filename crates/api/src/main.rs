//! API Server - Axum HTTP server for video downloader.
//!
//! This is the main entry point for the API deployment.

use std::net::{IpAddr, SocketAddr};
use std::num::NonZeroU32;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use governor::{clock::DefaultClock, state::keyed::DefaultKeyedStateStore, Quota, RateLimiter};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod auth;
mod config;
mod routes;

use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub jwt_secret: String,
    pub whop_webhook_secret: String,
}

type KeyedLimiter = RateLimiter<IpAddr, DefaultKeyedStateStore<IpAddr>, DefaultClock>;

static RATE_LIMIT_429_TOTAL: AtomicU64 = AtomicU64::new(0);
static RATE_LIMIT_403_TOTAL: AtomicU64 = AtomicU64::new(0);

fn make_rate_limiter() -> Arc<KeyedLimiter> {
    let quota = Quota::with_period(Duration::from_secs(6))
        .expect("quota period should be valid")
        .allow_burst(NonZeroU32::new(5).expect("burst size should be non-zero"));
    Arc::new(RateLimiter::keyed(quota))
}

fn extract_ip(request: &Request) -> Option<IpAddr> {
    if let Some(ip) = request
        .headers()
        .get("cf-connecting-ip")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse().ok())
    {
        return Some(ip);
    }

    request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|connect_info| connect_info.0.ip())
}

async fn rate_limit_middleware(
    State(limiter): State<Arc<KeyedLimiter>>,
    request: Request,
    next: axum::middleware::Next,
) -> Response {
    let path = request.uri().path().to_string();
    let Some(ip) = extract_ip(&request) else {
        let blocked_total = RATE_LIMIT_403_TOTAL.fetch_add(1, Ordering::Relaxed) + 1;
        warn!(
            blocked_total,
            path = %path,
            "rate limit middleware rejected request without identifiable client IP"
        );
        return (
            StatusCode::FORBIDDEN,
            axum::Json(json!({ "error": "Unable to identify client IP" })),
        )
            .into_response();
    };

    if limiter.check_key(&ip).is_err() {
        let limited_total = RATE_LIMIT_429_TOTAL.fetch_add(1, Ordering::Relaxed) + 1;
        warn!(
            limited_total,
            path = %path,
            client_ip = %ip,
            "rate limit exceeded"
        );
        return (
            StatusCode::TOO_MANY_REQUESTS,
            axum::Json(json!({ "error": "Rate limit exceeded" })),
        )
            .into_response();
    }

    next.run(request).await
}

fn build_app(app_state: AppState, limiter: Arc<KeyedLimiter>) -> Router {
    let protected_api = Router::new()
        .route(
            "/api/extract",
            post(routes::extract_handler).route_layer(middleware::from_fn_with_state(
                limiter,
                rate_limit_middleware,
            )),
        )
        .route("/api/stream", get(routes::stream_handler))
        .route("/api/stream/muxed", get(routes::muxed_stream_handler))
        .route("/api/batch", get(routes::batch_handler))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth::jwt_middleware::jwt_auth_middleware,
        ));

    Router::new()
        .route("/health", get(routes::health_check))
        .route("/openapi.json", get(routes::openapi_handler))
        .route("/bm.js", get(routes::bm_js_handler))
        .route("/userscript", get(routes::userscript_handler))
        .route("/api/webhooks/whop", post(routes::whop_webhook_handler))
        .merge(protected_api)
        .with_state(app_state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let config = Config::from_env()?;
    let extractor_path = Path::new(&config.extractor_dir);
    if extractor_path.is_file() {
        info!(
            "Initializing extractor pool from external bundle {}",
            config.extractor_dir
        );
        extractor::init(Some(extractor_path)).await?;
    } else {
        info!(
            "Extractor path '{}' is not a file; using embedded extractor bundle",
            config.extractor_dir
        );
        extractor::init(None).await?;
    }

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    info!("Database pool connected");

    let app_state = AppState {
        db_pool,
        jwt_secret: config.jwt_secret,
        whop_webhook_secret: config.whop_webhook_secret,
    };

    info!("Starting API server on port {}", config.port);
    let limiter = make_rate_limiter();
    let app = build_app(app_state, limiter);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("API server listening on {}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app_state() -> AppState {
        let db_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_lazy("postgres://postgres:postgres@localhost:5432/postgres")
            .expect("valid postgres url for lazy pool");

        AppState {
            db_pool,
            jwt_secret: "test-secret".to_string(),
            whop_webhook_secret: "test-whop-secret".to_string(),
        }
    }

    async fn spawn_test_server(with_connect_info: bool) -> SocketAddr {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind test listener");
        let addr = listener.local_addr().expect("local addr");
        let app = build_app(test_app_state(), make_rate_limiter());

        tokio::spawn(async move {
            if with_connect_info {
                let _ = axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
                    .await;
            } else {
                let _ = axum::serve(listener, app.into_make_service()).await;
            }
        });

        tokio::time::sleep(Duration::from_millis(50)).await;
        addr
    }

    #[tokio::test]
    async fn test_rate_limit_returns_429_on_sixth_extract_request() {
        let addr = spawn_test_server(true).await;
        let client = reqwest::Client::new();
        let mut statuses = Vec::new();

        for _ in 0..6 {
            let status = client
                .post(format!("http://{addr}/api/extract"))
                .header("Content-Type", "application/json")
                .body("{}")
                .send()
                .await
                .expect("request should succeed")
                .status();
            statuses.push(status);
        }

        assert!(
            statuses
                .iter()
                .take(5)
                .all(|status| *status != StatusCode::TOO_MANY_REQUESTS),
            "first 5 requests should consume burst tokens, got: {statuses:?}"
        );
        assert_eq!(statuses[5], StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn test_rate_limit_returns_403_when_no_ip_context_available() {
        let addr = spawn_test_server(false).await;
        let client = reqwest::Client::new();

        let response = client
            .post(format!("http://{addr}/api/extract"))
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        let body_text = response.text().await.expect("body text");
        let body: serde_json::Value =
            serde_json::from_str(&body_text).expect("valid json response");
        assert_eq!(body["error"], "Unable to identify client IP");
    }
}

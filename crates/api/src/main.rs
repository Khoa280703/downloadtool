//! API Server - Axum HTTP server for video downloader.
//!
//! This is the main entry point for the API deployment.

use std::net::{IpAddr, SocketAddr};
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use axum::{
    extract::{ConnectInfo, MatchedPath, Request, State},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use governor::{clock::DefaultClock, state::keyed::DefaultKeyedStateStore, Quota, RateLimiter};
use object_store::backend_factory::{build_storage_backend, StorageBackendConfig};
use proxy::init_global_proxy_pool;
use queue::redis_streams::RedisStreamsQueue;
use queue::JobQueuePublisher;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultOnFailure, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::{info, info_span, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod auth;
mod config;
mod limit_profiles;
mod routes;
mod services;

use config::{Config, MuxArtifactBackend};
use job_system::JobProgressStore;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub jwt_secret: String,
    pub whop_webhook_secret: String,
    pub job_control_plane: Arc<services::job_control_plane::JobControlPlaneService>,
    pub job_progress_store: Arc<JobProgressStore>,
    pub storage_ticket_service: Arc<services::storage_ticket_service::StorageTicketService>,
    pub mux_direct_download: bool,
}

type KeyedLimiter = RateLimiter<IpAddr, DefaultKeyedStateStore<IpAddr>, DefaultClock>;

static RATE_LIMIT_429_TOTAL: AtomicU64 = AtomicU64::new(0);
static RATE_LIMIT_403_TOTAL: AtomicU64 = AtomicU64::new(0);

async fn run_app_migrations(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    let mut migrator = sqlx::migrate::Migrator::new(resolve_app_migrations_dir()).await?;
    migrator.set_ignore_missing(true);
    migrator.run(pool).await?;
    Ok(())
}

fn resolve_app_migrations_dir() -> PathBuf {
    let compile_time_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("app-migrations");
    if compile_time_dir.exists() {
        return compile_time_dir;
    }

    let runtime_dir = PathBuf::from("/app/app-migrations");
    if runtime_dir.exists() {
        return runtime_dir;
    }

    if let Ok(executable_path) = std::env::current_exe() {
        if let Some(bin_dir) = executable_path.parent() {
            let sibling_dir = bin_dir.join("app-migrations");
            if sibling_dir.exists() {
                return sibling_dir;
            }
        }
    }

    compile_time_dir
}

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

fn build_app(
    app_state: AppState,
    limiter: Arc<KeyedLimiter>,
    extract_rate_limit_enabled: bool,
) -> Router {
    let extract_route = if extract_rate_limit_enabled {
        post(routes::extract_handler).route_layer(middleware::from_fn_with_state(
            limiter,
            rate_limit_middleware,
        ))
    } else {
        post(routes::extract_handler)
    };

    let protected_api = Router::new()
        .route("/api/extract", extract_route)
        .route("/api/stream", get(routes::stream_handler))
        .route("/api/jobs", post(routes::create_job_handler))
        .route("/api/jobs/{job_id}", get(routes::job_status_handler))
        .route("/api/jobs/{job_id}/events", get(routes::job_events_handler))
        .route(
            "/api/jobs/{job_id}/file-ticket",
            get(routes::job_file_ticket_handler),
        )
        .route("/api/jobs/{job_id}/file", get(routes::job_file_handler))
        .route(
            "/api/jobs/{job_id}/release",
            post(routes::release_job_handler),
        )
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
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str)
                        .unwrap_or_else(|| request.uri().path());

                    info_span!(
                        "http_request",
                        method = %request.method(),
                        matched_path = %matched_path
                    )
                })
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                )
                .on_failure(
                    DefaultOnFailure::new()
                        .level(Level::ERROR)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env (if present) so local `cargo run` works without manual `source .env`.
    let _ = dotenvy::dotenv();

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
    services::auth_schema_bootstrap::ensure_better_auth_schema(&db_pool).await?;
    run_app_migrations(&db_pool).await?;
    let proxy_db_pool = if config.proxy_database_url == config.database_url {
        db_pool.clone()
    } else {
        PgPoolOptions::new()
            .max_connections(3)
            .connect(&config.proxy_database_url)
            .await?
    };
    proxy::ensure_proxy_schema(&proxy_db_pool).await?;
    init_global_proxy_pool(
        proxy_db_pool,
        &config.proxy_redis_url,
        config.proxy_quarantine_ttl_secs,
    )
    .await?;
    info!("Database pool connected");
    let durable_job_repository = Arc::new(job_system::JobRepository::new(db_pool.clone()));
    let queue_publisher = Arc::new(RedisStreamsQueue::new(
        &config.redis_url,
        &config.mux_queue_stream,
        "mux-workers",
        "api-publisher",
    )?) as Arc<dyn JobQueuePublisher>;
    let storage_backend = if config.mux_artifact_backend == MuxArtifactBackend::LocalFs {
        None
    } else {
        Some(
            build_storage_backend(&StorageBackendConfig {
                backend: config
                    .mux_artifact_backend
                    .storage_backend_name()
                    .to_string(),
                local_dir: Path::new("/tmp/downloadtool-api-object-store-fallback").to_path_buf(),
                s3_bucket: config.s3_bucket.clone(),
                s3_region: config.s3_region.clone(),
                s3_endpoint: config.s3_endpoint.clone(),
                s3_access_key_id: config.s3_access_key_id.clone(),
                s3_secret_access_key: config.s3_secret_access_key.clone(),
                s3_force_path_style: config.s3_force_path_style,
                multipart_part_size_bytes: 8 * 1024 * 1024,
            })
            .await?,
        )
    };
    let job_control_plane = services::job_control_plane::JobControlPlaneService::new(
        durable_job_repository,
        queue_publisher,
        config.mux_job_max_attempts,
    );
    let storage_ticket_service = services::storage_ticket_service::StorageTicketService::new(
        storage_backend,
        config.mux_file_ticket_ttl_secs,
    );
    let job_progress_store = Arc::new(JobProgressStore::new(&config.redis_url)?);
    let app_state = AppState {
        db_pool,
        jwt_secret: config.jwt_secret,
        whop_webhook_secret: config.whop_webhook_secret,
        job_control_plane,
        job_progress_store,
        storage_ticket_service,
        mux_direct_download: config.mux_direct_download,
    };

    info!("Starting API server on port {}", config.port);
    let limiter = make_rate_limiter();
    info!(
        "/api/extract rate limit: {}",
        if config.extract_rate_limit_enabled {
            "enabled"
        } else {
            "disabled"
        }
    );
    info!(
        mux_artifact_backend = config.mux_artifact_backend.storage_backend_name(),
        mux_direct_download = config.mux_direct_download,
        "Mux job runtime configuration loaded"
    );
    let app = build_app(app_state, limiter, config.extract_rate_limit_enabled);

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
        let durable_job_repository = Arc::new(job_system::JobRepository::new(db_pool.clone()));
        let job_control_plane = services::job_control_plane::JobControlPlaneService::new(
            durable_job_repository,
            Arc::new(
                RedisStreamsQueue::new(
                    "redis://127.0.0.1:6379",
                    "mux_jobs",
                    "mux-workers",
                    "api-test-publisher",
                )
                .expect("test redis queue should initialize"),
            ),
            3,
        );

        AppState {
            db_pool,
            jwt_secret: "test-secret".to_string(),
            whop_webhook_secret: "test-whop-secret".to_string(),
            job_control_plane,
            job_progress_store: Arc::new(
                JobProgressStore::new("redis://127.0.0.1:6379")
                    .expect("test job progress store should initialize"),
            ),
            storage_ticket_service: services::storage_ticket_service::StorageTicketService::new(
                None, 900,
            ),
            mux_direct_download: false,
        }
    }

    async fn spawn_test_server(with_connect_info: bool) -> SocketAddr {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind test listener");
        let addr = listener.local_addr().expect("local addr");
        let app = build_app(test_app_state(), make_rate_limiter(), true);

        tokio::spawn(async move {
            if with_connect_info {
                let _ = axum::serve(
                    listener,
                    app.into_make_service_with_connect_info::<SocketAddr>(),
                )
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

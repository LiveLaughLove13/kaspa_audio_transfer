mod config;
mod jobs;
mod process;
mod routes;

use axum::{extract::DefaultBodyLimit, http::{header, HeaderValue, Method}, routing::{get, post}, Json, Router};
use serde::Serialize;
use std::path::PathBuf;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::cors::{Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::routes::{
    estimate_handler, job_status_handler, receive_get_handler, receive_handler, send_async_handler,
    send_handler, send_job_status_handler, AppState, library_handler, library_file_handler,
    tx_accepting_block_hash_handler,
};

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env if present
    let _ = dotenvy::dotenv();

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::from_env();
    tracing::info!(
        kaspa_rpc_url = %cfg.kaspa_rpc_url,
        service_receive_address = %cfg.service_receive_address,
        backend_port = cfg.backend_port,
        upload_dir = ?cfg.upload_dir,
        kaspa_binary = %cfg.kaspa_binary,
        has_service_private_key = !cfg.service_private_key.trim().is_empty(),
        "Starting Kaspa file web backend"
    );

    let state = AppState::new(cfg.clone());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let frontend_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../frontend");
    tracing::info!(?frontend_dir, "Serving frontend directory");
    let frontend_index = frontend_dir.join("index.html");

    let frontend = ServeDir::new(frontend_dir)
        .not_found_service(ServeFile::new(frontend_index));

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/estimate", post(estimate_handler))
        .route("/api/send", post(send_handler))
        .route("/api/send_async", post(send_async_handler))
        .route("/api/send_jobs/:id", get(send_job_status_handler))
        .route(
            "/api/tx_accepting_block_hash",
            get(tx_accepting_block_hash_handler),
        )
        .route("/api/receive", post(receive_handler).get(receive_get_handler))
        .route("/api/library", get(library_handler))
        .route("/api/library/files/:name", get(library_file_handler))
        .route("/api/jobs/:id", get(job_status_handler))
        .with_state(state)
        .layer(match cfg.max_upload_bytes {
            Some(max) => DefaultBodyLimit::max(max),
            None => DefaultBodyLimit::disable(),
        })
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .fallback_service(frontend);

    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.backend_port));
    tracing::info!(%addr, "Listening");

    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!(%addr, "failed to bind TCP listener: {e}");
            return;
        }
    };

    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

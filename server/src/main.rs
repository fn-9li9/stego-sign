mod config;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use aws_sdk_s3::Client as S3Client;
use axum::Router;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

use config::{Env, cors, db, logger};
use services::storage;

// -- shared state available to every handler
#[derive(Clone)]
pub struct AppState {
    pub env: Arc<Env>,
    pub db: Arc<DatabaseConnection>,
    pub storage: Arc<S3Client>,
    pub signing_key: Arc<String>,
    pub verify_key: Arc<String>,
    pub app_base_url: String,
}

#[tokio::main]
async fn main() {
    logger::init();

    let env = Env::load();
    info!(host = %env.server_host, port = %env.server_port, "starting stego-server");

    // -- database
    let db_conn = db::connect(&env).await.expect("database connection failed");

    // -- minio client + ensure buckets exist
    let s3_client = storage::build_client(&env).await;
    // -- self-healing: buckets se crean automáticamente al arrancar
    storage::ensure_buckets(&s3_client, &db_conn, &env.storage_bucket_prefix)
        .await
        .expect("failed to ensure buckets");

    // -- signing keys (in prod: load from vault or env)
    // -- for now: hardcoded dev keys, replace with real key management
    let signing_key = Arc::new(env.signing_key.clone());
    let verify_key = Arc::new(env.verify_key.clone());

    let state = AppState {
        env: Arc::new(env.clone()),
        db: Arc::new(db_conn),
        storage: Arc::new(s3_client),
        signing_key,
        verify_key,
        app_base_url: env.app_base_url.clone(),
    };

    let app = Router::new()
        .merge(routes::v1::router())
        .with_state(state)
        .layer(cors::layer())
        .layer(
            TraceLayer::new_for_http().make_span_with(|req: &axum::http::Request<_>| {
                tracing::info_span!(
                    "http_request",
                    method = %req.method(),
                    uri    = %req.uri(),
                )
            }),
        );

    let addr = format!("{}:{}", env.server_host, env.server_port);
    info!(address = %addr, "server listening");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind address");

    axum::serve(listener, app).await.expect("server error");
}

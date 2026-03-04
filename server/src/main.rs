mod config;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use aws_sdk_s3::Client as S3Client;
use axum::Router;
use config::{Env, cors, db, logger};
use sea_orm::DatabaseConnection;
use services::storage;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

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

    // -- run migrations automatically on startup
    info!("running migrations...");
    let migrations_dir = std::path::Path::new("migrations");
    for migration in ["schema_files.sql", "schema_app.sql"] {
        let path = migrations_dir.join(migration);
        let sql = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("failed to read migration: {}", path.display()));
        sea_orm::ConnectionTrait::execute_unprepared(&db_conn, &sql)
            .await
            .unwrap_or_else(|e| panic!("failed to apply migration {}: {}", migration, e));
        info!(migration = %migration, "migration applied");
    }
    info!("all migrations applied");

    // -- storage client + ensure buckets exist
    let s3_client = storage::build_client(&env).await;
    storage::ensure_buckets(&s3_client, &db_conn, &env.storage_bucket_prefix)
        .await
        .expect("failed to ensure buckets");

    // -- signing keys
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

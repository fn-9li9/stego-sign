use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    handlers::{admin, documents, health, registry, sign, stats, storage_download, verify},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health_handler))
        .route("/api/v1/sign", post(sign::sign_handler))
        .route("/api/v1/verify", post(verify::verify_handler))
        .route("/api/v1/documents", get(documents::list_handler))
        .route("/api/v1/documents/{id}", get(documents::get_handler))
        .route(
            "/api/v1/documents/{id}/audit",
            get(documents::audit_handler),
        )
        .route(
            "/api/v1/documents/{id}/download",
            get(documents::download_handler),
        )
        // -- dev only
        .route("/api/v1/admin/keygen", get(admin::keygen_handler))
        .route("/api/v1/stats", get(stats::stats_handler))
        .route("/api/v1/registry", get(registry::registry_handler))
        .route(
            "/api/v1/storage/{bucket}/{*key}",
            get(storage_download::storage_download_handler),
        )
}

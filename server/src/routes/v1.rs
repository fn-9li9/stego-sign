use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    handlers::{admin, documents, health, sign, verify},
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
}

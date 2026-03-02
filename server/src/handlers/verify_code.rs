use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use tracing::info;

use crate::{AppState, models::response::ApiResponse, repositories::document as doc_repo};

// -- GET /api/v1/verify/code/:code
// -- lookup por verification_code — no genera audit_log
pub async fn verify_code_handler(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    let code = code.trim().to_uppercase();
    info!(code = %code, "verify by code requested");

    match doc_repo::find_by_verification_code(&state.db, &code).await {
        Ok(Some(doc)) => Json(ApiResponse::ok(serde_json::json!({
            "found":             true,
            "document_id":       doc.id,
            "filename":          doc.filename,
            "author":            doc.author,
            "signed_at":         doc.signed_at,
            "status":            doc.status,
            "hash":              doc.hash_sha256,
            "verification_code": doc.verification_code,
        })))
        .into_response(),
        Ok(None) => Json(ApiResponse::ok(serde_json::json!({
            "found":   false,
            "message": "invalid or unknown verification code",
        })))
        .into_response(),
        Err(e) => Json(ApiResponse::<()>::err(&e.to_string())).into_response(),
    }
}

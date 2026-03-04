use axum::{
    Json,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use tracing::info;
use uuid::Uuid;

use crate::{
    AppState,
    models::response::{ApiError, ApiResponse},
    repositories::{audit_log as audit_repo, document as doc_repo},
    services::storage,
};

// -- GET /api/v1/documents
pub async fn list_handler(State(state): State<AppState>) -> Response {
    info!("list documents requested");
    match doc_repo::list(&state.db, 100).await {
        Ok(docs) => (StatusCode::OK, Json(ApiResponse::ok(docs))).into_response(),
        Err(e) => ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
        .into_response(),
    }
}

// -- GET /api/v1/documents/{id}
pub async fn get_handler(State(state): State<AppState>, Path(id): Path<Uuid>) -> Response {
    info!(document_id = %id, "get document requested");
    match doc_repo::find_by_id(&state.db, id).await {
        Ok(Some(doc)) => (StatusCode::OK, Json(ApiResponse::ok(doc))).into_response(),
        Ok(None) => ApiError {
            status: StatusCode::NOT_FOUND,
            message: format!("document {} not found", id),
        }
        .into_response(),
        Err(e) => ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
        .into_response(),
    }
}

// -- GET /api/v1/documents/{id}/audit
pub async fn audit_handler(State(state): State<AppState>, Path(id): Path<Uuid>) -> Response {
    info!(document_id = %id, "audit log requested");
    match audit_repo::list_by_document(&state.db, id).await {
        Ok(logs) => (StatusCode::OK, Json(ApiResponse::ok(logs))).into_response(),
        Err(e) => ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
        .into_response(),
    }
}

// -- GET /api/v1/documents/{id}/download
// -- downloads the signed file from minio signatures bucket
pub async fn download_handler(State(state): State<AppState>, Path(id): Path<Uuid>) -> Response {
    info!(document_id = %id, "download requested");

    // -- find document to get the signed_key from metadata
    let doc = match doc_repo::find_by_id(&state.db, id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return ApiError {
                status: StatusCode::NOT_FOUND,
                message: format!("document {} not found", id),
            }
            .into_response();
        }
        Err(e) => {
            return ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: e.to_string(),
            }
            .into_response();
        }
    };

    // -- extract signed_key from metadata
    let signed_key = match doc
        .metadata
        .as_ref()
        .and_then(|m| m.get("signed_key"))
        .and_then(|v| v.as_str())
    {
        Some(k) => k.to_string(),
        None => {
            return ApiError {
                status: StatusCode::NOT_FOUND,
                message: "signed file key not found in document metadata".to_string(),
            }
            .into_response();
        }
    };

    // -- download from minio signatures bucket

    // bucket dinámico según prefix
    let bucket_signatures = storage::bucket_signatures(&state.env.storage_bucket_prefix);

    let file_bytes = match storage::download(&state.storage, &bucket_signatures, &signed_key).await
    {
        Ok(b) => b,
        Err(e) => {
            return ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("failed to retrieve file: {}", e),
            }
            .into_response();
        }
    };

    // -- build filename for download: signed_{original}
    let download_name = format!("signed_{}", doc.filename);

    info!(
        document_id  = %id,
        filename     = %download_name,
        size_bytes   = %file_bytes.len(),
        "file download served"
    );

    // -- return file as binary response with content-disposition header
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/octet-stream".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", download_name),
            ),
            (header::CONTENT_LENGTH, file_bytes.len().to_string()),
        ],
        file_bytes,
    )
        .into_response()
}

use axum::http::StatusCode;
use axum::{
    Json,
    extract::{Multipart, State},
    response::IntoResponse,
};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{
    AppState,
    models::{
        audit_log::CreateAuditLog,
        document::{CreateDocument, DocumentStatus},
        response::{ApiError, ApiResponse},
    },
    repositories::{audit_log as audit_repo, document as doc_repo, object as obj_repo},
    services::{crypto, qr, stego, storage, watermark},
};

pub async fn sign_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut file_bytes = None;
    let mut filename = String::from("unknown");
    let mut author = String::from("anonymous");
    let mut wm_pos = "bottom-right".to_string();

    // buckets dinámicos según prefix de entorno
    let bucket_uploads = storage::bucket_uploads(&state.env.storage_bucket_prefix);
    let bucket_signatures = storage::bucket_signatures(&state.env.storage_bucket_prefix);

    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("file") => {
                filename = field.file_name().unwrap_or("file").to_string();
                file_bytes = field.bytes().await.ok();
            }
            Some("author") => {
                author = field.text().await.unwrap_or_default();
            }
            Some("watermark_position") => {
                wm_pos = field
                    .text()
                    .await
                    .unwrap_or_else(|_| "bottom-right".to_string());
            }
            _ => {}
        }
    }

    let bytes = match file_bytes {
        Some(b) => b,
        None => {
            return ApiError {
                status: StatusCode::BAD_REQUEST,
                message: "no file provided".to_string(),
            }
            .into_response();
        }
    };

    info!(filename = %filename, author = %author, "sign request received");

    // -- 1. aplica watermark PRIMERO si es PDF (antes de hashear)
    // --    así el hash corresponde al contenido watermarkeado
    let watermarked_bytes = if filename.to_lowercase().ends_with(".pdf") {
        let verification_code_preview = qr::generate_verification_code();
        let verify_url = format!(
            "{}/verify?code={}",
            state.app_base_url, verification_code_preview
        );

        match qr::generate_qr_png(&verify_url, 256) {
            Ok(qr_png) => {
                let pos = watermark::WatermarkPosition::from_str(&wm_pos);
                match watermark::insert_qr_into_pdf(&bytes, &qr_png, pos, 72.0) {
                    Ok(wm) => {
                        info!(filename = %filename, "qr watermark applied before hashing");
                        (bytes::Bytes::from(wm), Some(verification_code_preview))
                    }
                    Err(e) => {
                        warn!(error = %e, "watermark failed, proceeding without it");
                        (bytes.clone(), None)
                    }
                }
            }
            Err(e) => {
                warn!(error = %e, "qr generation failed");
                (bytes.clone(), None)
            }
        }
    } else {
        (bytes.clone(), None)
    };

    let (content_bytes, wm_code) = watermarked_bytes;

    // -- 2. genera verification_code definitivo
    let verification_code = wm_code.unwrap_or_else(|| qr::generate_verification_code());

    // -- 3. hash del contenido watermarkeado (o del original si no es PDF)
    let hash = crypto::sha256(&content_bytes);

    // -- 4. firma el hash
    let signature = match crypto::sign(&hash, state.signing_key.as_str()) {
        Ok(s) => s,
        Err(e) => {
            return ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("signing failed: {}", e),
            }
            .into_response();
        }
    };

    // -- 5. embed stego payload en el contenido watermarkeado
    let document_id = Uuid::new_v4();
    let verify_url = format!("{}/verify?code={}", state.app_base_url, verification_code);
    let signed_bytes = match stego::embed(
        &content_bytes,
        &filename,
        document_id,
        &hash,
        &signature,
        &author,
    ) {
        Ok(b) => b,
        Err(e) => {
            error!(error = %e, "stego embed failed");
            return ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "embed failed".to_string(),
            }
            .into_response();
        }
    };

    let file_size = bytes.len() as i64;
    let signed_size = signed_bytes.len() as i64;

    // -- 6. upload original sin modificar
    let upload_key = format!("{}/{}", document_id, filename);

    if let Err(e) = storage::upload(
        &state.storage,
        &bucket_uploads,
        &upload_key,
        bytes.clone(),
        "application/octet-stream",
    )
    .await
    {
        error!(error = %e, "upload original failed");
        return ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "storage upload failed".to_string(),
        }
        .into_response();
    }

    // -- 7. upload firmado (watermark + stego)
    let signed_key = format!("{}/signed_{}", document_id, filename);

    if let Err(e) = storage::upload(
        &state.storage,
        &bucket_signatures,
        &signed_key,
        signed_bytes,
        "application/octet-stream",
    )
    .await
    {
        error!(error = %e, "upload signed file failed");
        return ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "storage upload failed".to_string(),
        }
        .into_response();
    }

    // -- 8. registra en files.objects
    let object_id = match obj_repo::register(
        &state.db,
        obj_repo::CreateObject {
            bucket_name: bucket_uploads.clone(),
            object_key: upload_key.clone(),
            filename: filename.clone(),
            content_type: "application/octet-stream".to_string(),
            size_bytes: file_size,
        },
    )
    .await
    {
        Ok(id) => id,
        Err(e) => {
            error!(error = %e, "register object failed");
            return ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "database error".to_string(),
            }
            .into_response();
        }
    };

    let _ = obj_repo::register(
        &state.db,
        obj_repo::CreateObject {
            bucket_name: bucket_signatures.clone(),
            object_key: signed_key.clone(),
            filename: format!("signed_{}", filename),
            content_type: "application/octet-stream".to_string(),
            size_bytes: signed_size,
        },
    )
    .await;

    // -- 9. registra documento con verification_code
    let doc_id = match doc_repo::create(
        &state.db,
        CreateDocument {
            filename: filename.clone(),
            hash_sha256: hash.clone(),
            signature: signature.clone(),
            author: author.clone(),
            object_id,
            verification_code: Some(verification_code.clone()),
            metadata: Some(serde_json::json!({
                "upload_key":  upload_key,
                "signed_key":  signed_key,
                "verify_url":  verify_url,
            })),
        },
    )
    .await
    {
        Ok(id) => id,
        Err(e) => {
            error!(error = %e, "db insert failed");
            return ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "database error".to_string(),
            }
            .into_response();
        }
    };

    // -- 10. audit log
    let _ = audit_repo::create(
        &state.db,
        CreateAuditLog {
            document_id: Some(doc_id),
            action: "SIGN".to_string(),
            result: DocumentStatus::Valid,
            checked_hash: Some(hash.clone()),
            details: serde_json::json!({
                "filename":          filename,
                "author":            author,
                "verification_code": verification_code,
            }),
        },
    )
    .await;

    info!(document_id = %doc_id, verification_code = %verification_code, "document signed");

    Json(ApiResponse::ok(serde_json::json!({
        "document_id":       doc_id,
        "filename":          filename,
        "hash":              hash,
        "author":            author,
        "signed_key":        signed_key,
        "verification_code": verification_code,
        "verify_url":        format!("{}/verify?code={}", state.app_base_url, verification_code),
    })))
    .into_response()
}

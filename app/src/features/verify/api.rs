use crate::shared::config::api_base_url;
use gloo_net::http::Request;
use serde::Deserialize;
use web_sys::FormData;

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyData {
    pub status: String,
    pub document_id: Option<String>,
    pub filename: Option<String>,
    pub hash: Option<String>,
    pub author: Option<String>,
    pub signed_at: Option<serde_json::Value>,
    pub hash_match: Option<bool>,
    pub signature_valid: Option<bool>,
    pub registered: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
struct VerifyResponse {
    pub success: bool,
    pub data: Option<VerifyData>,
    pub error: Option<String>,
}

pub async fn verify_document(file: web_sys::File) -> Result<VerifyData, String> {
    let form = FormData::new().map_err(|_| "failed to create form data".to_string())?;

    form.append_with_blob("file", &file)
        .map_err(|_| "failed to append file".to_string())?;

    let resp = Request::post(&format!("{}/api/v1/verify", api_base_url()))
        .body(form)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let parsed: VerifyResponse = resp.json().await.map_err(|e| e.to_string())?;

    if parsed.success {
        parsed.data.ok_or("empty response data".to_string())
    } else {
        Err(parsed.error.unwrap_or("unknown error".to_string()))
    }
}

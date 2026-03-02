use crate::shared::config::api_base_url;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SignResponse {
    pub success: bool,
    pub data: Option<SignData>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignData {
    pub document_id: String,
    pub filename: String,
    pub hash: String,
    pub author: String,
    pub signed_key: String,
}

// -- POST /api/v1/sign — sends multipart form with file + author
pub async fn sign_document(file: web_sys::File, author: String) -> Result<SignData, String> {
    use gloo_net::http::Request;
    use web_sys::FormData;

    let form = FormData::new().map_err(|_| "failed to create form data".to_string())?;

    form.append_with_blob("file", &file)
        .map_err(|_| "failed to append file".to_string())?;

    form.append_with_str("author", &author)
        .map_err(|_| "failed to append author".to_string())?;

    let resp = Request::post(&format!("{}/api/v1/sign", api_base_url()))
        .body(form)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let parsed: SignResponse = resp.json().await.map_err(|e| e.to_string())?;

    if parsed.success {
        parsed.data.ok_or("empty response data".to_string())
    } else {
        Err(parsed.error.unwrap_or("unknown error".to_string()))
    }
}

// -- builds download url for signed document
pub fn download_url(document_id: &str) -> String {
    format!(
        "{}/api/v1/documents/{}/download",
        api_base_url(),
        document_id
    )
}

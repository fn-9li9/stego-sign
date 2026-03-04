use crate::config::api_base_url;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SignedDoc {
    pub id: String,
    pub filename: String,
    pub hash_sha256: String,
    pub author: String,
    pub signed_at: String,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerificationEntry {
    pub id: String,
    pub document_id: Option<String>,
    pub filename: Option<String>,
    pub result: String,
    pub checked_hash: Option<String>,
    pub checked_at: String,
    pub upload_key: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Registry {
    pub signed: Vec<SignedDoc>,
    pub verifications: Vec<VerificationEntry>,
}

#[derive(Debug, Clone, Deserialize)]
struct RegistryData {
    pub signed: Vec<SignedDoc>,
    pub verifications: Vec<VerificationEntry>,
}

#[derive(Debug, Clone, Deserialize)]
struct RegistryResponse {
    pub data: RegistryData,
}

pub async fn fetch_registry() -> Result<Registry, String> {
    let resp = Request::get(&format!("{}/api/v1/registry", api_base_url()))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let parsed: RegistryResponse = resp.json().await.map_err(|e| e.to_string())?;

    Ok(Registry {
        signed: parsed.data.signed,
        verifications: parsed.data.verifications,
    })
}

pub fn download_url(document_id: &str) -> String {
    format!(
        "{}/api/v1/documents/{}/download",
        api_base_url(),
        document_id
    )
}

pub fn storage_download_url(bucket: &str, key: &str) -> String {
    format!("{}/api/v1/storage/{}/{}", api_base_url(), bucket, key)
}

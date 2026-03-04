use crate::config::api_base_url;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct StatsData {
    pub documents_signed: u64,
    pub verifications: u64,
    pub tampered: u64,
    pub storage_vaults: u64,
    pub objects: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct StatsResponse {
    pub data: StatsData,
}

#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub documents_signed: u64,
    pub verifications: u64,
    pub tampered: u64,
    pub objects: u64,
}

pub async fn fetch_stats() -> Result<Stats, String> {
    let resp = gloo_net::http::Request::get(&format!("{}/api/v1/stats", api_base_url()))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let parsed: StatsResponse = resp.json().await.map_err(|e| e.to_string())?;

    Ok(Stats {
        documents_signed: parsed.data.documents_signed,
        verifications: parsed.data.verifications,
        tampered: parsed.data.tampered,
        objects: parsed.data.objects,
    })
}

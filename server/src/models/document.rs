use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// -- document status matching the postgres enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DocumentStatus {
    Valid,
    Tampered,
    Unregistered,
    Invalid,
}

impl std::fmt::Display for DocumentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Valid => write!(f, "VALID"),
            Self::Tampered => write!(f, "TAMPERED"),
            Self::Unregistered => write!(f, "UNREGISTERED"),
            Self::Invalid => write!(f, "INVALID"),
        }
    }
}

// -- full document record returned from db
#[derive(Debug, Serialize)]
pub struct Document {
    pub id: Uuid,
    pub filename: String,
    pub hash_sha256: String,
    pub signature: String,
    pub author: String,
    pub object_id: Uuid,
    pub signed_at: DateTime<Utc>,
    pub status: DocumentStatus,
    pub metadata: Option<serde_json::Value>,
    pub verification_code: Option<String>,
}

// -- payload used to create a new document
#[derive(Debug)]
pub struct CreateDocument {
    pub filename: String,
    pub hash_sha256: String,
    pub signature: String,
    pub author: String,
    pub object_id: Uuid,
    pub metadata: Option<serde_json::Value>,
    pub verification_code: Option<String>,
}

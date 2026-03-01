use sea_orm::{ConnectionTrait, DatabaseConnection, QueryResult};
use tracing::info;
use uuid::Uuid;

use crate::models::audit_log::{AuditLog, CreateAuditLog};
use crate::models::document::DocumentStatus;

// -- insert a new audit log entry
pub async fn create(
    db: &DatabaseConnection,
    payload: CreateAuditLog,
) -> Result<Uuid, sea_orm::DbErr> {
    let id = Uuid::new_v4();

    db.execute(sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        INSERT INTO app.audit_log
            (id, document_id, result, checked_hash, details)
        VALUES ($1, $2, $3::app.document_status, $4, $5)
        "#,
        [
            id.into(),
            payload
                .document_id
                .map(|u| u.to_string())
                .unwrap_or_default()
                .into(),
            payload.result.to_string().into(),
            payload.checked_hash.unwrap_or_default().into(),
            payload.details.into(),
        ],
    ))
    .await?;

    info!(audit_id = %id, result = %payload.result, "audit log entry created");
    Ok(id)
}

// -- list audit entries for a specific document
pub async fn list_by_document(
    db: &DatabaseConnection,
    document_id: Uuid,
) -> Result<Vec<AuditLog>, sea_orm::DbErr> {
    let rows: Vec<QueryResult> = db
        .query_all(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
        SELECT id, document_id, checked_at, result::text, checked_hash, details
        FROM app.audit_log
        WHERE document_id = $1
        ORDER BY checked_at DESC
        "#,
            [document_id.into()],
        ))
        .await?;

    Ok(rows
        .into_iter()
        .map(|r: QueryResult| {
            let status_str: String = r.try_get("", "result").unwrap_or_default();
            AuditLog {
                id: r.try_get("", "id").unwrap(),
                document_id: r.try_get("", "document_id").ok(),
                checked_at: r.try_get("", "checked_at").unwrap(),
                checked_hash: r.try_get("", "checked_hash").ok(),
                details: r.try_get("", "details").unwrap_or(serde_json::json!({})),
                result: match status_str.as_str() {
                    "VALID" => DocumentStatus::Valid,
                    "TAMPERED" => DocumentStatus::Tampered,
                    "UNREGISTERED" => DocumentStatus::Unregistered,
                    _ => DocumentStatus::Invalid,
                },
            }
        })
        .collect())
}

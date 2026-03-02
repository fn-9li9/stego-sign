use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, QueryResult};
use tracing::info;
use uuid::Uuid;

use crate::models::audit_log::{AuditLog, CreateAuditLog};
use crate::models::document::DocumentStatus;

pub async fn create(db: &DatabaseConnection, payload: CreateAuditLog) -> Result<Uuid, DbErr> {
    let id = Uuid::new_v4();
    let action = payload.action.clone(); // -- clona antes del match

    match payload.document_id {
        Some(doc_uuid) => {
            db.execute(sea_orm::Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                r#"
                INSERT INTO app.audit_log
                    (id, document_id, action, result, checked_hash, details)
                VALUES ($1, $2, $3, $4::app.document_status, $5, $6)
                "#,
                [
                    id.into(),
                    doc_uuid.into(),
                    payload.action.into(),
                    payload.result.to_string().into(),
                    payload.checked_hash.unwrap_or_default().into(),
                    payload.details.into(),
                ],
            ))
            .await?;
        }
        None => {
            db.execute(sea_orm::Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                r#"
                INSERT INTO app.audit_log
                    (id, document_id, action, result, checked_hash, details)
                VALUES ($1, NULL, $2, $3::app.document_status, $4, $5)
                "#,
                [
                    id.into(),
                    payload.action.into(),
                    payload.result.to_string().into(),
                    payload.checked_hash.unwrap_or_default().into(),
                    payload.details.into(),
                ],
            ))
            .await?;
        }
    }

    info!(audit_id = %id, action = %action, "audit log entry created"); // -- usa el clone
    Ok(id)
}

pub async fn list_by_document(
    db: &DatabaseConnection,
    document_id: Uuid,
) -> Result<Vec<AuditLog>, DbErr> {
    let rows: Vec<QueryResult> = db
        .query_all(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT id, document_id, action, checked_at,
                   result::text, checked_hash, details
            FROM app.audit_log
            WHERE document_id = $1
            ORDER BY checked_at DESC
            "#,
            [document_id.into()],
        ))
        .await?;

    Ok(rows.into_iter().map(map_row).collect())
}

pub async fn count_by_action(db: &DatabaseConnection, action: &str) -> Result<u64, DbErr> {
    let row = db
        .query_one(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT COUNT(*)::bigint AS count FROM app.audit_log WHERE action = $1",
            [action.into()],
        ))
        .await?;

    Ok(row
        .and_then(|r| r.try_get::<i64>("", "count").ok())
        .unwrap_or(0) as u64)
}

fn map_row(r: QueryResult) -> AuditLog {
    let status_str: String = r.try_get("", "result").unwrap_or_default();
    AuditLog {
        id: r.try_get("", "id").unwrap(),
        document_id: r.try_get("", "document_id").ok(),
        action: r.try_get("", "action").unwrap_or_default(),
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
}

pub async fn count_failed_verifications(db: &DatabaseConnection) -> Result<u64, DbErr> {
    let row = db
        .query_one(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT COUNT(*)::bigint AS count
            FROM app.audit_log
            WHERE action = 'VERIFY'
              AND result != 'VALID'::app.document_status
            "#
            .to_string(),
        ))
        .await?;

    Ok(row
        .and_then(|r| r.try_get::<i64>("", "count").ok())
        .unwrap_or(0) as u64)
}

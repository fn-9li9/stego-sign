use crate::models::document::{CreateDocument, Document, DocumentStatus};
use sea_orm::DbErr;
use sea_orm::{ConnectionTrait, DatabaseConnection, QueryResult};
use tracing::info;
use uuid::Uuid;

// -- insert a new document record
pub async fn create(
    db: &DatabaseConnection,
    payload: CreateDocument,
) -> Result<Uuid, sea_orm::DbErr> {
    let id = Uuid::new_v4();

    db.execute(sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        INSERT INTO app.documents
            (id, filename, hash_sha256, signature, author, object_id, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        [
            id.into(),
            payload.filename.into(),
            payload.hash_sha256.into(),
            payload.signature.into(),
            payload.author.into(),
            payload.object_id.into(),
            payload.metadata.unwrap_or(serde_json::Value::Null).into(),
        ],
    ))
    .await?;

    info!(document_id = %id, "document record created");
    Ok(id)
}

// -- find document by its sha256 hash
pub async fn find_by_hash(
    db: &DatabaseConnection,
    hash: &str,
) -> Result<Option<Document>, sea_orm::DbErr> {
    let row: Option<QueryResult> = db
        .query_one(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
        SELECT id, filename, hash_sha256, signature, author,
               object_id, signed_at, status::text, metadata
        FROM app.documents
        WHERE hash_sha256 = $1
        LIMIT 1
        "#,
            [hash.into()],
        ))
        .await?;

    Ok(row.map(|r| map_row(r)))
}

// -- find document by id
pub async fn find_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<Option<Document>, sea_orm::DbErr> {
    let row: Option<QueryResult> = db
        .query_one(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
        SELECT id, filename, hash_sha256, signature, author,
               object_id, signed_at, status::text, metadata
        FROM app.documents
        WHERE id = $1
        "#,
            [id.into()],
        ))
        .await?;

    Ok(row.map(|r| map_row(r)))
}

// -- list all documents ordered by signed_at desc
pub async fn list(db: &DatabaseConnection, limit: u64) -> Result<Vec<Document>, sea_orm::DbErr> {
    let rows: Vec<QueryResult> = db
        .query_all(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
        SELECT id, filename, hash_sha256, signature, author,
               object_id, signed_at, status::text, metadata
        FROM app.documents
        ORDER BY signed_at DESC
        LIMIT $1
        "#,
            [limit.into()],
        ))
        .await?;

    Ok(rows.into_iter().map(map_row).collect())
}

// -- maps a raw query row to a Document model
fn map_row(r: sea_orm::QueryResult) -> Document {
    let status_str: String = r.try_get("", "status").unwrap_or_default();
    Document {
        id: r.try_get("", "id").unwrap(),
        filename: r.try_get("", "filename").unwrap(),
        hash_sha256: r.try_get("", "hash_sha256").unwrap(),
        signature: r.try_get("", "signature").unwrap(),
        author: r.try_get("", "author").unwrap(),
        object_id: r.try_get("", "object_id").unwrap(),
        signed_at: r.try_get("", "signed_at").unwrap(),
        metadata: r.try_get("", "metadata").ok(),
        status: match status_str.as_str() {
            "VALID" => DocumentStatus::Valid,
            "TAMPERED" => DocumentStatus::Tampered,
            "UNREGISTERED" => DocumentStatus::Unregistered,
            _ => DocumentStatus::Invalid,
        },
    }
}

pub async fn count_all(db: &DatabaseConnection) -> Result<u64, DbErr> {
    let row = db
        .query_one(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT COUNT(*)::bigint AS count FROM app.documents".to_string(),
        ))
        .await?;

    Ok(row
        .and_then(|r| r.try_get::<i64>("", "count").ok())
        .unwrap_or(0) as u64)
}

use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr};
use uuid::Uuid;

pub struct CreateObject {
    pub bucket_name: String,
    pub object_key: String,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
}

// -- inserta en files.objects y devuelve el id generado
pub async fn register(db: &DatabaseConnection, payload: CreateObject) -> Result<Uuid, DbErr> {
    let id = Uuid::new_v4();

    db.execute(sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        INSERT INTO files.objects
            (id, bucket_id, object_key, filename, content_type, size_bytes)
        SELECT $1, b.id, $2, $3, $4, $5
        FROM files.buckets b
        WHERE b.name = $6
        ON CONFLICT (bucket_id, object_key) DO NOTHING
        "#,
        [
            id.into(),
            payload.object_key.into(),
            payload.filename.into(),
            payload.content_type.into(),
            payload.size_bytes.into(),
            payload.bucket_name.into(),
        ],
    ))
    .await?;

    Ok(id)
}

// -- cuenta todos los objetos registrados
pub async fn count_all(db: &DatabaseConnection) -> Result<u64, DbErr> {
    let row = db
        .query_one(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT COUNT(*)::bigint AS count FROM files.objects".to_string(),
        ))
        .await?;

    Ok(row
        .and_then(|r| r.try_get::<i64>("", "count").ok())
        .unwrap_or(0) as u64)
}

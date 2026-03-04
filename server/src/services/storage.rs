use crate::config::Env;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    Client,
    config::{Credentials, Region},
};
use bytes::Bytes;
use tracing::info;

// -- bucket name helpers usando prefijo
pub fn bucket_uploads(prefix: &str) -> String {
    format!("{}-uploads", prefix)
}
pub fn bucket_signatures(prefix: &str) -> String {
    format!("{}-signatures", prefix)
}
pub fn bucket_corrupted(prefix: &str) -> String {
    format!("{}-corrupted", prefix)
}

pub async fn build_client(env: &Env) -> Client {
    let creds = Credentials::new(
        &env.storage_access_key,
        &env.storage_secret_key,
        None,
        None,
        "provider",
    );
    match env.storage_provider.as_str() {
        "aws" => {
            let config = aws_config::defaults(BehaviorVersion::latest())
                .region(Region::new("us-east-1"))
                .credentials_provider(creds)
                .load()
                .await;
            Client::new(&config)
        }
        _ => {
            let config = aws_config::defaults(BehaviorVersion::latest())
                .region(Region::new("us-east-1"))
                .endpoint_url(&env.storage_endpoint)
                .credentials_provider(creds)
                .load()
                .await;
            let s3_config = aws_sdk_s3::config::Builder::from(&config)
                .force_path_style(true)
                .build();
            Client::from_conf(s3_config)
        }
    }
}

pub async fn ensure_buckets(
    client: &Client,
    db: &sea_orm::DatabaseConnection,
    prefix: &str,
) -> anyhow::Result<()> {
    use sea_orm::ConnectionTrait;
    for bucket in [
        bucket_uploads(prefix),
        bucket_signatures(prefix),
        bucket_corrupted(prefix),
    ] {
        match client.head_bucket().bucket(&bucket).send().await {
            Ok(_) => info!(bucket = %bucket, "bucket already exists"),
            Err(_) => {
                client.create_bucket().bucket(&bucket).send().await?;
                info!(bucket = %bucket, "bucket created successfully");
            }
        }
        db.execute(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            INSERT INTO files.buckets (name, region)
            VALUES ($1, 'us-east-1')
            ON CONFLICT (name) DO NOTHING
            "#,
            [bucket.as_str().into()],
        ))
        .await?;
        info!(bucket = %bucket, "bucket registered in database");
    }
    info!("all buckets verified and ready");
    Ok(())
}

pub async fn upload(
    client: &Client,
    bucket: &str,
    key: &str,
    data: Bytes,
    content_type: &str,
) -> anyhow::Result<String> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(data.into())
        .content_type(content_type)
        .send()
        .await?;
    info!(bucket = %bucket, key = %key, "object uploaded");
    Ok(key.to_string())
}

pub async fn download(client: &Client, bucket: &str, key: &str) -> anyhow::Result<Bytes> {
    let resp = client.get_object().bucket(bucket).key(key).send().await?;
    let data = resp.body.collect().await?.into_bytes();
    Ok(data)
}

pub async fn download_storage(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
) -> Result<(bytes::Bytes, String), String> {
    let resp = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let content_type = resp
        .content_type()
        .unwrap_or("application/octet-stream")
        .to_string();
    let data = resp
        .body
        .collect()
        .await
        .map_err(|e| e.to_string())?
        .into_bytes();
    Ok((data, content_type))
}

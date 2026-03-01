use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    Client,
    config::{Credentials, Region},
};
use bytes::Bytes;
use tracing::{info, warn};

use crate::config::Env;

// -- bucket names
pub const BUCKET_UPLOADS: &str = "uploads";
pub const BUCKET_SIGNATURES: &str = "signatures";
pub const BUCKET_CORRUPTED: &str = "corrupted";

// -- builds and returns a configured s3 client pointing to minio
pub async fn build_client(env: &Env) -> Client {
    let creds = Credentials::new(
        &env.minio_root_user,
        &env.minio_root_password,
        None,
        None,
        "minio",
    );

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new("us-east-1"))
        .endpoint_url(&env.minio_endpoint)
        .credentials_provider(creds)
        .load()
        .await;

    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .force_path_style(true) // -- required for minio
        .build();

    Client::from_conf(s3_config)
}

// -- ensures all required buckets exist, creates them if missing
// -- self-healing: runs on every server startup
pub async fn ensure_buckets(client: &Client) -> anyhow::Result<()> {
    for bucket in [BUCKET_UPLOADS, BUCKET_SIGNATURES, BUCKET_CORRUPTED] {
        match client.head_bucket().bucket(bucket).send().await {
            Ok(_) => {
                info!(bucket = %bucket, "bucket already exists");
            }
            Err(_) => {
                // -- bucket does not exist, create it
                match client.create_bucket().bucket(bucket).send().await {
                    Ok(_) => info!(bucket = %bucket, "bucket created successfully"),
                    Err(e) => {
                        warn!(bucket = %bucket, error = %e, "failed to create bucket");
                        return Err(anyhow::anyhow!("failed to create bucket {}: {}", bucket, e));
                    }
                }
            }
        }
    }

    info!("all buckets verified and ready");
    Ok(())
}

// -- uploads bytes to a specific bucket and returns the object key
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

// -- downloads an object and returns its bytes
pub async fn download(client: &Client, bucket: &str, key: &str) -> anyhow::Result<Bytes> {
    let resp = client.get_object().bucket(bucket).key(key).send().await?;

    let data = resp.body.collect().await?.into_bytes();
    Ok(data)
}

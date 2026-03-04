use std::env;

#[derive(Debug, Clone)]
pub struct Env {
    // -- database
    pub database_url: String,
    // -- storage
    pub storage_provider: String, // "aistor" | "aws"
    pub storage_endpoint: String,
    pub storage_access_key: String,
    pub storage_secret_key: String,
    // -- server
    pub server_host: String,
    pub server_port: u16,
    // -- signing keys
    pub signing_key: String,
    pub verify_key: String,
    // -- app
    pub app_base_url: String,
    // -- storage-bucket-prefix
    pub storage_bucket_prefix: String,
}

impl Env {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        // -- database_url directo o construido desde partes
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            let host = var("POSTGRES_HOST", "localhost");
            let port = var("POSTGRES_PORT", "5432");
            let db = var("POSTGRES_DB", "stegosign");
            let user = var("POSTGRES_USER", "sre");
            let pass = var("POSTGRES_PASSWORD", "sre");
            format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db)
        });

        let signing_key = var("SIGNING_KEY", "");
        let verify_key = var("VERIFY_KEY", "");

        if signing_key.is_empty() || verify_key.is_empty() {
            tracing::warn!(
                "SIGNING_KEY or VERIFY_KEY is empty — \
                call GET /api/v1/admin/keygen to generate them, \
                then add to .env and restart"
            );
        }

        Self {
            database_url,
            storage_provider: var("STORAGE_PROVIDER", "aistor"),
            storage_endpoint: var("STORAGE_ENDPOINT", "http://localhost:9000"),
            storage_access_key: var("STORAGE_ACCESS_KEY", "admin"),
            storage_secret_key: var("STORAGE_SECRET_KEY", "fn-stella-sre"),
            server_host: var("SERVER_HOST", "0.0.0.0"),
            server_port: var("SERVER_PORT", "4000")
                .parse::<u16>()
                .expect("SERVER_PORT must be a valid port number"),
            signing_key,
            verify_key,
            app_base_url: var("APP_BASE_URL", "http://localhost:3001"),
            storage_bucket_prefix: var("STORAGE_BUCKET_PREFIX", "stego"),
        }
    }
}

fn var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

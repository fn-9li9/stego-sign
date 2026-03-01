use std::env;

// -- holds all environment variables for the application
#[derive(Debug, Clone)]
pub struct Env {
    // -- database
    pub database_url: String,
    pub postgres_host: String,
    pub postgres_port: u16,
    pub postgres_db: String,
    pub postgres_user: String,
    pub postgres_password: String,

    // -- server
    pub server_host: String,
    pub server_port: u16,

    // -- minio
    pub minio_endpoint: String,
    pub minio_root_user: String,
    pub minio_root_password: String,

    // -- en el struct Env
    pub signing_key: String, // -- ed25519 private key pkcs8 base64
    pub verify_key: String,  // -- ed25519 public key raw base64
}

impl Env {
    // -- loads and validates all env vars, panics early if any required var is missing
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        let postgres_host = var("POSTGRES_HOST", "localhost");
        let postgres_port = var("POSTGRES_PORT", "55432")
            .parse::<u16>()
            .expect("POSTGRES_PORT must be a valid port number");
        let postgres_db = var("POSTGRES_DB", "stegosign");
        let postgres_user = var("POSTGRES_USER", "sre");
        let postgres_pass = var("POSTGRES_PASSWORD", "sre");

        // -- build database_url from parts if DATABASE_URL is not set
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                postgres_user, postgres_pass, postgres_host, postgres_port, postgres_db
            )
        });

        Self {
            database_url,
            postgres_host,
            postgres_port,
            postgres_db,
            postgres_user,
            postgres_password: postgres_pass,

            server_host: var("SERVER_HOST", "0.0.0.0"),
            server_port: var("SERVER_PORT", "4000")
                .parse::<u16>()
                .expect("SERVER_PORT must be a valid port number"),

            minio_endpoint: var("MINIO_ENDPOINT", "http://localhost:9002"),
            minio_root_user: var("MINIO_ROOT_USER", "admin"),
            minio_root_password: var("MINIO_ROOT_PASSWORD", "fn-stella-sre"),

            signing_key: var("SIGNING_KEY", ""),
            verify_key: var("VERIFY_KEY", ""),
        }
    }
}

// -- helper: reads env var or returns a default value
fn var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

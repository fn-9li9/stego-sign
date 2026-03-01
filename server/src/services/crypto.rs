use base64::{Engine, engine::general_purpose::STANDARD as B64};
use ring::{
    digest::{self, SHA256},
    rand::SystemRandom,
    signature::{ED25519, Ed25519KeyPair, KeyPair, UnparsedPublicKey},
};
use tracing::{error, info};

// -- computes sha256 of bytes and returns hex string
pub fn sha256(data: &[u8]) -> String {
    let digest = digest::digest(&SHA256, data);
    hex::encode(digest.as_ref())
}

// -- generates a new ed25519 keypair and returns (private_pkcs8, public_raw) as base64
pub fn generate_keypair() -> anyhow::Result<(String, String)> {
    let rng = SystemRandom::new();
    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(|e| anyhow::anyhow!("keypair generation failed: {:?}", e))?;

    let pair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref())
        .map_err(|e| anyhow::anyhow!("keypair parse failed: {:?}", e))?;

    let private_b64 = B64.encode(pkcs8.as_ref());
    let public_b64 = B64.encode(pair.public_key().as_ref());

    info!("ed25519 keypair generated");
    Ok((private_b64, public_b64))
}

// -- signs a message with the ed25519 private key (pkcs8 base64 encoded)
pub fn sign(message: &str, private_key_b64: &str) -> anyhow::Result<String> {
    let pkcs8 = B64
        .decode(private_key_b64)
        .map_err(|e| anyhow::anyhow!("private key decode failed: {}", e))?;

    let pair = Ed25519KeyPair::from_pkcs8(&pkcs8)
        .map_err(|e| anyhow::anyhow!("keypair load failed: {:?}", e))?;

    let sig = pair.sign(message.as_bytes());
    Ok(B64.encode(sig.as_ref()))
}

// -- verifies an ed25519 signature
// -- returns true if valid, false otherwise (never panics)
pub fn verify(message: &str, signature_b64: &str, public_key_b64: &str) -> bool {
    let Ok(sig_bytes) = B64.decode(signature_b64) else {
        return false;
    };
    let Ok(pub_bytes) = B64.decode(public_key_b64) else {
        return false;
    };

    let public_key = UnparsedPublicKey::new(&ED25519, pub_bytes);
    match public_key.verify(message.as_bytes(), &sig_bytes) {
        Ok(_) => true,
        Err(e) => {
            error!(error = ?e, "signature verification failed");
            false
        }
    }
}

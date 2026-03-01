use bytes::Bytes;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use uuid::Uuid;

// -- magic marker to locate the stego payload block
const MAGIC: &[u8] = b">>STEGO::PAYLOAD<<";
const MAGIC_END: &[u8] = b">>STEGO::END<<";

// -- payload embedded in every signed file
#[derive(Debug, Serialize, Deserialize)]
pub struct StegoPayload {
    pub document_id: String,
    pub original_hash: String,
    pub signature: String,
    pub author: String,
    pub version: u8,
}

// -- embeds the stego payload into the file bytes
// -- strategy: append magic block at end of file (works for png, jpg, pdf, any binary)
pub fn embed(
    data: &Bytes,
    _filename: &str,
    document_id: Uuid,
    hash: &str,
    signature: &str,
    author: &str,
) -> anyhow::Result<Bytes> {
    let payload = StegoPayload {
        document_id: document_id.to_string(),
        original_hash: hash.to_string(),
        signature: signature.to_string(),
        author: author.to_string(),
        version: 1,
    };

    let payload_json = serde_json::to_vec(&payload)?;

    // -- build: original bytes + MAGIC + payload json + MAGIC_END
    let mut output =
        Vec::with_capacity(data.len() + MAGIC.len() + payload_json.len() + MAGIC_END.len());
    output.extend_from_slice(data);
    output.extend_from_slice(MAGIC);
    output.extend_from_slice(&payload_json);
    output.extend_from_slice(MAGIC_END);

    info!(
        document_id = %document_id,
        payload_size = payload_json.len(),
        "stego payload embedded"
    );

    Ok(Bytes::from(output))
}

// -- extracts the stego payload from file bytes
// -- returns error if no valid payload found
pub fn extract(_filename: &str, data: &Bytes) -> anyhow::Result<StegoPayload> {
    // -- locate MAGIC marker
    let magic_pos = find_subsequence(data, MAGIC)
        .ok_or_else(|| anyhow::anyhow!("stego magic marker not found"))?;

    let payload_start = magic_pos + MAGIC.len();

    // -- locate MAGIC_END marker
    let end_pos = find_subsequence(&data[payload_start..], MAGIC_END)
        .ok_or_else(|| anyhow::anyhow!("stego end marker not found"))?;

    let payload_bytes = &data[payload_start..payload_start + end_pos];

    let payload: StegoPayload = serde_json::from_slice(payload_bytes)
        .map_err(|e| anyhow::anyhow!("payload deserialization failed: {}", e))?;

    info!(
        document_id = %payload.document_id,
        version     = %payload.version,
        "stego payload extracted"
    );

    Ok(payload)
}

// -- returns the original file bytes without the stego payload block
// -- useful when you need to recompute the hash of the original content
pub fn strip(data: &Bytes) -> Bytes {
    if let Some(pos) = find_subsequence(data, MAGIC) {
        Bytes::copy_from_slice(&data[..pos])
    } else {
        warn!("strip called on file with no stego payload");
        data.clone()
    }
}

// -- finds the start index of a subsequence within a slice
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

use image::{GrayImage, Luma};
use qrcode::{EcLevel, QrCode};
use rand::RngExt;

/// Genera un QR code como PNG en memoria
pub fn generate_qr_png(content: &str, pixel_size: u32) -> Result<Vec<u8>, String> {
    let code = QrCode::with_error_correction_level(content, EcLevel::M)
        .map_err(|e| format!("qr encode: {}", e))?;

    let image: GrayImage = code
        .render::<Luma<u8>>()
        .min_dimensions(pixel_size, pixel_size)
        .quiet_zone(true)
        .build();

    let mut buf = Vec::new();
    image
        .write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
        .map_err(|e| format!("png encode: {}", e))?;

    Ok(buf)
}

/// Genera un código de verificación en formato XDS-6H2
pub fn generate_verification_code() -> String {
    const CHARS: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ23456789";
    let mut rng = rand::rng();

    let mut code = String::with_capacity(7);

    for i in 0..6 {
        if i == 3 {
            code.push('-');
        }
        let idx = rng.random_range(0..CHARS.len());
        code.push(CHARS[idx] as char);
    }

    code
}

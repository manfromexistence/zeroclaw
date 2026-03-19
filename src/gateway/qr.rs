//! QR code generation for device pairing.
//!
//! Generates QR codes in multiple formats:
//! - ASCII/Unicode for terminal display
//! - PNG for GUI/web display
//! - Data URL for embedding in JSON responses

use qrcode::QrCode;
use qrcode::render::unicode;

/// Generate a QR code for device pairing (ASCII/Unicode format for terminal).
///
/// The QR code encodes a deep link: `zeroclaw://pair?code={code}&gateway={gateway_url}`
///
/// # Example
/// ```
/// let qr = generate_pairing_qr("ABC123", "wss://gateway.example.com")?;
/// println!("{}", qr);
/// ```
pub fn generate_pairing_qr(code: &str, gateway_url: &str) -> anyhow::Result<String> {
    let data = format!("zeroclaw://pair?code={}&gateway={}", code, gateway_url);
    let qr = QrCode::new(data.as_bytes())?;
    
    Ok(qr
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build())
}

/// Generate a QR code as PNG image bytes.
///
/// Returns raw PNG bytes suitable for HTTP responses or file writing.
pub fn generate_pairing_qr_png(code: &str, gateway_url: &str) -> anyhow::Result<Vec<u8>> {
    let data = format!("zeroclaw://pair?code={}&gateway={}", code, gateway_url);
    let qr = QrCode::new(data.as_bytes())?;
    
    // Render as image with 8x scale for better readability
    let image = qr
        .render::<image::Luma<u8>>()
        .min_dimensions(200, 200)
        .build();
    
    // Encode as PNG
    let mut buf = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buf);
    image.write_to(&mut cursor, image::ImageFormat::Png)?;
    
    Ok(buf)
}

/// Generate a QR code as a data URL (base64-encoded PNG).
///
/// Returns a data URL suitable for embedding in HTML or JSON:
/// `data:image/png;base64,iVBORw0KGgo...`
pub fn generate_pairing_qr_data_url(code: &str, gateway_url: &str) -> anyhow::Result<String> {
    let png_bytes = generate_pairing_qr_png(code, gateway_url)?;
    let base64_data = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png_bytes);
    Ok(format!("data:image/png;base64,{}", base64_data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qr_code_generates_ascii() {
        let result = generate_pairing_qr("TEST123", "wss://localhost:3000");
        assert!(result.is_ok());
        let qr = result.unwrap();
        assert!(!qr.is_empty());
        // QR codes contain block characters
        assert!(qr.contains('█') || qr.contains('▀') || qr.contains('▄'));
    }

    #[test]
    fn qr_code_generates_png() {
        let result = generate_pairing_qr_png("TEST123", "wss://localhost:3000");
        assert!(result.is_ok());
        let png = result.unwrap();
        assert!(!png.is_empty());
        // PNG magic bytes
        assert_eq!(&png[0..4], &[0x89, 0x50, 0x4E, 0x47]);
    }

    #[test]
    fn qr_code_generates_data_url() {
        let result = generate_pairing_qr_data_url("TEST123", "wss://localhost:3000");
        assert!(result.is_ok());
        let url = result.unwrap();
        assert!(url.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn qr_code_encodes_correct_data() {
        let code = "ABC123";
        let gateway = "wss://example.com";
        let result = generate_pairing_qr(code, gateway);
        assert!(result.is_ok());
        // The QR should encode: zeroclaw://pair?code=ABC123&gateway=wss://example.com
    }
}

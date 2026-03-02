use image::GenericImageView;
use lopdf::{Dictionary, Document, Object, ObjectId, Stream};

#[derive(Debug, Clone, Copy)]
pub enum WatermarkPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl WatermarkPosition {
    pub fn from_str(s: &str) -> Self {
        match s {
            "top-left" => Self::TopLeft,
            "top-center" => Self::TopCenter,
            "top-right" => Self::TopRight,
            "bottom-left" => Self::BottomLeft,
            "bottom-center" => Self::BottomCenter,
            _ => Self::BottomRight,
        }
    }
}

/// Inserta un QR PNG en todas las páginas de un PDF existente
pub fn insert_qr_into_pdf(
    pdf_bytes: &[u8],
    qr_png: &[u8],
    position: WatermarkPosition,
    size_pts: f64,
) -> Result<Vec<u8>, String> {
    let mut doc = Document::load_mem(pdf_bytes).map_err(|e| format!("load pdf: {}", e))?;

    // -- decodifica QR a RGB raw
    let img = image::load_from_memory(qr_png).map_err(|e| format!("load qr image: {}", e))?;
    let (img_w, img_h) = img.dimensions();
    let rgb_data = img.to_rgb8().into_raw();

    // -- crea Image XObject
    let mut img_dict = Dictionary::new();
    img_dict.set("Type", Object::Name(b"XObject".to_vec()));
    img_dict.set("Subtype", Object::Name(b"Image".to_vec()));
    img_dict.set("Width", Object::Integer(img_w as i64));
    img_dict.set("Height", Object::Integer(img_h as i64));
    img_dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
    img_dict.set("BitsPerComponent", Object::Integer(8));

    let img_obj_id = doc.add_object(Stream::new(img_dict, rgb_data));

    // -- recorre todas las páginas
    let page_ids: Vec<ObjectId> = doc.get_pages().values().cloned().collect();

    for page_id in page_ids {
        // -- lee MediaBox
        let (page_w, page_h) = get_page_dimensions(&doc, page_id);
        let (x, y) = compute_xy(position, page_w, page_h, size_pts, 20.0);

        // -- agrega XObject a Resources de la página
        add_xobject_to_page(&mut doc, page_id, img_obj_id)
            .map_err(|e| format!("add xobject: {}", e))?;

        // -- crea stream de dibujo
        let draw = format!(
            "q\n{sz} 0 0 {sz} {x} {y} cm\n/QRWatermark Do\nQ\n",
            sz = size_pts,
            x = x,
            y = y,
        );
        let draw_id = doc.add_object(Stream::new(Dictionary::new(), draw.into_bytes()));

        // -- añade el stream a Contents
        append_content(&mut doc, page_id, draw_id).map_err(|e| format!("append content: {}", e))?;
    }

    let mut out = Vec::new();
    doc.save_to(&mut out)
        .map_err(|e| format!("save pdf: {}", e))?;

    Ok(out)
}

// -- helpers

fn get_page_dimensions(doc: &Document, page_id: ObjectId) -> (f64, f64) {
    doc.get_object(page_id)
        .ok()
        .and_then(|o| o.as_dict().ok())
        .and_then(|d| d.get(b"MediaBox").ok())
        .and_then(|o| o.as_array().ok())
        .map(|arr| {
            let w = arr.get(2).and_then(|v| v.as_float().ok()).unwrap_or(612.0);
            let h = arr.get(3).and_then(|v| v.as_float().ok()).unwrap_or(792.0);
            (w as f64, h as f64)
        })
        .unwrap_or((612.0, 792.0))
}

fn compute_xy(pos: WatermarkPosition, pw: f64, ph: f64, sz: f64, margin: f64) -> (f64, f64) {
    match pos {
        WatermarkPosition::TopLeft => (margin, ph - sz - margin),
        WatermarkPosition::TopCenter => ((pw - sz) / 2.0, ph - sz - margin),
        WatermarkPosition::TopRight => (pw - sz - margin, ph - sz - margin),
        WatermarkPosition::BottomLeft => (margin, margin),
        WatermarkPosition::BottomCenter => ((pw - sz) / 2.0, margin),
        WatermarkPosition::BottomRight => (pw - sz - margin, margin),
    }
}

fn add_xobject_to_page(
    doc: &mut Document,
    page_id: ObjectId,
    img_id: ObjectId,
) -> Result<(), lopdf::Error> {
    let page = doc.get_object_mut(page_id)?.as_dict_mut()?;

    // -- obtiene o crea Resources
    if !page.has(b"Resources") {
        page.set("Resources", Object::Dictionary(Dictionary::new()));
    }

    let resources = page.get_mut(b"Resources")?.as_dict_mut()?;

    // -- obtiene o crea XObject dentro de Resources
    if !resources.has(b"XObject") {
        resources.set("XObject", Object::Dictionary(Dictionary::new()));
    }

    let xobjects = resources.get_mut(b"XObject")?.as_dict_mut()?;
    xobjects.set("QRWatermark", Object::Reference(img_id));

    Ok(())
}

fn append_content(
    doc: &mut Document,
    page_id: ObjectId,
    draw_id: ObjectId,
) -> Result<(), lopdf::Error> {
    let page = doc.get_object_mut(page_id)?.as_dict_mut()?;

    match page.get_mut(b"Contents") {
        Ok(contents) => match contents {
            Object::Reference(r) => {
                let old = Object::Reference(*r);
                *contents = Object::Array(vec![old, Object::Reference(draw_id)]);
            }
            Object::Array(arr) => {
                arr.push(Object::Reference(draw_id));
            }
            _ => {
                *contents = Object::Array(vec![Object::Reference(draw_id)]);
            }
        },
        Err(_) => {
            page.set("Contents", Object::Reference(draw_id));
        }
    }

    Ok(())
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::assets::{ICON_SVG, OGP_SVG};
use axum::body::{Body, Bytes};
use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE, ETAG, IF_NONE_MATCH};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::Response;
use image::codecs::avif::AvifEncoder;
use image::codecs::ico::IcoEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::imageops::FilterType;
use image::{ColorType, DynamicImage, ImageEncoder, Rgba, RgbaImage};
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::Options;

#[derive(Clone)]
pub struct EncodedImage {
    pub bytes: Bytes,
    pub content_type: &'static str,
    pub etag: String,
}

#[derive(Clone)]
pub struct CachedImage {
    image: RgbaImage,
    png: Option<EncodedImage>,
    jpg: Option<EncodedImage>,
    avif: Option<EncodedImage>,
    ico: Option<EncodedImage>,
}

impl CachedImage {
    pub fn new(image: RgbaImage) -> Self {
        Self {
            image,
            png: None,
            jpg: None,
            avif: None,
            ico: None,
        }
    }

    pub fn encoded(&mut self, kind: ImageKind) -> Option<&EncodedImage> {
        match kind {
            ImageKind::Png => get_or_encode(&mut self.png, || encode_png(&self.image)),
            ImageKind::Jpeg => get_or_encode(&mut self.jpg, || encode_jpeg(&self.image)),
            ImageKind::Avif => get_or_encode(&mut self.avif, || encode_avif(&self.image)),
            ImageKind::Ico => get_or_encode(&mut self.ico, || encode_ico(&self.image)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ImageKind {
    Png,
    Jpeg,
    Avif,
    Ico,
}

pub fn ext_to_kind(ext: &str) -> Option<ImageKind> {
    match ext.to_ascii_lowercase().as_str() {
        "png" => Some(ImageKind::Png),
        "jpg" | "jpeg" => Some(ImageKind::Jpeg),
        "avif" => Some(ImageKind::Avif),
        "ico" => Some(ImageKind::Ico),
        _ => None,
    }
}

pub fn respond_image_with_cache(
    headers: &HeaderMap,
    image: &EncodedImage,
    cache_control: &str,
) -> Response {
    let not_modified = headers
        .get(IF_NONE_MATCH)
        .and_then(|value| value.to_str().ok())
        == Some(image.etag.as_str());
    let mut res = if not_modified {
        let mut res = Response::new(Body::empty());
        *res.status_mut() = StatusCode::NOT_MODIFIED;
        res
    } else {
        Response::new(Body::from(image.bytes.clone()))
    };
    let headers_mut = res.headers_mut();
    if let Ok(value) = HeaderValue::from_str(cache_control) {
        headers_mut.insert(CACHE_CONTROL, value);
    }
    if let Ok(value) = HeaderValue::from_str(&image.etag) {
        headers_mut.insert(ETAG, value);
    }
    if let Ok(value) = HeaderValue::from_str(image.content_type) {
        headers_mut.insert(CONTENT_TYPE, value);
    }
    res
}

pub fn resize_square(image: RgbaImage, size: u32) -> RgbaImage {
    if image.width() == size && image.height() == size {
        return image;
    }
    image::imageops::resize(&image, size, size, FilterType::Triangle)
}

pub fn render_icon_image(size: u32) -> RgbaImage {
    render_svg(ICON_SVG, size, size).unwrap()
}

pub fn render_ogp_image(width: u32, height: u32) -> RgbaImage {
    render_svg(OGP_SVG, width, height).unwrap()
}

fn render_svg(svg: &str, width: u32, height: u32) -> Option<RgbaImage> {
    if width == 0 || height == 0 {
        return None;
    }
    let mut opt = Options::default();
    opt.fontdb_mut().load_system_fonts();
    let svg_bytes = svg.as_bytes();
    let rtree = resvg::usvg::Tree::from_data(svg_bytes, &opt).ok()?;
    let target_size = rtree.size();
    let mut pixmap = Pixmap::new(width, height)?;
    let sx = (width as f32) / target_size.width();
    let sy = (height as f32) / target_size.height();
    let transform = Transform::from_row(sx, 0.0, 0.0, sy, 0.0, 0.0);
    resvg::render(&rtree, transform, &mut pixmap.as_mut());
    let data = pixmap.data();
    let mut out = RgbaImage::new(width, height);
    for (i, chunk) in data.chunks_exact(4).enumerate() {
        let x = i as u32 % width;
        let y = i as u32 / width;
        let r_p = chunk[0];
        let g_p = chunk[1];
        let b_p = chunk[2];
        let a = chunk[3];
        let (r, g, b) = if a == 0 {
            (0u8, 0u8, 0u8)
        } else {
            let ap = a as u32;
            let r = ((r_p as u32) * 255 + ap / 2) / ap;
            let g = ((g_p as u32) * 255 + ap / 2) / ap;
            let b = ((b_p as u32) * 255 + ap / 2) / ap;
            (r.min(255) as u8, g.min(255) as u8, b.min(255) as u8)
        };
        out.put_pixel(x, y, Rgba([r, g, b, a]));
    }
    Some(out)
}

pub fn to_rgba(image: DynamicImage) -> RgbaImage {
    image.to_rgba8()
}

fn compute_etag(bytes: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    format!("W/\"{:x}\"", hasher.finish())
}

fn get_or_encode(
    slot: &mut Option<EncodedImage>,
    encode: impl FnOnce() -> Option<EncodedImage>,
) -> Option<&EncodedImage> {
    if slot.is_none() {
        *slot = encode();
    }
    slot.as_ref()
}

fn encoded(out: Vec<u8>, content_type: &'static str) -> Option<EncodedImage> {
    let etag = compute_etag(&out);
    Some(EncodedImage {
        bytes: Bytes::from(out),
        content_type,
        etag,
    })
}

fn encode_png(image: &RgbaImage) -> Option<EncodedImage> {
    let mut out = Vec::new();
    let encoder = PngEncoder::new(&mut out);
    if encoder
        .write_image(
            image,
            image.width(),
            image.height(),
            ColorType::Rgba8.into(),
        )
        .is_err()
    {
        return None;
    }
    encoded(out, "image/png")
}

fn encode_jpeg(image: &RgbaImage) -> Option<EncodedImage> {
    let mut out = Vec::new();
    let rgb = DynamicImage::ImageRgba8(image.clone()).to_rgb8();
    let mut encoder = JpegEncoder::new_with_quality(&mut out, 85);
    if encoder
        .encode(&rgb, rgb.width(), rgb.height(), ColorType::Rgb8.into())
        .is_err()
    {
        return None;
    }
    encoded(out, "image/jpeg")
}

fn encode_avif(image: &RgbaImage) -> Option<EncodedImage> {
    let mut out = Vec::new();
    let encoder = AvifEncoder::new_with_speed_quality(&mut out, 8, 60);
    if encoder
        .write_image(
            image,
            image.width(),
            image.height(),
            ColorType::Rgba8.into(),
        )
        .is_err()
    {
        return None;
    }
    encoded(out, "image/avif")
}

fn encode_ico(image: &RgbaImage) -> Option<EncodedImage> {
    let mut out = Vec::new();
    let encoder = IcoEncoder::new(&mut out);
    if encoder
        .write_image(
            image,
            image.width(),
            image.height(),
            ColorType::Rgba8.into(),
        )
        .is_err()
    {
        return None;
    }
    encoded(out, "image/x-icon")
}

#[cfg(test)]
mod tests {
    use super::{EncodedImage, respond_image_with_cache};
    use axum::body::{Bytes, to_bytes};
    use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE, ETAG, IF_NONE_MATCH};
    use axum::http::{HeaderMap, HeaderValue, StatusCode};

    #[tokio::test]
    async fn returns_not_modified_with_cache_headers() {
        let image = EncodedImage {
            bytes: Bytes::from_static(b"png"),
            content_type: "image/png",
            etag: "W/\"abc\"".to_string(),
        };
        let mut req_headers = HeaderMap::new();
        req_headers.insert(IF_NONE_MATCH, HeaderValue::from_static("W/\"abc\""));

        let response = respond_image_with_cache(&req_headers, &image, "public, max-age=10");

        assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
        assert_eq!(
            response.headers().get(CACHE_CONTROL).unwrap(),
            "public, max-age=10"
        );
        assert_eq!(response.headers().get(ETAG).unwrap(), "W/\"abc\"");
        assert_eq!(response.headers().get(CONTENT_TYPE).unwrap(), "image/png");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert!(body.is_empty());
    }
}

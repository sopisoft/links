use crate::assets::{ICON_SIZE, ICON_SVG, OGP_HEIGHT, OGP_SVG, OGP_WIDTH};
use crate::icons::fetch_best_icon;
use crate::images::{
    CachedImage, ImageKind, ext_to_kind, render_icon_image, render_ogp_image, resize_square,
    respond_image_with_cache,
};
use crate::links::LinkId;
use crate::page::{PageData, styles_css};
use crate::state::AppState;
use axum::Extension;
use axum::body::Body;
use axum::extract::Path;
use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE, HeaderName, VARY};
use axum::http::{HeaderMap, HeaderValue, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Redirect, Response};
use std::sync::Arc;
use std::time::{Duration, Instant};
const FAVICON_FETCH_TIMEOUT: Duration = Duration::from_secs(6);
const ICON_CACHE_CONTROL: &str = "public, max-age=31536000, immutable";
const DEFAULT_CACHE_CONTROL: &str = "public, max-age=86400";

fn cache_headers(content_type: &'static str, cache_control: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static(content_type));
    headers.insert(CACHE_CONTROL, HeaderValue::from_static(cache_control));
    headers.insert(VARY, HeaderValue::from_static("Accept-Encoding"));
    headers
}

pub async fn security_headers(request: Request<Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    insert_if_missing(
        headers,
        "content-security-policy",
        "default-src 'none'; img-src 'self'; style-src 'self';",
    );
    insert_if_missing(
        headers,
        "referrer-policy",
        "strict-origin-when-cross-origin",
    );
    insert_if_missing(headers, "x-content-type-options", "nosniff");
    insert_if_missing(headers, "x-frame-options", "DENY");
    insert_if_missing(
        headers,
        "permissions-policy",
        "camera=(), microphone=(), geolocation=(), interest-cohort=()",
    );
    insert_if_missing(headers, "cross-origin-opener-policy", "same-origin");
    insert_if_missing(headers, "cross-origin-resource-policy", "same-origin");
    response
}

pub async fn index(headers: HeaderMap) -> Response {
    let data = PageData::from_headers(&headers);
    let html = crate::page::render_page(data);
    let mut response = Html(html).into_response();
    let headers_mut = response.headers_mut();
    headers_mut.insert(
        CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=0, s-maxage=60, stale-while-revalidate=30"),
    );
    headers_mut.insert(
        VARY,
        HeaderValue::from_static("Accept-Language, Accept-Encoding"),
    );
    response
}

pub async fn redirect() -> impl IntoResponse {
    Redirect::permanent("/").into_response()
}

pub async fn styles() -> impl IntoResponse {
    (
        cache_headers("text/css; charset=utf-8", DEFAULT_CACHE_CONTROL),
        styles_css(),
    )
}

pub async fn robots() -> impl IntoResponse {
    (cache_headers("text/plain", DEFAULT_CACHE_CONTROL), "")
}

pub async fn icon_svg() -> impl IntoResponse {
    (cache_headers("image/svg+xml", ICON_CACHE_CONTROL), ICON_SVG)
}

pub async fn ogp_svg() -> impl IntoResponse {
    (
        cache_headers("image/svg+xml", DEFAULT_CACHE_CONTROL),
        OGP_SVG,
    )
}

pub async fn icon_raster(
    Extension(state): Extension<Arc<AppState>>,
    Path(ext): Path<String>,
    headers: HeaderMap,
) -> Response {
    let Some(kind) = ext_to_kind(&ext) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let mut cache = state
        .svgs
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let encoded = cache
        .icon
        .get_or_insert_with(|| CachedImage::new(render_icon_image(ICON_SIZE)))
        .encoded(kind)
        .unwrap_or_else(|| unreachable!("icon encoding should always succeed"));
    respond_image_with_cache(&headers, encoded, ICON_CACHE_CONTROL)
}

pub async fn ogp_raster(
    Extension(state): Extension<Arc<AppState>>,
    Path(ext): Path<String>,
    headers: HeaderMap,
) -> Response {
    let Some(kind) = ext_to_kind(&ext) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let mut cache = state
        .svgs
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let encoded = cache
        .ogp
        .get_or_insert_with(|| CachedImage::new(render_ogp_image(OGP_WIDTH, OGP_HEIGHT)))
        .encoded(kind)
        .unwrap_or_else(|| unreachable!("ogp encoding should always succeed"));
    respond_image_with_cache(&headers, encoded, DEFAULT_CACHE_CONTROL)
}

pub async fn favicon_ico(
    Extension(state): Extension<Arc<AppState>>,
    headers: HeaderMap,
) -> Response {
    let mut cache = state
        .svgs
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let encoded = cache
        .icon
        .get_or_insert_with(|| CachedImage::new(render_icon_image(ICON_SIZE)))
        .encoded(ImageKind::Ico)
        .unwrap_or_else(|| unreachable!("ico encoding should always succeed"));
    respond_image_with_cache(&headers, encoded, ICON_CACHE_CONTROL)
}

pub async fn favicon_by_id(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> Response {
    let trimmed = id.trim();
    let Some((id, ext)) = trimmed.rsplit_once('.') else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let Some(kind) = ext_to_kind(ext) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let Some(link_id) = LinkId::parse(id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let now = Instant::now();
    if let Some(response) = cached_favicon(&state, link_id, &headers, kind, now) {
        return response;
    }

    let href = link_id.href();
    let fetch = tokio::task::spawn_blocking(move || fetch_best_icon(href));
    let image = tokio::time::timeout(FAVICON_FETCH_TIMEOUT, fetch)
        .await
        .ok()
        .and_then(Result::ok)
        .flatten()
        .map(|image| resize_square(image, ICON_SIZE / 2));
    let Some(image) = image else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let mut cache = state
        .favicons
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    cache.insert(link_id, CachedImage::new(image), now);
    drop(cache);

    cached_favicon(&state, link_id, &headers, kind, now)
        .unwrap_or_else(|| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}

fn cached_favicon(
    state: &AppState,
    id: LinkId,
    headers: &HeaderMap,
    kind: ImageKind,
    now: Instant,
) -> Option<Response> {
    let mut map = state
        .favicons
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let entry = map.get_mut(id, now)?;
    let encoded = entry.encoded(kind)?;
    Some(respond_image_with_cache(
        headers,
        encoded,
        ICON_CACHE_CONTROL,
    ))
}

fn insert_if_missing(headers: &mut HeaderMap, name: &str, value: &str) {
    if headers.contains_key(name) {
        return;
    }
    let Ok(name) = HeaderName::from_lowercase(name.as_bytes()) else {
        return;
    };
    let Ok(value) = HeaderValue::from_str(value) else {
        return;
    };
    headers.insert(name, value);
}

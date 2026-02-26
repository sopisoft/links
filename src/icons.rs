use std::sync::OnceLock;
use std::time::{Duration, Instant};

use reqwest::Url;
use reqwest::blocking::Client;

use crate::images::to_rgba;

const MAX_ICON_BYTES: usize = 1_500_000;
const MAX_HTML_BYTES: usize = 512_000;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(3);
const HTML_TIMEOUT: Duration = Duration::from_secs(2);
const FETCH_BUDGET: Duration = Duration::from_secs(6);
const MAX_ICON_CANDIDATES: usize = 16;
const COMMON_ICON_PATHS: [(&str, u32); 5] = [
    ("/apple-touch-icon.png", 180),
    ("/apple-touch-icon-precomposed.png", 180),
    ("/favicon.png", 64),
    ("/favicon.ico", 64),
    ("/favicon-32x32.png", 32),
];
const HTML_ACCEPT: &str = "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8";
const IMAGE_ACCEPT: &str = "image/avif,image/webp,image/png,image/*;q=0.8,*/*;q=0.5";

fn build_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .http1_only()
        .redirect(reqwest::redirect::Policy::limited(3))
        .user_agent("links-bot/1.0")
        .connect_timeout(Duration::from_secs(2))
        .timeout(Duration::from_secs(3))
        .no_proxy()
        .build()
}

pub fn fetch_best_icon(link: &str) -> Option<image::RgbaImage> {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    if CLIENT.get().is_none() {
        let _ = CLIENT.set(build_client().ok()?);
    }
    let client = CLIENT.get()?;
    let target = Url::parse(link).ok()?;
    if !is_http(&target) {
        return None;
    }
    let origin = target.join("/").ok()?;
    let html = fetch_html(client, &target);
    let mut candidates = collect_icon_candidates(&target, &origin, html.as_deref());
    if candidates.len() > MAX_ICON_CANDIDATES {
        candidates.truncate(MAX_ICON_CANDIDATES);
    }
    let start = Instant::now();
    let mut best: Option<FetchedIcon> = None;
    for candidate in candidates {
        if start.elapsed() > FETCH_BUDGET {
            break;
        }
        let Some(icon) = fetch_icon(client, &candidate.url) else {
            continue;
        };
        let replace = best.as_ref().is_none_or(|current| icon.size > current.size);
        if replace {
            best = Some(icon);
        }
    }
    best.map(|icon| icon.image)
}

#[derive(Clone, Debug)]
struct IconCandidate {
    url: Url,
    hint: u32,
    order: usize,
}

struct FetchedIcon {
    image: image::RgbaImage,
    size: u32,
}

fn collect_icon_candidates(base: &Url, origin: &Url, html: Option<&str>) -> Vec<IconCandidate> {
    let mut out = Vec::new();
    let mut order = 0;
    for (path, hint) in COMMON_ICON_PATHS {
        if let Ok(url) = origin.join(path) {
            push_unique(&mut out, IconCandidate { url, hint, order });
        }
        order += 1;
    }

    if let Some(html) = html {
        let bytes = html.as_bytes();
        let mut index = 0;
        while let Some(start) = find_link_start(bytes, index) {
            let Some((end, tag)) = extract_tag(html, start) else {
                break;
            };
            if let Some(candidate) = parse_link_tag(base, origin, tag, order) {
                push_unique(&mut out, candidate);
                order += 1;
            }
            index = end.saturating_add(1);
        }
    }

    out.sort_by(|a, b| b.hint.cmp(&a.hint).then_with(|| a.order.cmp(&b.order)));
    out
}

fn push_unique(out: &mut Vec<IconCandidate>, candidate: IconCandidate) {
    if !out.iter().any(|existing| existing.url == candidate.url) {
        out.push(candidate);
    }
}

fn parse_link_tag(base: &Url, origin: &Url, tag: &str, order: usize) -> Option<IconCandidate> {
    let tag_bytes = tag.as_bytes();
    let mut pos = 0;
    if tag_bytes.first() == Some(&b'<') {
        pos += 1;
    }
    if tag_bytes.get(pos) == Some(&b'/') {
        return None;
    }
    if !starts_with_ignore_ascii_case(&tag_bytes[pos..], b"link") {
        return None;
    }
    pos += 4;

    let mut rel: Option<String> = None;
    let mut href: Option<String> = None;
    let mut sizes: Option<String> = None;
    let mut kind: Option<String> = None;

    while pos < tag_bytes.len() {
        while pos < tag_bytes.len()
            && (tag_bytes[pos].is_ascii_whitespace() || tag_bytes[pos] == b'/')
        {
            pos += 1;
        }
        if pos >= tag_bytes.len() || tag_bytes[pos] == b'>' {
            break;
        }
        let name_start = pos;
        while pos < tag_bytes.len() && is_attr_name_char(tag_bytes[pos]) {
            pos += 1;
        }
        if name_start == pos {
            pos += 1;
            continue;
        }
        let name = &tag[name_start..pos];
        while pos < tag_bytes.len() && tag_bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if pos >= tag_bytes.len() || tag_bytes[pos] != b'=' {
            continue;
        }
        pos += 1;
        while pos < tag_bytes.len() && tag_bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if pos >= tag_bytes.len() {
            break;
        }
        let (value, next_pos) = parse_attr_value(tag, pos);
        pos = next_pos;
        let value = value.trim();
        if value.is_empty() {
            continue;
        }
        match name.to_ascii_lowercase().as_str() {
            "rel" => rel = Some(value.to_ascii_lowercase()),
            "href" => href = Some(decode_basic_html(value)),
            "sizes" => sizes = Some(value.to_ascii_lowercase()),
            "type" => kind = Some(value.to_ascii_lowercase()),
            _ => {}
        }
    }

    let rel = rel?;
    if !rel_is_icon(&rel) {
        return None;
    }
    let href = href?;
    if is_svg_hint(kind.as_deref(), &href) {
        return None;
    }
    let url = resolve_icon_url(base, origin, &href)?;
    let hint = sizes.as_deref().and_then(parse_sizes).unwrap_or(0);
    Some(IconCandidate { url, hint, order })
}

fn rel_is_icon(rel: &str) -> bool {
    rel.split_ascii_whitespace().any(|token| {
        matches!(
            token,
            "icon"
                | "apple-touch-icon"
                | "apple-touch-icon-precomposed"
                | "fluid-icon"
                | "shortcut"
        )
    })
}

fn parse_attr_value(tag: &str, mut pos: usize) -> (String, usize) {
    let bytes = tag.as_bytes();
    if bytes[pos] == b'"' || bytes[pos] == b'\'' {
        let quote = bytes[pos];
        pos += 1;
        let start = pos;
        while pos < bytes.len() && bytes[pos] != quote {
            pos += 1;
        }
        let value = tag[start..pos].to_string();
        if pos < bytes.len() {
            pos += 1;
        }
        return (value, pos);
    }
    let start = pos;
    while pos < bytes.len() && !bytes[pos].is_ascii_whitespace() && bytes[pos] != b'>' {
        pos += 1;
    }
    (tag[start..pos].to_string(), pos)
}

fn parse_sizes(value: &str) -> Option<u32> {
    let mut best: Option<u32> = None;
    for token in value.split_ascii_whitespace() {
        let Some((w, h)) = token.split_once('x') else {
            continue;
        };
        let Ok(width) = w.parse::<u32>() else {
            continue;
        };
        let Ok(height) = h.parse::<u32>() else {
            continue;
        };
        let size = width.max(height);
        best = Some(best.map_or(size, |current| current.max(size)));
    }
    best
}

fn resolve_icon_url(base: &Url, origin: &Url, href: &str) -> Option<Url> {
    let trimmed = href.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.starts_with("data:") || trimmed.starts_with("javascript:") {
        return None;
    }
    let url = if trimmed.starts_with("//") {
        let absolute = format!("{}:{}", base.scheme(), trimmed);
        Url::parse(&absolute).ok()?
    } else if let Ok(url) = Url::parse(trimmed) {
        url
    } else {
        base.join(trimmed).ok()?
    };
    if same_site(origin, &url) {
        Some(url)
    } else {
        None
    }
}

fn same_site(base: &Url, other: &Url) -> bool {
    let Some(base_host) = base.host_str() else {
        return false;
    };
    let Some(other_host) = other.host_str() else {
        return false;
    };
    let host_ok = other_host == base_host || other_host.ends_with(&format!(".{base_host}"));
    base.scheme() == other.scheme()
        && base.port_or_known_default() == other.port_or_known_default()
        && host_ok
}

fn is_svg_hint(kind: Option<&str>, href: &str) -> bool {
    if let Some(kind) = kind
        && kind.contains("svg")
    {
        return true;
    }
    href.ends_with(".svg") || href.ends_with(".svgz")
}

fn decode_basic_html(value: &str) -> String {
    if value.contains("&amp;") {
        return value.replace("&amp;", "&");
    }
    value.to_string()
}

fn fetch_html(client: &Client, url: &Url) -> Option<String> {
    let (bytes, content_type) =
        fetch_bytes(client, url, HTML_ACCEPT, HTML_TIMEOUT, MAX_HTML_BYTES)?;
    if let Some(content_type) = content_type
        && !(content_type.contains("text/html") || content_type.contains("application/xhtml"))
    {
        return None;
    }
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

fn fetch_icon(client: &Client, url: &Url) -> Option<FetchedIcon> {
    let (bytes, content_type) =
        fetch_bytes(client, url, IMAGE_ACCEPT, REQUEST_TIMEOUT, MAX_ICON_BYTES)?;
    let content_type = content_type.unwrap_or_default();
    if is_svg_hint(Some(content_type.as_str()), url.path()) {
        return None;
    }
    let decoded = image::load_from_memory(&bytes).ok()?;
    let size = decoded.width().max(decoded.height());
    Some(FetchedIcon {
        image: to_rgba(decoded),
        size,
    })
}

fn fetch_bytes(
    client: &Client,
    url: &Url,
    accept: &str,
    timeout: Duration,
    limit: usize,
) -> Option<(Vec<u8>, Option<String>)> {
    let res = client
        .get(url.clone())
        .header("accept", accept)
        .header("accept-encoding", "identity")
        .timeout(timeout)
        .send()
        .ok()?;
    if !res.status().is_success() {
        return None;
    }
    if let Some(length) = res.content_length()
        && length as usize > limit
    {
        return None;
    }
    let content_type = res
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.split(';').next().unwrap_or(value).trim().to_string());
    let bytes = res.bytes().ok()?.to_vec();
    if bytes.is_empty() || bytes.len() > limit {
        return None;
    }
    Some((bytes, content_type))
}

fn find_link_start(bytes: &[u8], from: usize) -> Option<usize> {
    let mut index = from;
    while index + 5 <= bytes.len() {
        if bytes[index] == b'<' {
            let mut tag = index + 1;
            if bytes.get(tag) == Some(&b'/') {
                tag += 1;
            }
            if tag + 4 <= bytes.len() && starts_with_ignore_ascii_case(&bytes[tag..], b"link") {
                let next = bytes.get(tag + 4).copied().unwrap_or(b' ');
                if next.is_ascii_whitespace() || next == b'>' || next == b'/' {
                    return Some(index);
                }
            }
        }
        index += 1;
    }
    None
}

fn extract_tag(html: &str, start: usize) -> Option<(usize, &str)> {
    const MAX_TAG_LEN: usize = 2048;
    let bytes = html.as_bytes();
    let mut index = start;
    let mut quote: Option<u8> = None;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'"' || byte == b'\'' {
            if quote == Some(byte) {
                quote = None;
            } else if quote.is_none() {
                quote = Some(byte);
            }
        } else if byte == b'>' && quote.is_none() {
            return Some((index, &html[start..=index]));
        }
        if index.saturating_sub(start) > MAX_TAG_LEN {
            return None;
        }
        index += 1;
    }
    None
}

fn is_attr_name_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'-' || byte == b':'
}

fn starts_with_ignore_ascii_case(slice: &[u8], expected: &[u8]) -> bool {
    if slice.len() < expected.len() {
        return false;
    }
    slice
        .iter()
        .take(expected.len())
        .zip(expected.iter())
        .all(|(left, right)| left.to_ascii_lowercase() == *right)
}

fn is_http(url: &Url) -> bool {
    matches!(url.scheme(), "http" | "https")
}

#[cfg(test)]
mod tests {
    use super::collect_icon_candidates;
    use reqwest::Url;

    #[test]
    fn parses_icon_link_candidates() {
        let base = Url::parse("https://example.com/profile").unwrap();
        let origin = base.join("/").unwrap();
        let html = r#"
            <html>
              <head>
                <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
                <link rel="apple-touch-icon" href="/apple-touch-icon.png" sizes="180x180" />
              </head>
            </html>
        "#;
        let candidates = collect_icon_candidates(&base, &origin, Some(html));
        assert!(
            !candidates
                .iter()
                .any(|c| c.url.as_str() == "https://example.com/favicon.svg")
        );
        assert!(
            candidates
                .iter()
                .any(|c| c.url.as_str() == "https://example.com/apple-touch-icon.png")
        );
    }

    #[test]
    fn ignores_cross_site_icons() {
        let base = Url::parse("https://example.com/profile").unwrap();
        let origin = base.join("/").unwrap();
        let html = r#"
            <link rel="icon" href="https://cdn.other.com/icon.png" />
        "#;
        let candidates = collect_icon_candidates(&base, &origin, Some(html));
        assert!(
            !candidates
                .iter()
                .any(|c| c.url.as_str() == "https://cdn.other.com/icon.png")
        );
    }
}

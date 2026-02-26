use axum::http::HeaderMap;
use axum::http::uri::Authority;
use maud::{DOCTYPE, Markup, html};

use crate::i18n::{Lang, description, title};
use crate::links::{LINKS, Link, LinkId};

const STYLES: &str = "/assets/styles.css";

#[derive(Clone, Debug, PartialEq)]
pub struct PageData {
    lang: Lang,
    og_image: String,
}

impl Default for PageData {
    fn default() -> Self {
        Self {
            lang: Lang::Ja,
            og_image: "/ogp.png".to_string(),
        }
    }
}

impl PageData {
    pub fn from_headers(headers: &HeaderMap) -> Self {
        let lang = detect_lang(headers);
        let og_image = og_image_from_headers(headers);
        Self { lang, og_image }
    }
}

pub fn render_page(data: PageData) -> String {
    let PageData { lang, og_image } = data;
    let page_title = title(lang);
    let markup = html! {
        (DOCTYPE)
        html lang=(lang.tag()) {
            head {
                (page_head(page_title, og_image.as_str()))
            }
            body {
                (page_body(page_title, lang))
            }
        }
    };
    markup.into_string()
}

pub const fn styles_css() -> &'static str {
    include_str!("../assets/styles.css")
}

fn page_head(page_title: &str, og_image: &str) -> Markup {
    html! {
        meta charset="UTF-8";
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        meta name="description" content=(page_title);
        meta property="og:title" content=(page_title);
        meta property="og:description" content=(page_title);
        meta property="og:image" content=(og_image);
        meta property="og:type" content="website";
        meta name="twitter:card" content="summary_large_image";
        meta name="twitter:title" content=(page_title);
        meta name="twitter:description" content=(page_title);
        meta name="twitter:image" content=(og_image);
        link rel="icon" type="image/png" href="/icon.png";
        link rel="apple-touch-icon" href="/icon.png";
        link rel="stylesheet" href=(STYLES);
        title { (page_title) }
    }
}

fn page_body(page_title: &str, lang: Lang) -> Markup {
    html! {
        main class="page" {
            article class="card" {
                (page_header(page_title))
                (link_list(lang))
            }
        }
    }
}

fn page_header(title: &str) -> Markup {
    html! {
        header class="header" {
            h1 class="title" { (title) }
        }
    }
}

fn link_list(lang: Lang) -> Markup {
    html! {
        ul class="list" {
            @for link in LINKS.iter() {
                (link_item(link, lang))
            }
        }
    }
}

fn link_item(link: &Link, lang: Lang) -> Markup {
    let id = link.id;
    let description_text = description(lang, id);
    let href = link.href;
    let text = link.text;
    let handle = link.handle;
    html! {
        li class="item" {
            a class="link" href=(href) target="_blank" rel="noreferrer noopener" {
                div class="item-inner" {
                    (link_favicon(id, text))
                    div class="title-wrap" {
                        div class="title-line" {
                            span { (text) }
                            span class="badge" { (handle) }
                        }
                        small class="desc" { (description_text) }
                    }
                }
            }
        }
    }
}

fn link_favicon(id: LinkId, text: &str) -> Markup {
    let id = id.as_str();
    html! {
        span class="icon-wrap" {
            picture {
                source srcset=(format_args!("/img/{}.avif", id)) r#type="image/avif";
                img
                    src=(format_args!("/img/{}.png", id))
                    alt=(format_args!("{} favicon", text))
                    width="64"
                    height="64"
                    loading="lazy"
                    decoding="async";
            }
        }
    }
}

fn detect_lang(headers: &HeaderMap) -> Lang {
    headers
        .get("accept-language")
        .and_then(|v| v.to_str().ok())
        .map(|raw| {
            raw.split(',')
                .fold((Lang::Ja, 0.0f32), |best, part| {
                    let mut p = part.trim().split(';');
                    let tag = p.next().unwrap_or("").trim();
                    let q = p
                        .find_map(|s| s.strip_prefix("q=").and_then(|s| s.parse::<f32>().ok()))
                        .unwrap_or(1.0);
                    let lang = lang_from_tag(tag);
                    match lang {
                        Some(l) if q > best.1 => (l, q),
                        _ => best,
                    }
                })
                .0
        })
        .unwrap_or(Lang::Ja)
}

fn lang_from_tag(tag: &str) -> Option<Lang> {
    if tag.len() < 2 {
        return None;
    }
    match &tag[..2].to_ascii_lowercase()[..] {
        "ja" => Some(Lang::Ja),
        "en" => Some(Lang::En),
        _ => None,
    }
}

fn og_image_from_headers(headers: &HeaderMap) -> String {
    let host = headers
        .get("host")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<Authority>().ok());
    let proto = headers
        .get("x-forwarded-proto")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.trim())
        .filter(|value| *value == "http" || *value == "https")
        .unwrap_or("https");
    match host {
        Some(host) => format!("{proto}://{host}/ogp.png"),
        None => "/ogp.png".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{detect_lang, og_image_from_headers};
    use axum::http::{HeaderMap, HeaderValue};

    use crate::i18n::Lang;

    #[test]
    fn detect_lang_honors_quality() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "accept-language",
            HeaderValue::from_static("en-US;q=0.4,ja;q=0.9"),
        );
        assert_eq!(detect_lang(&headers), Lang::Ja);
    }

    #[test]
    fn og_image_uses_proxy_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("host", HeaderValue::from_static("example.com"));
        headers.insert("x-forwarded-proto", HeaderValue::from_static("http"));
        assert_eq!(
            og_image_from_headers(&headers),
            "http://example.com/ogp.png"
        );
    }
}

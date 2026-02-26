mod assets;
mod i18n;
mod icons;
mod images;
mod links;
mod page;
mod routes;
mod state;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use axum::Extension;
use axum::Router;
use axum::middleware;
use axum::routing::get;
use tower_http::compression::CompressionLayer;
use tower_http::compression::predicate::NotForContentType;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::default());
    let router = Router::new()
        .route("/", get(routes::index))
        .route("/assets/styles.css", get(routes::styles))
        .route("/robots.txt", get(routes::robots))
        .route("/icon.svg", get(routes::icon_svg))
        .route("/ogp.svg", get(routes::ogp_svg))
        .route("/favicon.ico", get(routes::favicon_ico))
        .route("/icon.{ext}", get(routes::icon_raster))
        .route("/ogp.{ext}", get(routes::ogp_raster))
        .route("/img/{id}", get(routes::favicon_by_id))
        .route("/{*any}", get(routes::redirect))
        .layer(Extension(state))
        .layer(middleware::from_fn(routes::security_headers))
        .layer(CompressionLayer::new().compress_when(NotForContentType::IMAGES));

    let addr = server_addr();
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("failed to bind {addr}: {err}");
            return;
        }
    };
    if let Err(err) = axum::serve(listener, router).await {
        eprintln!("server error: {err}");
    }
}

fn server_addr() -> SocketAddr {
    let port = env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3000);
    let ip = env::var("IP")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED));
    SocketAddr::new(ip, port)
}

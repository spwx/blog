use axum::{
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Static;

pub async fn sitemap() -> Response {
    match Static::get("sitemap.xml") {
        Some(content) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                "application/xml".parse().unwrap(),
            );

            // Cache for 1 day - sitemap doesn't change often
            headers.insert(
                header::CACHE_CONTROL,
                "public, max-age=86400".parse().unwrap(),
            );

            (headers, content.data).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn robots() -> Response {
    match Static::get("robots.txt") {
        Some(content) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                "text/plain".parse().unwrap(),
            );

            // Cache for 1 day
            headers.insert(
                header::CACHE_CONTROL,
                "public, max-age=86400".parse().unwrap(),
            );

            (headers, content.data).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

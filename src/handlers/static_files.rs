use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Static;

pub async fn serve_static(Path(path): Path<String>) -> Response {
    match Static::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, mime.as_ref().parse().unwrap());

            // Set long cache headers for static assets (1 year)
            // Browser will cache these files, and Cloudflare will also cache them
            headers.insert(
                header::CACHE_CONTROL,
                "public, max-age=31536000, immutable".parse().unwrap(),
            );

            (headers, content.data).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

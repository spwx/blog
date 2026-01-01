use askama::Template;
use axum::{http::StatusCode, response::{Html, IntoResponse, Response}};

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate;

pub async fn not_found() -> Response {
    match NotFoundTemplate.render() {
        Ok(html) => (StatusCode::NOT_FOUND, Html(html)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

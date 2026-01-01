use askama::Template;
use axum::{http::StatusCode, response::{Html, IntoResponse, Response}};

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate {
    site_description: String,
}

pub async fn not_found() -> Response {
    let site_description = std::env::var("SITE_DESCRIPTION")
        .unwrap_or_else(|_| "A technical blog about software development, systems programming, electronics, cybersecurity, and engineering insights.".to_string());

    match (NotFoundTemplate { site_description }).render() {
        Ok(html) => (StatusCode::NOT_FOUND, Html(html)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

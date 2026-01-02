use crate::models::AppState;
use askama::Template;
use axum::{extract::State, http::StatusCode, response::{Html, IntoResponse, Response}};
use std::sync::Arc;

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate {
    site_name: String,
    site_description: String,
}

pub async fn not_found(State(state): State<Arc<AppState>>) -> Response {
    match (NotFoundTemplate {
        site_name: state.config.site.name.clone(),
        site_description: state.config.site.description.clone(),
    }).render() {
        Ok(html) => (StatusCode::NOT_FOUND, Html(html)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

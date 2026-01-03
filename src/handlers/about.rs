use crate::models::AppState;
use askama::Template;
use axum::{extract::State, http::StatusCode, response::{Html, IntoResponse}};
use std::sync::Arc;

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    content: String,
    site_name: String,
    default_theme: String,
}

pub async fn about(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match (AboutTemplate {
        content: state.about_content.clone(),
        site_name: state.config.site.name.clone(),
        default_theme: state.config.site.default_theme.clone(),
    }).render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

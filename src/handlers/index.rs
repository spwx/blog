use crate::models::{AppState, Post};
use askama::Template;
use axum::{extract::State, http::StatusCode, response::{Html, IntoResponse}};
use std::sync::Arc;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    posts: Vec<Post>,
    site_name: String,
    site_description: String,
    default_theme: String,
}

pub async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut posts: Vec<Post> = state.posts.values().cloned().collect();
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    match (IndexTemplate {
        posts,
        site_name: state.config.site.name.clone(),
        site_description: state.config.site.description.clone(),
        default_theme: state.config.site.default_theme.clone(),
    }).render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

use crate::models::{AppState, Post};
use askama::Template;
use axum::{extract::{Path, State}, http::StatusCode, response::{Html, IntoResponse}};
use std::sync::Arc;

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    post: Post,
}

pub async fn post(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match state.posts.get(&slug).cloned() {
        Some(post) => match (PostTemplate { post }).render() {
            Ok(html) => Html(html).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

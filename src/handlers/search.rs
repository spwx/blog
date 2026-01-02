use crate::models::{AppState, SearchQuery, SearchResult};
use crate::utils::generate_excerpt;
use askama::Template;
use axum::{extract::{Query, State}, http::StatusCode, response::{Html, IntoResponse}};
use std::{sync::Arc, time::Duration};
use tokio::time::timeout;

#[derive(Template)]
#[template(path = "search.html")]
struct SearchTemplate {
    query: String,
    results: Vec<SearchResult>,
    site_name: String,
    site_description: String,
}

pub async fn search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> impl IntoResponse {
    const MAX_QUERY_LENGTH: usize = 200;
    const SEARCH_TIMEOUT: Duration = Duration::from_secs(5);

    let query = params.q.unwrap_or_default().trim().to_string();
    let query = if query.len() > MAX_QUERY_LENGTH {
        query.chars().take(MAX_QUERY_LENGTH).collect()
    } else {
        query
    };

    let search_future = async {
        let mut results: Vec<SearchResult> = if query.is_empty() {
            Vec::new()
        } else {
            let query_lower = query.to_lowercase();
            state
                .posts
                .values()
                .filter(|post| {
                    post.title_lower.contains(&query_lower)
                        || post.content_lower.contains(&query_lower)
                })
                .map(|post| {
                    let excerpt = if post.title_lower.contains(&query_lower) {
                        // If title matches, show beginning of content
                        generate_excerpt(&post.content, "", 200)
                    } else {
                        // If content matches, show context around match
                        generate_excerpt(&post.content, &query, 200)
                    };
                    SearchResult {
                        post: post.clone(),
                        excerpt,
                    }
                })
                .collect()
        };

        // Sort results by date (newest first)
        results.sort_by(|a, b| b.post.date.cmp(&a.post.date));
        results
    };

    let results = match timeout(SEARCH_TIMEOUT, search_future).await {
        Ok(results) => results,
        Err(_) => {
            // Timeout occurred, return empty results
            Vec::new()
        }
    };

    match (SearchTemplate {
        query,
        results,
        site_name: state.config.site.name.clone(),
        site_description: state.config.site.description.clone(),
    }).render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

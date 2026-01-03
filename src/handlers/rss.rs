use crate::models::AppState;
use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use rss::{Channel, ChannelBuilder, ItemBuilder};
use std::sync::Arc;

pub async fn rss(State(state): State<Arc<AppState>>) -> Response {
    // Sort posts by date, newest first
    let mut posts: Vec<_> = state.posts.values().collect();
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    // Get the domain or use a default
    let domain = state.config.site.domain.as_deref().unwrap_or("http://localhost:3000");

    // Build RSS items from posts
    let items: Vec<_> = posts
        .iter()
        .map(|post| {
            ItemBuilder::default()
                .title(Some(post.title.clone()))
                .link(Some(format!("{}/post/{}", domain, post.slug)))
                .description(Some(post.description.clone()))
                .pub_date(Some(post.date.format("%a, %d %b %Y 00:00:00 +0000").to_string()))
                .build()
        })
        .collect();

    // Build RSS channel
    let channel = ChannelBuilder::default()
        .title(&state.config.site.name)
        .link(domain)
        .description(&state.config.site.description)
        .items(items)
        .build();

    // Serialize to XML
    match channel_to_string(&channel) {
        Ok(xml) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                "application/rss+xml; charset=utf-8".parse().unwrap(),
            );

            // Cache for 1 hour - RSS feeds update when new posts are added
            headers.insert(
                header::CACHE_CONTROL,
                "public, max-age=3600".parse().unwrap(),
            );

            (headers, xml).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

fn channel_to_string(channel: &Channel) -> Result<String, rss::Error> {
    let mut buffer = Vec::new();
    channel.write_to(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

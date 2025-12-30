use crate::handlers::{index, post, search, serve_static};
use crate::models::AppState;
use crate::parsing::parse_posts;
use anyhow::{Context, Result};
use axum::routing::get;
use axum::Router;
use std::{net::SocketAddr, sync::Arc};
use tower_http::compression::CompressionLayer;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};

pub async fn run() -> Result<()> {
    let posts = parse_posts()
        .context("Failed to parse blog posts during startup")?;
    let state = Arc::new(AppState { posts });

    // Configure rate limiter: 10 requests per second with burst of 20
    // SmartIpKeyExtractor reads X-Forwarded-For header to get real client IP behind Cloudflare
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(10)
            .burst_size(20)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .context("Failed to build rate limiter configuration")?,
    );

    let app = Router::new()
        .route("/", get(index))
        .route("/search", get(search))
        .route("/post/{slug}", get(post))
        .route("/static/{*path}", get(serve_static))
        .with_state(state)
        .layer(GovernorLayer::new(governor_conf))
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .context("Failed to bind to port 3000 - is it already in use?")?;

    println!("Blog server running on http://127.0.0.1:3000");
    println!("Compression: enabled (gzip)");
    println!("Rate limiting: 10 req/sec per IP, burst 20");

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .context("Server error during operation")?;

    Ok(())
}

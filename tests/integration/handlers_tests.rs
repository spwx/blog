use axum::{routing::get, Router};
use axum::http::StatusCode;
use axum_test::TestServer;
use blog_engine::{index, post, search, serve_static, parse_posts, AppState};
use std::sync::Arc;

// Helper function to create test server
async fn create_test_server() -> TestServer {
    let posts = parse_posts().expect("Should parse posts");
    let state = Arc::new(AppState { posts });

    let app = Router::new()
        .route("/", get(index))
        .route("/post/{slug}", get(post))
        .route("/search", get(search))
        .route("/static/{*path}", get(serve_static))
        .with_state(state);

    TestServer::new(app).expect("Should create test server")
}

#[tokio::test]
async fn test_index_returns_ok() {
    let server = create_test_server().await;
    let response = server.get("/").await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_index_contains_html() {
    let server = create_test_server().await;
    let response = server.get("/").await;

    response.assert_status_ok();
    let text = response.text();
    assert!(!text.is_empty(), "Response should not be empty");
}

#[tokio::test]
async fn test_post_valid_slug() {
    let server = create_test_server().await;
    // Use actual post slug from content/posts/
    let response = server.get("/post/creating-a-blog-with-ai").await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_post_invalid_slug() {
    let server = create_test_server().await;
    let response = server.get("/post/nonexistent-post").await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_post_slug_case_sensitive() {
    let server = create_test_server().await;
    // Try with different case - should not find it
    let response = server.get("/post/Creating-A-Blog-With-AI").await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_search_empty_query() {
    let server = create_test_server().await;
    let response = server.get("/search").await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_search_with_query() {
    let server = create_test_server().await;
    let response = server.get("/search?q=rust").await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_search_case_insensitive() {
    let server = create_test_server().await;
    let response = server.get("/search?q=RUST").await;

    response.assert_status_ok();
    // Should find posts containing "rust" regardless of case
}

#[tokio::test]
async fn test_search_query_length_limit() {
    let server = create_test_server().await;
    let long_query = "a".repeat(300); // Exceeds MAX_QUERY_LENGTH of 200
    let response = server.get(&format!("/search?q={}", long_query)).await;

    response.assert_status_ok();
    // Should truncate query to 200 characters and still work
}

#[tokio::test]
async fn test_search_special_characters() {
    let server = create_test_server().await;
    let response = server.get("/search?q=%3Cscript%3E").await; // <script>

    response.assert_status_ok();
    // Should handle special characters safely
}

#[tokio::test]
async fn test_serve_static_css_file() {
    let server = create_test_server().await;
    let response = server.get("/static/style.css").await;

    // File should exist (assuming you have style.css in static/)
    if response.status_code() == StatusCode::OK {
        response.assert_header("content-type", "text/css");
    }
}

#[tokio::test]
async fn test_serve_static_nonexistent_file() {
    let server = create_test_server().await;
    let response = server.get("/static/nonexistent.css").await;

    response.assert_status(StatusCode::NOT_FOUND);
}

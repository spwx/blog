use anyhow::{Context, Result};
use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::sync::LazyLock;
use chrono::NaiveDate;
use orgize::{
    export::{DefaultHtmlHandler, HtmlHandler},
    Element, Event, Org,
};
use regex::Regex;
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::{collections::HashMap, io::Write, net::SocketAddr, sync::Arc, time::Duration};
use syntect::{
    html::ClassedHTMLGenerator,
    parsing::SyntaxSet,
    util::LinesWithEndings,
};
use tokio::time::timeout;
use tower_http::compression::CompressionLayer;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};

// Include generated metadata from build.rs
include!(concat!(env!("OUT_DIR"), "/generated_metadata.rs"));

#[derive(RustEmbed)]
#[folder = "static/"]
struct Static;

#[derive(RustEmbed)]
#[folder = "content/posts/"]
struct Posts;

#[derive(Clone, Debug)]
struct Post {
    slug: String,
    title: String,
    date: NaiveDate,
    updated: String,
    content: String,
    title_lower: String,
    content_lower: String,
}

struct AppState {
    posts: HashMap<String, Post>,
}

struct SyntectHandler {
    syntax_set: SyntaxSet,
    default: DefaultHtmlHandler,
}

impl Default for SyntectHandler {
    fn default() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            default: DefaultHtmlHandler,
        }
    }
}

impl HtmlHandler<std::io::Error> for SyntectHandler {
    fn start<W: Write>(
        &mut self,
        mut w: W,
        element: &Element,
    ) -> Result<(), std::io::Error> {
        if let Element::SourceBlock(block) = element {
            let lang = block.language.as_ref();
            let syntax = self
                .syntax_set
                .find_syntax_by_token(lang)
                .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

            let mut generator = ClassedHTMLGenerator::new_with_class_style(
                syntax,
                &self.syntax_set,
                syntect::html::ClassStyle::Spaced,
            );

            for line in LinesWithEndings::from(&block.contents) {
                if let Err(e) = generator.parse_html_for_line_which_includes_newline(line) {
                    eprintln!("Syntax highlighting error: {}", e);
                    // Fall back to plain text rendering
                    let escaped = html_escape::encode_text(&block.contents);
                    write!(w, "<pre class=\"code\"><code>{}</code></pre>", escaped)?;
                    return Ok(());
                }
            }

            let html = generator.finalize();
            write!(w, "<pre class=\"code\"><code>{}</code></pre>", html)?;
            Ok(())
        } else {
            self.default.start(w, element)
        }
    }

    fn end<W: Write>(
        &mut self,
        w: W,
        element: &Element,
    ) -> Result<(), std::io::Error> {
        if matches!(element, Element::SourceBlock(_)) {
            Ok(())
        } else {
            self.default.end(w, element)
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    post: Post,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

#[derive(Clone)]
struct SearchResult {
    post: Post,
    excerpt: String,
}

#[derive(Template)]
#[template(path = "search.html")]
struct SearchTemplate {
    query: String,
    results: Vec<SearchResult>,
}

static TAG_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<[^>]*>").expect("TAG_REGEX: hardcoded pattern is invalid")
});

static WHITESPACE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\s+").expect("WHITESPACE_REGEX: hardcoded pattern is invalid")
});

fn strip_html_tags(html: &str) -> String {
    let without_tags = TAG_REGEX.replace_all(html, " ");
    let decoded = html_escape::decode_html_entities(&without_tags);
    WHITESPACE_REGEX.replace_all(&decoded, " ").trim().to_string()
}

fn generate_excerpt(content: &str, query: &str, max_length: usize) -> String {
    let text = strip_html_tags(content);
    let query_lower = query.to_lowercase();
    let text_lower = text.to_lowercase();

    if let Some(pos) = text_lower.find(&query_lower) {
        let start = if pos > 60 {
            // Find word boundary before the match
            text[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0)
        } else {
            0
        };

        let end = (start + max_length).min(text.len());
        let end = if end < text.len() {
            // Find word boundary after max_length
            text[start..end].rfind(' ').map(|i| start + i).unwrap_or(end)
        } else {
            end
        };

        let mut excerpt = text[start..end].to_string();
        if start > 0 {
            excerpt = format!("...{}", excerpt);
        }
        if end < text.len() {
            excerpt = format!("{}...", excerpt);
        }
        excerpt
    } else {
        // If no match found, return beginning of text
        let end = max_length.min(text.len());
        let end = if end < text.len() {
            text[..end].rfind(' ').unwrap_or(end)
        } else {
            end
        };
        let mut excerpt = text[..end].to_string();
        if end < text.len() {
            excerpt = format!("{}...", excerpt);
        }
        excerpt
    }
}

async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut posts: Vec<Post> = state.posts.values().cloned().collect();
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    match (IndexTemplate { posts }).render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn post(
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

async fn search(
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

    match (SearchTemplate { query, results }).render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn serve_static(Path(path): Path<String>) -> Response {
    match Static::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            (
                [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                content.data,
            )
                .into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

fn parse_posts() -> Result<HashMap<String, Post>> {
    let mut posts = HashMap::new();

    for filename in Posts::iter() {
        let filename_str = filename.as_ref();
        if !filename_str.ends_with(".org") {
            continue;
        }

        let slug = filename_str.trim_end_matches(".org").to_string();
        let content = Posts::get(filename_str)
            .with_context(|| format!("Failed to load embedded post file: {}", filename_str))?;
        let text = std::str::from_utf8(content.data.as_ref())
            .context("Post file contains invalid UTF-8")?;

        let org = Org::parse(text);

        let mut title = String::new();
        let mut date = None;

        for event in org.iter() {
            if let Event::Start(Element::Keyword(keyword)) = event {
                match keyword.key.to_uppercase().as_str() {
                    "TITLE" => title = keyword.value.to_string(),
                    "DATE" => {
                        date = NaiveDate::parse_from_str(&keyword.value, "%Y-%m-%d").ok();
                    }
                    _ => {}
                }
            }
        }

        let mut handler = SyntectHandler::default();
        let mut html_bytes = Vec::new();
        org.write_html_custom(&mut html_bytes, &mut handler)
            .context("Failed to generate HTML from org-mode content")?;
        let html = String::from_utf8(html_bytes)
            .context("Generated HTML contains invalid UTF-8")?;

        // Safe unwrap: 1970-01-01 is a valid date
        let pub_date = date.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

        // Look up last updated date from git metadata
        let updated = POST_UPDATED_DATES
            .iter()
            .find(|(fname, _)| *fname == filename_str)
            .and_then(|(_, date_str)| {
                NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .ok()
                    .filter(|updated_date| updated_date != &pub_date)
                    .map(|_| date_str.to_string())
            })
            .unwrap_or_default();

        posts.insert(
            slug.clone(),
            Post {
                slug,
                title_lower: title.to_lowercase(),
                title,
                date: pub_date,
                updated,
                content_lower: html.to_lowercase(),
                content: html,
            },
        );
    }

    Ok(posts)
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Fatal error: {:?}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // Tests for strip_html_tags()
    #[test]
    fn test_strip_html_tags_simple() {
        let input = "<p>Hello <strong>world</strong></p>";
        let expected = "Hello world";
        assert_eq!(strip_html_tags(input), expected);
    }

    #[test]
    fn test_strip_html_tags_with_entities() {
        let input = "<p>Hello &amp; goodbye &quot;world&quot;</p>";
        let expected = "Hello & goodbye \"world\"";
        assert_eq!(strip_html_tags(input), expected);
    }

    #[test]
    fn test_strip_html_tags_nested() {
        let input = "<div><p><span>Nested</span> content</p></div>";
        let expected = "Nested content";
        assert_eq!(strip_html_tags(input), expected);
    }

    #[test]
    fn test_strip_html_tags_multiple_spaces() {
        let input = "<p>Multiple    spaces   here</p>";
        let expected = "Multiple spaces here";
        assert_eq!(strip_html_tags(input), expected);
    }

    #[test]
    fn test_strip_html_tags_empty() {
        assert_eq!(strip_html_tags(""), "");
    }

    // Tests for generate_excerpt()
    #[test]
    fn test_generate_excerpt_with_match_in_middle() {
        let content = "<p>This is a long content with search term somewhere in the middle and more text after it continues</p>";
        let query = "search term";
        let excerpt = generate_excerpt(content, query, 50);

        assert!(excerpt.contains("search term"));
        assert!(excerpt.len() <= 55); // Account for "..."
    }

    #[test]
    fn test_generate_excerpt_no_match() {
        let content = "<p>This is some content without the query</p>";
        let query = "missing";
        let excerpt = generate_excerpt(content, query, 30);

        assert!(excerpt.starts_with("This is"));
        assert!(excerpt.ends_with("...") || excerpt.len() <= 30);
    }

    #[test]
    fn test_generate_excerpt_query_at_start() {
        let content = "<p>search term is at the beginning of this content</p>";
        let query = "search term";
        let excerpt = generate_excerpt(content, query, 50);

        assert!(excerpt.contains("search term"));
        assert!(!excerpt.starts_with("..."));
    }

    #[test]
    fn test_generate_excerpt_short_content() {
        let content = "<p>Short</p>";
        let query = "query";
        let excerpt = generate_excerpt(content, query, 100);

        assert_eq!(excerpt, "Short");
        assert!(!excerpt.ends_with("..."));
    }

    #[test]
    fn test_generate_excerpt_empty_query() {
        let content = "<p>Some content here</p>";
        let excerpt = generate_excerpt(content, "", 20);

        assert!(!excerpt.is_empty());
    }

    #[test]
    fn test_generate_excerpt_empty_content() {
        let excerpt = generate_excerpt("", "query", 100);
        assert_eq!(excerpt, "");
    }
}

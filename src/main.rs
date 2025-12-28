use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use chrono::NaiveDate;
use orgize::{
    export::{DefaultHtmlHandler, HtmlHandler},
    Element, Event, Org,
};
use regex::Regex;
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::{collections::HashMap, io::Write, sync::Arc};
use syntect::{
    html::ClassedHTMLGenerator,
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

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
    content: String,
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
                generator.parse_html_for_line_which_includes_newline(line).unwrap();
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

fn decode_html_entities(text: &str) -> String {
    text.replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&nbsp;", " ")
}

fn strip_html_tags(html: &str) -> String {
    let tag_regex = Regex::new(r"<[^>]*>").unwrap();
    let without_tags = tag_regex.replace_all(html, " ");
    let whitespace_regex = Regex::new(r"\s+").unwrap();
    let decoded = decode_html_entities(&without_tags);
    whitespace_regex.replace_all(&decoded, " ").trim().to_string()
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

    IndexTemplate { posts }
}

async fn post(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    state
        .posts
        .get(&slug)
        .cloned()
        .map(|post| PostTemplate { post })
        .ok_or(StatusCode::NOT_FOUND)
}

async fn search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> impl IntoResponse {
    let query = params.q.unwrap_or_default().trim().to_string();

    let mut results: Vec<SearchResult> = if query.is_empty() {
        Vec::new()
    } else {
        let query_lower = query.to_lowercase();
        state
            .posts
            .values()
            .filter(|post| {
                post.title.to_lowercase().contains(&query_lower)
                    || post.content.to_lowercase().contains(&query_lower)
            })
            .map(|post| {
                let excerpt = if post.title.to_lowercase().contains(&query_lower) {
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

    SearchTemplate { query, results }
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

fn parse_posts() -> HashMap<String, Post> {
    let mut posts = HashMap::new();

    for filename in Posts::iter() {
        let filename_str = filename.as_ref();
        if !filename_str.ends_with(".org") {
            continue;
        }

        let slug = filename_str.trim_end_matches(".org").to_string();
        let content = Posts::get(filename_str).unwrap();
        let text = std::str::from_utf8(content.data.as_ref()).unwrap();

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
        org.write_html_custom(&mut html_bytes, &mut handler).unwrap();
        let html = String::from_utf8(html_bytes).unwrap();

        posts.insert(
            slug.clone(),
            Post {
                slug,
                title,
                date: date.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
                content: html,
            },
        );
    }

    posts
}

#[tokio::main]
async fn main() {
    let posts = parse_posts();
    let state = Arc::new(AppState { posts });

    let app = Router::new()
        .route("/", get(index))
        .route("/search", get(search))
        .route("/post/:slug", get(post))
        .route("/static/*path", get(serve_static))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Blog server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

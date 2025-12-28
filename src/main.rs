use askama::Template;
use axum::{
    extract::{Path, State},
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
use rust_embed::RustEmbed;
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
        .route("/post/:slug", get(post))
        .route("/static/*path", get(serve_static))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Blog server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

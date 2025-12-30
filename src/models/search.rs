use super::Post;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

#[derive(Clone)]
pub struct SearchResult {
    pub post: Post,
    pub excerpt: String,
}

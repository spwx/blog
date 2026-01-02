mod models;
mod utils;
mod parsing;
mod handlers;
mod server;

// Re-export models for public API
pub use models::{Post, AppState, SearchQuery, SearchResult, SiteConfig};

// Re-export parsing functions
pub use parsing::parse_posts;

// Re-export handlers
pub use handlers::{index, post, search, serve_static};

// Re-export server
pub use server::run;

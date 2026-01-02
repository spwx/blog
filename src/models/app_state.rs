use super::{Post, SiteConfig};
use std::collections::HashMap;

pub struct AppState {
    pub posts: HashMap<String, Post>,
    pub config: SiteConfig,
}

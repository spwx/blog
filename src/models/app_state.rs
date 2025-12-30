use super::Post;
use std::collections::HashMap;

pub struct AppState {
    pub posts: HashMap<String, Post>,
}

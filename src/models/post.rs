use chrono::NaiveDate;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct TocItem {
    pub id: String,
    pub text: String,
    pub level: usize,
}

#[derive(Clone, Debug)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: NaiveDate,
    pub updated: String,
    pub description: String,
    pub content: String,
    pub title_lower: String,
    pub content_lower: String,
    pub toc: Vec<TocItem>,
}

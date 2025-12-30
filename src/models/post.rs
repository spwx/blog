use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: NaiveDate,
    pub updated: String,
    pub content: String,
    pub title_lower: String,
    pub content_lower: String,
}

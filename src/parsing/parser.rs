use crate::models::Post;
use super::SyntectHandler;
use anyhow::{Context, Result};
use chrono::NaiveDate;
use orgize::{Element, Event, Org};
use rust_embed::RustEmbed;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "content/posts/"]
struct Posts;

// Include generated metadata from build.rs
include!(concat!(env!("OUT_DIR"), "/generated_metadata.rs"));

pub fn parse_posts() -> Result<HashMap<String, Post>> {
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
        let mut description = String::new();

        for event in org.iter() {
            if let Event::Start(Element::Keyword(keyword)) = event {
                match keyword.key.to_uppercase().as_str() {
                    "TITLE" => title = keyword.value.to_string(),
                    "DATE" => {
                        date = NaiveDate::parse_from_str(&keyword.value, "%Y-%m-%d").ok();
                    }
                    "DESCRIPTION" => description = keyword.value.to_string(),
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
        let toc = handler.into_toc();

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
                description,
                content_lower: html.to_lowercase(),
                content: html,
                toc,
            },
        );
    }

    Ok(posts)
}

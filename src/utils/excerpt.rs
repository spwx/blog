use super::html::strip_html_tags;

pub fn generate_excerpt(content: &str, query: &str, max_length: usize) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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

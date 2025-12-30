use regex::Regex;
use std::sync::LazyLock;

static TAG_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<[^>]*>").expect("TAG_REGEX: hardcoded pattern is invalid")
});

static WHITESPACE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\s+").expect("WHITESPACE_REGEX: hardcoded pattern is invalid")
});

pub fn strip_html_tags(html: &str) -> String {
    let without_tags = TAG_REGEX.replace_all(html, " ");
    let decoded = html_escape::decode_html_entities(&without_tags);
    WHITESPACE_REGEX.replace_all(&decoded, " ").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_strip_html_tags_simple() {
        let input = "<p>Hello <strong>world</strong></p>";
        let expected = "Hello world";
        assert_eq!(strip_html_tags(input), expected);
    }

    #[test]
    fn test_strip_html_tags_with_entities() {
        let input = "<p>Hello &amp; goodbye &quot;world&quot;</p>";
        let expected = "Hello & goodbye \"world\"";
        assert_eq!(strip_html_tags(input), expected);
    }

    #[test]
    fn test_strip_html_tags_nested() {
        let input = "<div><p><span>Nested</span> content</p></div>";
        let expected = "Nested content";
        assert_eq!(strip_html_tags(input), expected);
    }

    #[test]
    fn test_strip_html_tags_multiple_spaces() {
        let input = "<p>Multiple    spaces   here</p>";
        let expected = "Multiple spaces here";
        assert_eq!(strip_html_tags(input), expected);
    }

    #[test]
    fn test_strip_html_tags_empty() {
        assert_eq!(strip_html_tags(""), "");
    }
}

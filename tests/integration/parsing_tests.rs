use blog_engine::parse_posts;
use chrono::Datelike;

#[test]
fn test_parse_posts_loads_existing_posts() {
    // Will use actual posts in content/posts/
    let posts = parse_posts().expect("Should parse posts successfully");

    assert!(!posts.is_empty(), "Should load at least one post");
}

#[test]
fn test_parse_posts_extracts_frontmatter() {
    let posts = parse_posts().expect("Should parse posts successfully");

    for (slug, post) in posts.iter() {
        // Each post should have extracted metadata
        assert!(!post.title.is_empty(), "Post {} should have title", slug);
        assert!(post.date.year() >= 1970, "Post {} should have valid date", slug);
        assert_eq!(post.slug, *slug, "Slug should match key");
    }
}

#[test]
fn test_parse_posts_generates_html() {
    let posts = parse_posts().expect("Should parse posts successfully");

    for (slug, post) in posts.iter() {
        // HTML should be generated
        assert!(!post.content.is_empty(), "Post {} should have content", slug);

        // Should contain HTML tags (basic sanity check)
        assert!(
            post.content.contains("<") && post.content.contains(">"),
            "Post {} content should be HTML",
            slug
        );
    }
}

#[test]
fn test_parse_posts_lowercase_search_fields() {
    let posts = parse_posts().expect("Should parse posts successfully");

    for (slug, post) in posts.iter() {
        // Lowercase fields should be properly generated
        assert_eq!(
            post.title_lower,
            post.title.to_lowercase(),
            "Post {} title_lower should match",
            slug
        );
        assert_eq!(
            post.content_lower,
            post.content.to_lowercase(),
            "Post {} content_lower should match",
            slug
        );
    }
}

#[test]
fn test_parse_posts_slug_format() {
    let posts = parse_posts().expect("Should parse posts successfully");

    for (slug, post) in posts.iter() {
        // Slug should be valid (no .org extension, lowercase)
        assert!(!slug.contains(".org"), "Slug {} should not contain .org", slug);
        assert_eq!(post.slug, *slug, "Post slug field should match key");
    }
}

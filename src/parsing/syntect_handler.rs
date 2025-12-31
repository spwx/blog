use crate::models::TocItem;
use orgize::{export::{DefaultHtmlHandler, HtmlHandler}, Element};
use std::io::Write;
use syntect::{html::ClassedHTMLGenerator, parsing::SyntaxSet, util::LinesWithEndings};

pub struct SyntectHandler {
    syntax_set: SyntaxSet,
    default: DefaultHtmlHandler,
    toc: Vec<TocItem>,
    heading_counter: usize,
}

impl Default for SyntectHandler {
    fn default() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            default: DefaultHtmlHandler,
            toc: Vec::new(),
            heading_counter: 0,
        }
    }
}

impl SyntectHandler {
    pub fn into_toc(self) -> Vec<TocItem> {
        self.toc
    }

    /// Clean org-mode link syntax from text
    /// Converts [[URL][link text]] to "link text"
    /// Converts [[URL]] to "URL"
    fn clean_org_links(text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '[' && chars.peek() == Some(&'[') {
                chars.next(); // consume second '['

                // Extract the URL part
                let mut url = String::new();
                while let Some(c) = chars.next() {
                    if c == ']' && chars.peek() == Some(&']') {
                        chars.next(); // consume second ']'
                        // No link text, use URL
                        result.push_str(&url);
                        break;
                    } else if c == ']' && chars.peek() == Some(&'[') {
                        chars.next(); // consume '['
                        // Extract link text
                        let mut link_text = String::new();
                        while let Some(t) = chars.next() {
                            if t == ']' && chars.peek() == Some(&']') {
                                chars.next(); // consume second ']'
                                result.push_str(&link_text);
                                break;
                            } else {
                                link_text.push(t);
                            }
                        }
                        break;
                    } else {
                        url.push(c);
                    }
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Convert text to URL-friendly slug
    /// "What's Nix?" -> "whats-nix"
    /// "Setting Up Direnv" -> "setting-up-direnv"
    fn slugify(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c
                } else if c.is_whitespace() {
                    '-'
                } else {
                    ' ' // Will be filtered out
                }
            })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("")
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }
}

impl HtmlHandler<std::io::Error> for SyntectHandler {
    fn start<W: Write>(
        &mut self,
        mut w: W,
        element: &Element,
    ) -> Result<(), std::io::Error> {
        match element {
            Element::SourceBlock(block) => {
                let lang = block.language.as_ref();
                let syntax = self
                    .syntax_set
                    .find_syntax_by_token(lang)
                    .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

                let mut generator = ClassedHTMLGenerator::new_with_class_style(
                    syntax,
                    &self.syntax_set,
                    syntect::html::ClassStyle::Spaced,
                );

                for line in LinesWithEndings::from(&block.contents) {
                    if let Err(e) = generator.parse_html_for_line_which_includes_newline(line) {
                        eprintln!("Syntax highlighting error: {}", e);
                        // Fall back to plain text rendering
                        let escaped = html_escape::encode_text(&block.contents);
                        write!(w, "<pre class=\"code\"><code>{}</code></pre>", escaped)?;
                        return Ok(());
                    }
                }

                let html = generator.finalize();
                write!(w, "<pre class=\"code\"><code>{}</code></pre>", html)?;
                Ok(())
            }
            Element::Title(title) => {
                // Clean org-mode link syntax from title text
                let clean_text = Self::clean_org_links(title.raw.trim());

                // Generate slug from cleaned text
                let slug = Self::slugify(&clean_text);

                // Handle potential duplicates by appending counter if needed
                let id = if self.toc.iter().any(|item| item.id == slug) {
                    format!("{}-{}", slug, self.heading_counter)
                } else {
                    slug
                };

                self.heading_counter += 1;

                // Add to TOC
                self.toc.push(TocItem {
                    id: id.clone(),
                    text: clean_text,
                    level: title.level as usize,
                });

                // Write the heading with ID
                write!(w, "<h{} id=\"{}\">", title.level, id)?;
                Ok(())
            }
            _ => self.default.start(w, element),
        }
    }

    fn end<W: Write>(
        &mut self,
        mut w: W,
        element: &Element,
    ) -> Result<(), std::io::Error> {
        match element {
            Element::SourceBlock(_) => Ok(()),
            Element::Title(title) => {
                write!(w, "</h{}>", title.level)?;
                Ok(())
            }
            _ => self.default.end(w, element),
        }
    }
}

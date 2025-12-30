use orgize::{export::{DefaultHtmlHandler, HtmlHandler}, Element};
use std::io::Write;
use syntect::{html::ClassedHTMLGenerator, parsing::SyntaxSet, util::LinesWithEndings};

pub struct SyntectHandler {
    syntax_set: SyntaxSet,
    default: DefaultHtmlHandler,
}

impl Default for SyntectHandler {
    fn default() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            default: DefaultHtmlHandler,
        }
    }
}

impl HtmlHandler<std::io::Error> for SyntectHandler {
    fn start<W: Write>(
        &mut self,
        mut w: W,
        element: &Element,
    ) -> Result<(), std::io::Error> {
        if let Element::SourceBlock(block) = element {
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
        } else {
            self.default.start(w, element)
        }
    }

    fn end<W: Write>(
        &mut self,
        w: W,
        element: &Element,
    ) -> Result<(), std::io::Error> {
        if matches!(element, Element::SourceBlock(_)) {
            Ok(())
        } else {
            self.default.end(w, element)
        }
    }
}

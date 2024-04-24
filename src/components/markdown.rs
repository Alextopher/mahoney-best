use std::{cell::RefCell, time::Duration};

use comrak::{
    arena_tree::Node,
    nodes::{Ast, AstNode, NodeHtmlBlock, NodeValue},
    Arena, Options,
};
use lol_html::{element, Settings};
use maud::{html, Render};
use once_cell::sync::Lazy;
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};

const FRONT_MATTER_DELIMITER: &str = "---";
const THEME: &str = "base16-eighties.dark";

static PS: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static TS: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

/// Wrapper around markdown content, implementing the [`Render`](maud::Render) trait.
///
/// # Features
///
/// - Render markdown content to HTML (comrak)
/// - Front Matter Parsing (serde_yaml)
/// - Syntax Highlighting (syntect)
/// - Post processing (lol_html)
/// - Reading time estimation
///
/// # Warning
///
/// HTML is not sanitized by this component
pub struct Markdown<'a>(pub &'a str);

impl Markdown<'_> {
    /// Parses the front matter of the markdown content.
    ///
    /// # Returns
    ///
    /// This function returns Option<serde_yaml::Result<T>> where T is provided by the caller.
    ///
    /// `Some(Ok(T))` is returned if the front matter exists and is successfully parsed.
    /// `Some(Err(_))` is returned if the front matter exists but could not be parsed.
    /// `None` is returned if the front matter does not exist.
    ///
    /// # Front Matter
    ///
    /// Front matter is metadata about the markdown file that is not rendered in the final HTML.
    /// It is typically used to store information like tags, categories, or other metadata.
    ///
    /// The front matter is defined at the top of the markdown file between two "---" delimiters and
    /// is written in YAML format.
    ///
    /// # Example
    ///
    /// ```
    /// # use maud::html;
    /// use mahoney_best::components::Markdown;
    ///
    /// let markdown = Markdown(r#"
    /// ---
    /// tags: ["very cool"]
    /// ---
    ///
    /// # Hello World
    /// "#.to_string());
    ///
    /// #[derive(Debug, serde::Deserialize)]
    /// struct FrontMatter {
    ///    tags: Vec<String>,
    /// }
    ///
    /// let front_matter: Option<serde_yaml::Result<FrontMatter>> = markdown.front_matter();
    /// println!("{:?}", front_matter);
    /// assert_eq!(front_matter.unwrap().unwrap().tags, vec!["very cool"]);
    /// ```
    pub fn front_matter<T>(&self) -> Option<serde_yaml::Result<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut lines = self.0.lines().peekable();
        let mut front_matter = String::new();

        while let Some(line) = lines.peek() {
            if line.trim().is_empty() {
                lines.next();
            } else {
                break;
            }
        }

        if lines.next()? != FRONT_MATTER_DELIMITER {
            return None;
        }

        for line in lines {
            if line == FRONT_MATTER_DELIMITER {
                break;
            }
            front_matter.push_str(line);
            front_matter.push('\n');
        }

        Some(serde_yaml::from_str(&front_matter))
    }

    /// Estimates the reading time of the markdown content.
    ///
    /// The reading time is calculated based on the average reading speed of 200 words per minute.
    pub fn reading_time(&self) -> Duration {
        let words = self.0.split_whitespace().count();
        Duration::from_secs_f32(words as f32 / 200.0)
    }
}

/// Returns my chosen comrak options
fn get_comrak_options() -> Options {
    let mut options = Options::default();

    // Extensions
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_ids = None;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.front_matter_delimiter = Some(FRONT_MATTER_DELIMITER.to_string());
    options.extension.multiline_block_quotes = true;
    options.extension.math_dollars = true;
    options.extension.math_code = true;
    options.extension.shortcodes = true;

    // Render
    options.render.unsafe_ = true;

    options
}

fn perform_syntax_highlighting<'a>(ast: &'a Node<'a, RefCell<Ast>>) {
    let iter = NodeIter::new(ast);
    for mut node in iter
        .map(|node| node.data.borrow_mut())
        .filter(|node| matches!(node.value, NodeValue::CodeBlock(_)))
    {
        // Extract the code block
        let code_block = match &mut node.value {
            NodeValue::CodeBlock(code_block) => code_block,
            _ => unreachable!(),
        };

        // Find supported syntax
        let syntax = match PS.find_syntax_by_token(&code_block.info) {
            Some(syntax) => syntax,
            None => {
                log::warn!(
                    "Language {:?} is not supported in code block",
                    code_block.info
                );
                continue;
            }
        };

        // Preform syntax highlighting with the THEME
        let highlighted = match syntect::html::highlighted_html_for_string(
            &code_block.literal,
            &PS,
            syntax,
            &TS.themes[THEME],
        ) {
            Ok(html) => html,
            Err(e) => {
                log::error!("Error highlighting code block: {}", e);
                continue;
            }
        };

        // Add the `code` class to the new `pre` element
        let post_processed = lol_html::rewrite_str(
            &highlighted,
            Settings {
                element_content_handlers: vec![element!("pre", |el| {
                    el.set_attribute("class", "code")?;
                    Ok(())
                })],
                ..Settings::default()
            },
        );

        let html = match post_processed {
            Ok(html) => html,
            Err(e) => {
                log::error!("Error adding code class: {}", e);
                continue;
            }
        };

        // Finally we can edit the AST, replacing the code block with a raw HTML block
        node.value = NodeValue::HtmlBlock(NodeHtmlBlock {
            block_type: 0,
            literal: html,
        });
    }
}

// Adds `target="_blank"` and `rel="noopener"` to all links that lead to external websites
fn add_target_blank(html: &str) -> String {
    lol_html::rewrite_str(
        html,
        Settings {
            element_content_handlers: vec![element!("a", |el| {
                if let Some(href) = el.get_attribute("href") {
                    if href.starts_with("http") {
                        el.set_attribute("target", "_blank")?;
                        el.set_attribute("rel", "noopener")?;
                    }
                }
                Ok(())
            })],
            ..Settings::default()
        },
    )
    .expect("a tag rewriting failed")
}

impl Render for Markdown<'_> {
    fn render(&self) -> maud::Markup {
        let arena = Arena::new();
        let ast = comrak::parse_document(&arena, self.0, &get_comrak_options());

        // Preform transformations on the AST
        perform_syntax_highlighting(ast);

        // Render the AST to HTML
        let mut html = vec![];
        comrak::format_html(ast, &get_comrak_options(), &mut html).unwrap();
        let html = String::from_utf8_lossy(&html);

        // Post processing
        let html = add_target_blank(&html);

        html! {
            (maud::PreEscaped(&html))
        }
    }
}

struct NodeIter<'a> {
    stack: Vec<&'a AstNode<'a>>,
}

impl<'a> NodeIter<'a> {
    fn new(node: &'a AstNode<'a>) -> Self {
        let stack = vec![node];
        NodeIter { stack }
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a AstNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        for c in node.children() {
            self.stack.push(c);
        }
        Some(node)
    }
}

# [Markdown: Syntax](https://github.com/mxstbr/markdown-test-file/tree/master)

* [Overview](#overview)
  * [Philosophy](#philosophy)
  * [Inline HTML](#html)
  * [Automatic Escaping for Special Characters](#autoescape)
* [Block Elements](#block)
  * [Paragraphs and Line Breaks](#p)
  * [Headers](#header)
  * [Blockquotes](#blockquote)
  * [Lists](#list)
  * [Code Blocks](#precode)
  * [Horizontal Rules](#hr)
* [Span Elements](#span)
  * [Links](#link)
  * [Emphasis](#em)
  * [Code](#code)
  * [Images](#img)
* [Miscellaneous](#misc)
  * [Backslash Escapes](#backslash)
  * [Automatic Links](#autolink)

**Note:** This document is itself written using Markdown; you
can [see the source for it by adding '.text' to the URL](/projects/markdown/syntax.text).

----

## Overview

### Philosophy

Markdown is intended to be as easy-to-read and easy-to-write as is feasible.

Readability, however, is emphasized above all else. A Markdown-formatted
document should be publishable as-is, as plain text, without looking
like it's been marked up with tags or formatting instructions. While
Markdown's syntax has been influenced by several existing text-to-HTML
filters -- including [Setext](http://docutils.sourceforge.net/mirror/setext.html), [atx](http://www.aaronsw.com/2002/atx/), [Textile](http://textism.com/tools/textile/), [reStructuredText](http://docutils.sourceforge.net/rst.html),
[Grutatext](http://www.triptico.com/software/grutatxt.html), and [EtText](http://ettext.taint.org/doc/) -- the single biggest source of
inspiration for Markdown's syntax is the format of plain text email.

## Block Elements

### Paragraphs and Line Breaks

A paragraph is simply one or more consecutive lines of text, separated
by one or more blank lines. (A blank line is any line that looks like a
blank line -- a line containing nothing but spaces or tabs is considered
blank.) Normal paragraphs should not be indented with spaces or tabs.

The implication of the "one or more consecutive lines of text" rule is
that Markdown supports "hard-wrapped" text paragraphs. This differs
significantly from most other text-to-HTML formatters (including Movable
Type's "Convert Line Breaks" option) which translate every line break
character in a paragraph into a `<br />` tag.

When you *do* want to insert a `<br />` break tag using Markdown, you
end a line with two or more spaces, then type return.

### Headers

Markdown supports two styles of headers, [Setext] [1] and [atx] [2].

Optionally, you may "close" atx-style headers. This is purely
cosmetic -- you can use this if you think it looks better. The
closing hashes don't even need to match the number of hashes
used to open the header. (The number of opening hashes
determines the header level.)

### Blockquotes

Markdown uses email-style `>` characters for blockquoting. If you're
familiar with quoting passages of text in an email message, then you
know how to create a blockquote in Markdown. It looks best if you hard
wrap the text and put a `>` before every line:

> This is a blockquote with two paragraphs. Lorem ipsum dolor sit amet,
> consectetuer adipiscing elit. Aliquam hendrerit mi posuere lectus.
> Vestibulum enim wisi, viverra nec, fringilla in, laoreet vitae, risus.
>
> Donec sit amet nisl. Aliquam semper ipsum sit amet velit. Suspendisse
> id sem consectetuer libero luctus adipiscing.

Markdown allows you to be lazy and only put the `>` before the first
line of a hard-wrapped paragraph:

> This is a blockquote with two paragraphs. Lorem ipsum dolor sit amet,
consectetuer adipiscing elit. Aliquam hendrerit mi posuere lectus.
Vestibulum enim wisi, viverra nec, fringilla in, laoreet vitae, risus.

> Donec sit amet nisl. Aliquam semper ipsum sit amet velit. Suspendisse
id sem consectetuer libero luctus adipiscing.

Blockquotes can be nested (i.e. a blockquote-in-a-blockquote) by
adding additional levels of `>`:

> This is the first level of quoting.
>
> > This is nested blockquote.
>
> Back to the first level.

Blockquotes can contain other Markdown elements, including headers, lists,
and code blocks:

> ## This is a header
>
> 1. This is the first list item.
> 2. This is the second list item.
>
> Here's some example code:
>
>     return shell_exec("echo $input | $markdown_script");

Any decent text editor should make email-style quoting easy. For
example, with BBEdit, you can make a selection and choose Increase
Quote Level from the Text menu.

### Lists

Markdown supports ordered (numbered) and unordered (bulleted) lists.

Unordered lists use asterisks, pluses, and hyphens -- interchangably
-- as list markers:

* Red
* Green
* Blue

is equivalent to:

* Red
* Green
* Blue

and:

* Red
* Green
* Blue

Ordered lists use numbers followed by periods:

1. Bird
2. McHale
3. Parish

It's important to note that the actual numbers you use to mark the
list have no effect on the HTML output Markdown produces. The HTML
Markdown produces from the above list is:

If you instead wrote the list in Markdown like this:

1. Bird
1. McHale
1. Parish

or even:

3. Bird
1. McHale
8. Parish

you'd get the exact same HTML output. The point is, if you want to,
you can use ordinal numbers in your ordered Markdown lists, so that
the numbers in your source match the numbers in your published HTML.
But if you want to be lazy, you don't have to.

To make lists look nice, you can wrap items with hanging indents:

* Lorem ipsum dolor sit amet, consectetuer adipiscing elit.
    Aliquam hendrerit mi posuere lectus. Vestibulum enim wisi,
    viverra nec, fringilla in, laoreet vitae, risus.
* Donec sit amet nisl. Aliquam semper ipsum sit amet velit.
    Suspendisse id sem consectetuer libero luctus adipiscing.

But if you want to be lazy, you don't have to:

* Lorem ipsum dolor sit amet, consectetuer adipiscing elit.
Aliquam hendrerit mi posuere lectus. Vestibulum enim wisi,
viverra nec, fringilla in, laoreet vitae, risus.
* Donec sit amet nisl. Aliquam semper ipsum sit amet velit.
Suspendisse id sem consectetuer libero luctus adipiscing.

List items may consist of multiple paragraphs. Each subsequent
paragraph in a list item must be indented by either 4 spaces
or one tab:

1. This is a list item with two paragraphs. Lorem ipsum dolor
    sit amet, consectetuer adipiscing elit. Aliquam hendrerit
    mi posuere lectus.

    Vestibulum enim wisi, viverra nec, fringilla in, laoreet
    vitae, risus. Donec sit amet nisl. Aliquam semper ipsum
    sit amet velit.

2. Suspendisse id sem consectetuer libero luctus adipiscing.

It looks nice if you indent every line of the subsequent
paragraphs, but here again, Markdown will allow you to be
lazy:

* This is a list item with two paragraphs.

    This is the second paragraph in the list item. You're
only required to indent the first line. Lorem ipsum dolor
sit amet, consectetuer adipiscing elit.

* Another item in the same list.

To put a blockquote within a list item, the blockquote's `>`
delimiters need to be indented:

* A list item with a blockquote:

    > This is a blockquote
    > inside a list item.

To put a code block within a list item, the code block needs
to be indented *twice* -- 8 spaces or two tabs:

* A list item with a code block:

        <code goes here>

### Code Blocks

Pre-formatted code blocks are used for writing about programming or
markup source code. Rather than forming normal paragraphs, the lines
of a code block are interpreted literally. Markdown wraps a code block
in both `<pre>` and `<code>` tags.

To produce a code block in Markdown, simply indent every line of the
block by at least 4 spaces or 1 tab.

This is a normal paragraph:

    This is a code block.

Here is an example of AppleScript:

    tell application "Foo"
        beep
    end tell

A code block continues until it reaches a line that is not indented
(or the end of the article).

Within a code block, ampersands (`&`) and angle brackets (`<` and `>`)
are automatically converted into HTML entities. This makes it very
easy to include example HTML source code using Markdown -- just paste
it and indent it, and Markdown will handle the hassle of encoding the
ampersands and angle brackets. For example, this:

    <div class="footer">
        &copy; 2004 Foo Corporation
    </div>

Regular Markdown syntax is not processed within code blocks. E.g.,
asterisks are just literal asterisks within a code block. This means
it's also easy to use Markdown to write about Markdown's own syntax.

```
tell application "Foo"
    beep
end tell
```

## Span Elements

### Links

Markdown supports two style of links: *inline* and *reference*.

In both styles, the link text is delimited by [square brackets].

To create an inline link, use a set of regular parentheses immediately
after the link text's closing square bracket. Inside the parentheses,
put the URL where you want the link to point, along with an *optional*
title for the link, surrounded in quotes. For example:

This is [an example](http://example.com/) inline link.

[This link](http://example.net/) has no title attribute.

### Emphasis

Markdown treats asterisks (`*`) and underscores (`_`) as indicators of
emphasis. Text wrapped with one `*` or `_` will be wrapped with an
HTML `<em>` tag; double `*`'s or `_`'s will be wrapped with an HTML
`<strong>` tag. E.g., this input:

*single asterisks*

*single underscores*

**double asterisks**

**double underscores**

### Code

To indicate a span of code, wrap it with backtick quotes (`` ` ``).
Unlike a pre-formatted code block, a code span indicates code within a
normal paragraph. For example:

Use the `printf()` function.

```rust
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
```

use maud::{html, Render, DOCTYPE};

use crate::components;

use super::{navbar::SiteNav, Markdown};

#[derive(Debug, serde::Deserialize)]
pub struct MarkdownFrontMatter {
    #[serde(rename = "page-title")]
    pub title: Option<String>,
    #[serde(default)]
    pub hidden: bool,
    pub order: Option<i32>,
}

impl MarkdownFrontMatter {
    /// Creates a new "front matter" with a title.
    ///
    /// hidden is assumed `false` and order is assumed `None`.
    pub fn with_title(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            hidden: false,
            order: None,
        }
    }
}

/// A portfolio/blog post page.
pub struct Page<'a> {
    pub uri: &'a str,

    pub title: &'a str,
    pub content: Markdown<'a>,

    pub nav: &'a SiteNav,
}

impl Render for Page<'_> {
    fn render(&self) -> maud::Markup {
        html! {
            (DOCTYPE)
            (components::header(self.title))
            (self.nav.render(self.uri))
            main {
                (&self.content)
            }
        }
    }
}

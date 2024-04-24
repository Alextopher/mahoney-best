use maud::{html, Render, DOCTYPE};

use crate::components;

use super::Markdown;

/// A portfolio/blog post page.
pub struct Page<'a> {
    pub uri: &'a str,
    pub title: &'a str,
    pub content: Markdown<'a>,
}

impl Render for Page<'_> {
    fn render(&self) -> maud::Markup {
        html! {
            (DOCTYPE)
            (components::header(self.title))
            (components::navbar(self.uri))
            main {
                (&self.content)
            }
        }
    }
}

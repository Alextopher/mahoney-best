use maud::{html, Render};

use crate::components;

use super::Markdown;

pub struct Page<'a> {
    pub uri: &'a str,
    pub title: &'a str,
    pub content: Markdown<'a>,
}

impl Render for Page<'_> {
    fn render(&self) -> maud::Markup {
        html! {
            (components::header(self.title))
            (components::navbar(self.uri))
            main style {
                section {
                    article {
                        (&self.content)
                    }
                }
            }
        }
    }
}

use inflector::Inflector;
use markdown::Markdown;
use maud::{html, Markup, Render, DOCTYPE};

use crate::{MarkdownFrontMatter, SiteNav};

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
            (crate::header(self.title))
            (self.nav.render(self.uri))
            main {
                (&self.content)
            }
        }
    }
}

/// Required markdown page metadata
#[allow(unused)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct PageMetadata {
    pub path: String,
    pub title: String,
    pub hidden: bool,
    pub order: i32,
}

pub fn parse_page<'a>(path: &str, content: &'a str) -> (PageMetadata, Markdown<'a>) {
    let md = Markdown::new(content);
    let front_matter = md
        .front_matter::<MarkdownFrontMatter>()
        .transpose()
        .unwrap();

    let order = front_matter
        .as_ref()
        .and_then(|f| f.order)
        .unwrap_or(i32::MIN);

    let file_name = path.split('/').last().unwrap();

    let title = front_matter
        .as_ref()
        .and_then(|f| f.title.to_owned())
        .unwrap_or_else(|| file_name.to_title_case());

    let metadata = PageMetadata {
        path: path.to_string(),
        title,
        hidden: front_matter.as_ref().map(|f| f.hidden).unwrap_or(false),
        order,
    };

    (metadata, md)
}

/// Renders a page using the given metadata and markdown content
pub fn render_page<'a>(
    nav: &SiteNav,
    path: &'a str,
    metadata: &PageMetadata,
    md: &Markdown<'a>,
) -> Markup {
    html! {
        (DOCTYPE)
        (crate::header(&metadata.title))
        main {
            (nav.render(path))
            (md)
        }
    }
}

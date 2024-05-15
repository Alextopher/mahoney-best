mod blog;
mod markdown;
mod navbar;

use maud::{html, Markup};

pub use blog::{MarkdownFrontMatter, Page};
pub use markdown::Markdown;
pub use navbar::SiteNav;

pub fn header(title: &str) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/s/water.css";
            link rel="stylesheet" href="/s/style.css";

            title { (title) }
        }
    }
}

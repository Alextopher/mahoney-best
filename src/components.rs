mod blog;
mod markdown;
mod navbar;
mod robots;

pub use self::navbar::SiteNav;
pub use blog::{MarkdownFrontMatter, Page};
pub use markdown::Markdown;
pub use robots::robots;

use maud::{html, Markup};

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

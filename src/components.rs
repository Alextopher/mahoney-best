mod blog;
mod markdown;
mod robots;

pub use blog::Page;
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

pub fn navbar(current: &str) -> Markup {
    let sites = &[("/m/home.md", "Home"), ("/m/robotopia.md", "Robotics")];

    html! {
        nav {
            @for (path, name) in sites {
                a href=(path) style=(if *path == current { "text-decoration: underline" } else { "" }) { (name) }
            }
        }
        hr;
    }
}

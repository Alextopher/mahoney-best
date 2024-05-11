mod blog;
mod markdown;
mod robots;

pub use blog::{MarkdownFrontMatter, NavBar, Page};
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

pub fn navbar(nav: &NavBar, current: &str) -> Markup {
    // Figure out which part of the site we're on. It's either for example
    // /m/home.md -> ""
    // /m/robotics/home.md -> "robotics"
    let directory = if current.split('/').count() == 4 {
        current.split('/').nth(2).unwrap()
    } else {
        ""
    };

    let mut sites = match nav.0.get(directory).cloned() {
        Some(s) => s,
        None => {
            log::error!("No navigation found for directory: {}", directory);
            return html! {
                nav {
                    a href="/m/home.md" style="text-decoration: underline" { "FRC Programming" }
                }
                hr;
            };
        }
    };

    // The "home.md" file is special and is moved into it's own "breadcrumb" list. Remove it from the
    // sites list.
    let mut breadcrumbs = vec![];

    let index = sites.iter().position(|(path, _)| path.ends_with("home.md"));
    if let Some(index) = index {
        breadcrumbs.push(sites.remove(index));
    }

    // If we're on the top level pages then we add titles for all the subdirectories.
    // Otherwise, we link back to the home page.
    if directory.is_empty() {
        sites.push((
            "/m/robotics/home.md".to_string(),
            "FRC Programming".to_string(),
        ));
    } else {
        breadcrumbs.push(("/m/home.md".to_string(), "Home".to_string()));
    }

    html! {
        nav {
            @for (path, name) in breadcrumbs.iter().rev() {
                a href=(path) style=(if *path == *current { "text-decoration: underline" } else { "" }) { (name) }
            }

            div style="flex-grow: 1;" {};

            @for (path, name) in sites {
                a href=(path) style=(if *path == *current { "text-decoration: underline" } else { "" }) { (name) }
            }
        }
        hr;
    }
}

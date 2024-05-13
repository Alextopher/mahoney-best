use std::collections::HashMap;

use include_dir::{Dir, File};
use inflector::Inflector;
use maud::{html, Render, DOCTYPE};

use crate::components;

use super::Markdown;

#[derive(serde::Deserialize)]
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

/// The content directory is at most 2 levels deep.
#[derive(Debug)]
pub struct NavBar(pub HashMap<&'static str, Vec<(String, String)>>);

fn get_info(f: &File) -> Option<(String, Option<i32>)> {
    let content = std::str::from_utf8(f.contents()).unwrap();
    let markdown = Markdown(content);
    let front_matter = markdown
        .front_matter::<MarkdownFrontMatter>()
        .transpose()
        .expect("Failed to parse front matter");

    // If the front matter is hidden, return None.
    if let Some(front_matter) = &front_matter {
        if front_matter.hidden {
            return None;
        }
    }

    let order = front_matter.as_ref().and_then(|f| f.order);

    let title = front_matter.and_then(|f| f.title).unwrap_or_else(|| {
        f.path()
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_title_case()
    });

    Some((title, order))
}

impl NavBar {
    pub fn new(dir: &'static Dir) -> NavBar {
        let mut tree = HashMap::new();

        let mut dirs: Vec<&Dir> = dir.dirs().collect();
        dirs.push(dir);

        for dir in dirs {
            let key = dir.path().to_str().unwrap();

            let paths = dir
                .files()
                .map(|f| format!("/m/{}", f.path().to_str().unwrap()));

            let mut info: Vec<(String, Option<i32>, String)> = dir
                .files()
                .map(|f| get_info(f))
                .zip(paths)
                .filter_map(|(info, p)| info.map(|(title, order)| (title, order, p)))
                .collect::<Vec<_>>();

            info.sort_by_key(|(_, order, _)| order.unwrap_or(i32::MAX));

            let results = info
                .into_iter()
                .map(|(title, _, path)| (path, title))
                .collect();

            println!("{:?}", results);

            tree.insert(key, results);
        }

        NavBar(tree)
    }
}

/// A portfolio/blog post page.
pub struct Page<'a> {
    pub uri: &'a str,

    pub title: &'a str,
    pub content: Markdown<'a>,

    pub nav: &'a NavBar,
}

impl Render for Page<'_> {
    fn render(&self) -> maud::Markup {
        html! {
            (DOCTYPE)
            (components::header(self.title))
            (components::navbar(&self.nav, self.uri))
            main {
                (&self.content)
            }
        }
    }
}

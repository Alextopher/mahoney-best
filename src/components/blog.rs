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
}

/// The content directory is at most 2 levels deep.
#[derive(Debug)]
pub struct NavBar(pub HashMap<&'static str, Vec<(String, String)>>);

fn get_title(dir: &File) -> Option<String> {
    let content = std::str::from_utf8(dir.contents()).unwrap();
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

    Some(front_matter.and_then(|m| m.title).unwrap_or_else(|| {
        dir.path()
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_title_case()
    }))
}

impl NavBar {
    pub fn new(dir: &'static Dir) -> NavBar {
        let mut tree = HashMap::new();

        let mut dirs: Vec<&Dir> = dir.dirs().collect();
        dirs.push(dir);

        for dir in dirs {
            let key = dir.path().to_str().unwrap();

            let paths: Vec<String> = dir
                .files()
                .map(|f| format!("/m/{}", f.path().to_str().unwrap()))
                .collect();
            let titles: Vec<Option<String>> = dir.files().map(|f| get_title(f)).collect();

            // Zip the paths and titles together, and skip the hidden files.
            let results = paths
                .into_iter()
                .zip(titles)
                .filter_map(|(p, t)| t.map(|t| (p, t)))
                .collect();

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

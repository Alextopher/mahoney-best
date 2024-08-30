//! The blog is composed of a series of markdown files rendered into HTML.
//!
//! The pipe line for building the blog is as follows:
//! - Markdown files are read from the `content` directory.
//! - Metadata is extracted from front matter and defaults are applied.
//! - A navigation structure is built from the markdown files.
//! - The markdown content is rendered into HTML and cached.

use std::collections::HashMap;

use maud::Markup;

use crate::{
    page::{parse_page, render_page},
    SiteNav,
};

#[derive(Debug, serde::Deserialize)]
pub struct MarkdownFrontMatter {
    #[serde(rename = "page-title")]
    pub title: Option<String>,
    #[serde(default)]
    pub hidden: bool,
    pub order: Option<i32>,
}

/// The blog is a map from location to rendered HTML content
#[derive(Debug)]
pub struct Blog {
    rendered: HashMap<String, Markup>,
}

impl Blog {
    /// Create a new blog from a runtime file system path
    //    pub fn from_fs_path(path: impl AsRef<Path>) -> Self {
    //        let mut pages: HashMap<String, String> = HashMap::new();
    //        let base_path = path.as_ref();
    //
    //        let mut stack = vec![base_path.to_path_buf()];
    //        while let Some(path) = stack.pop() {
    //            for entry in std::fs::read_dir(&path).unwrap() {
    //                let path = entry.unwrap().path();
    //
    //                if path.is_dir() {
    //                    stack.push(path);
    //                } else if path.is_file() {
    //                    let relative_path: String = path
    //                        .strip_prefix(base_path)
    //                        .unwrap()
    //                        .to_string_lossy()
    //                        .to_string();
    //
    //                    pages.insert(relative_path, std::fs::read_to_string(&path).unwrap());
    //                }
    //            }
    //        }
    //
    //        Self::new(pages)
    //    }

    /// Create a new blog from an [`include_dir::Dir`]
    pub fn from_include_dir(dir: &include_dir::Dir) -> Self {
        let navbar = SiteNav::new(dir);
        let mut pages: HashMap<String, String> = HashMap::new();

        let mut stack = vec![dir];
        while let Some(path) = stack.pop() {
            for entry in path.entries() {
                match entry {
                    include_dir::DirEntry::Dir(dir) => stack.push(dir),
                    include_dir::DirEntry::File(file) => {
                        let relative_path = file.path().to_string_lossy().to_string();
                        pages.insert(relative_path, file.contents_utf8().unwrap().to_string());
                    }
                }
            }
        }

        Self::new(pages, navbar)
    }

    /// Create a new blog from a map of un-rendered markdown content
    ///
    /// Takes as input a map from relative path to markdown content
    fn new(pages: HashMap<String, String>, nav: SiteNav) -> Self {
        let mut parsed = HashMap::new();

        for (path, content) in pages.iter() {
            let (metadata, md) = parse_page(path.as_str(), content);

            if !metadata.hidden {
                parsed.insert(path.clone(), (metadata, md));
            }
        }

        let mut rendered = HashMap::new();
        for (path, (metadata, md)) in parsed.iter() {
            rendered.insert(path.clone(), render_page(&nav, path, metadata, md));
        }

        Self { rendered }
    }

    /// Gets the rendered HTML for a given path
    pub fn get(&self, path: &str) -> Option<&Markup> {
        self.rendered.get(path)
    }
}

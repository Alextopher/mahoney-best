use inflector::Inflector;
use markdown::Markdown;
use maud::{html, Markup};
use std::{
    collections::{HashMap, VecDeque},
    path::Path,
};

use include_dir::{Dir, DirEntry, File};

use super::MarkdownFrontMatter;

/// Get nav info returns the title, order, and path of a file
fn get_nav_info(page: &File) -> Option<(String, i32, String)> {
    let path = format!("/m/{}", page.path().to_str().unwrap());

    let content = std::str::from_utf8(page.contents()).unwrap();
    let markdown = Markdown::new(content);
    let front_matter = markdown
        .front_matter::<MarkdownFrontMatter>()
        .transpose()
        .expect("Failed to parse front matter");

    // If this page is hidden, return None
    if front_matter.as_ref().map(|f| f.hidden).unwrap_or(false) {
        return None;
    }

    let order = front_matter
        .as_ref()
        .and_then(|f| f.order)
        .unwrap_or(i32::MIN);

    let title = front_matter
        .as_ref()
        .and_then(|f| f.title.to_owned())
        .unwrap_or_else(|| {
            page.path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_title_case()
        });

    Some((title, order, path))
}

#[derive(Debug, Clone)]
struct NavItem {
    title: String,
    uri: String,
    order: i32,
}

impl NavItem {
    fn new(title: String, uri: String, order: i32) -> Self {
        NavItem { title, uri, order }
    }

    fn from_file(file: &File) -> Option<Self> {
        get_nav_info(file).map(|(title, order, uri)| NavItem::new(title, uri, order))
    }

    fn from_dir(dir: &Dir) -> Option<Self> {
        // Find the file in this directory that ends with "home.md"
        let home = dir
            .files()
            .find(|f| f.path().file_name().unwrap().to_str().unwrap() == "home.md");

        Self::from_file(home?)
    }

    fn from_dir_entry(entry: &DirEntry) -> Option<Self> {
        match entry {
            DirEntry::File(f) => Self::from_file(f),
            DirEntry::Dir(d) => Self::from_dir(d),
        }
    }
}

impl PartialEq for NavItem {
    fn eq(&self, other: &Self) -> bool {
        self.uri == other.uri
    }
}

impl Eq for NavItem {}

impl PartialOrd for NavItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NavItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order
            .cmp(&other.order)
            .then_with(|| self.title.cmp(&other.title))
    }
}

/// The navigation bar for a single directory
#[derive(Debug)]
struct NavBar {
    breadcrumbs: Vec<NavItem>,
    children: Vec<NavItem>,
}

impl NavBar {
    fn new(breadcrumbs: &[NavItem], dir: &Dir) -> Option<Self> {
        let mut children = dir
            .entries()
            .iter()
            .filter_map(|entry| NavItem::from_dir_entry(entry))
            .collect::<Vec<_>>();

        // Remove this directories home page from the children list, moving it to the breadcrumbs
        let home = dir
            .files()
            .map(|f| f.path().to_str().unwrap())
            .find(|p| p.ends_with("home.md"))?;

        let home = format!("/m/{}", home);
        let home = children.iter().position(|c| c.uri == home)?;
        let home = children.swap_remove(home);

        let mut breadcrumbs = breadcrumbs.to_vec();
        breadcrumbs.push(home);

        // Sort the children by their order
        children.sort();

        Some(NavBar {
            children,
            breadcrumbs,
        })
    }
}

/// The navigation bar for the entire site
#[derive(Debug)]
pub struct SiteNav {
    /// Maps directory uri to the navigation bar for that directory
    tree: HashMap<String, NavBar>,
}

impl SiteNav {
    /// Creates the site's navigation tree from an [`include_dir::Dir`]
    pub fn new(dir: &Dir) -> Self {
        let mut tree = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back((vec![], dir));

        while let Some((breadcrumbs, dir)) = queue.pop_front() {
            // Build the navigation bar for this directory
            let nav = NavBar::new(&breadcrumbs, dir).expect("Failed to create navigation bar");

            // Add this directory to the breadcrumbs for its children
            let breadcrumbs = nav.breadcrumbs.clone();

            // Add the children to the queue
            for dir in dir.dirs() {
                queue.push_back((breadcrumbs.clone(), dir));
            }

            let mut path = format!("/m/{}", dir.path().to_str().unwrap());
            if !path.ends_with('/') {
                path.push('/');
            }

            tree.insert(path, nav);
        }

        Self { tree }
    }

    /// Renders the navigation bar from the perspective of the current page
    pub fn try_render(&self, current: &str) -> Option<Markup> {
        // Find the directory of the current page, unlike the old version we can't assume the
        // length of the path
        let path: &Path = current.as_ref();

        let dir = path
            .parent()
            .and_then(|p| p.to_str())
            .map(|p| format!("{}/", p));

        // Get the navigation bar for this directory
        let nav = self.tree.get(&dir?)?;

        // Render the navigation bar
        Some(html! {
            nav {
                @for item in &nav.breadcrumbs {
                    a href=(item.uri) style=(if *item.uri == *current { "text-decoration: underline" } else { "" }) { (item.title) }
                }

                div style="flex-grow: 1;" {};

                @for item in &nav.children {
                    a href=(item.uri) style=(if *item.uri == *current { "text-decoration: underline" } else { "" }) { (item.title) }
                }
            }
            hr;
        })
    }

    /// Renders the navigation bar from the perspective of the current page, or fallback to rendering the home page's version
    pub fn render(&self, current: &str) -> Markup {
        let p = format!("/m/{}", current);
        self.try_render(&p).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use include_dir::include_dir;
    const CONTENT: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../content");

    #[test]
    fn site_nav() {
        let nav = SiteNav::new(&CONTENT);
        println!("{:#?}", nav.tree);
    }
}

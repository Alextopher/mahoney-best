use std::path::PathBuf;

use actix_web::{
    dev::HttpServiceFactory,
    get,
    web::{self, Data},
    HttpRequest, Responder,
};
use include_dir::{include_dir, Dir};
use maud::Render;

use crate::components::{self, Markdown, MarkdownFrontMatter, SiteNav};

const CONTENT: Dir = include_dir!("content");

/// Markdown rendering service that functions as the foundation of the site
pub fn markdown_service() -> impl HttpServiceFactory {
    let nav = SiteNav::new(&CONTENT);
    web::scope("/m")
        .app_data(Data::new(nav))
        .service(markdown_handler)
}

#[get("/{filename:.*}")]
async fn markdown_handler(
    path: web::Path<PathBuf>,
    nav: web::Data<SiteNav>,
    req: HttpRequest,
) -> impl Responder {
    let path = path.into_inner();
    let file = match CONTENT.get_file(&path) {
        Some(f) => f,
        None => {
            return Err(actix_web::error::ErrorNotFound("File not found"));
        }
    };

    let content = match std::str::from_utf8(file.contents()) {
        Ok(s) => s,
        Err(e) => return Err(actix_web::error::ErrorInternalServerError(e.to_string())),
    };

    let markdown = Markdown(content);
    let front_matter = markdown
        .front_matter()
        .transpose()
        .map_err(|e| {
            log::error!("Error parsing front matter: {}", e);
            e
        })
        .ok()
        .flatten()
        .unwrap_or_else(|| {
            MarkdownFrontMatter::with_title(path.file_stem().unwrap().to_string_lossy())
        });

    if front_matter.hidden {
        return Err(actix_web::error::ErrorNotFound("File not found"));
    }

    let page = components::Page {
        title: &front_matter
            .title
            .unwrap_or_else(|| path.file_stem().unwrap().to_string_lossy().to_string()),
        content: markdown,
        uri: req.uri().path(),
        nav: &nav,
    };

    Ok(page.render())
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::markdown_service;

    // Check what happens if the path includes ".."
    #[actix_web::test]
    async fn test_path_traversal() {
        let app = test::init_service(App::new().service(markdown_service())).await;

        let req = test::TestRequest::get()
            .uri("/m/../../Cargo.toml")
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), 404);
    }
}

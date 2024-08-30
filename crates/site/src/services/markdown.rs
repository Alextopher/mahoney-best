use std::path::PathBuf;

use actix_web::{
    dev::HttpServiceFactory,
    get,
    web::{self, Data},
};
use blog::Blog;
use include_dir::{include_dir, Dir};
use maud::Markup;

const CONTENT: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../content");

/// Markdown rendering service that functions as the foundation of the site
pub fn markdown_service() -> impl HttpServiceFactory {
    let blog = Blog::from_include_dir(&CONTENT);

    web::scope("/m")
        .app_data(Data::new(blog))
        .service(markdown_handler)
}

#[get("/{filename:.*}")]
async fn markdown_handler(path: web::Path<PathBuf>, blog: web::Data<Blog>) -> Option<Markup> {
    blog.get(&path.to_string_lossy()).cloned()
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

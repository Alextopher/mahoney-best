use actix_web::{web, HttpResponse};
use include_dir::{include_dir, Dir};
use pulldown_cmark::Parser;
use tera::Context;

const CONTENT: Dir = include_dir!("content");

/// Markdown rendering service that functions as the foundation of the site
pub fn markdown_service() -> impl actix_web::dev::HttpServiceFactory {
    let tera = tera::Tera::new("templates/**/*").expect("Failed to compile templates");

    web::resource("/m/{filename}")
        .route(web::get().to(markdown_handler))
        .app_data(tera.clone())
}

async fn markdown_handler(req: actix_web::HttpRequest) -> actix_web::Result<HttpResponse> {
    let _span = tracing::info_span!("markdown_handler");

    let path = req.match_info().query("filename");
    let file = CONTENT
        .get_file(path)
        .and_then(|f| f.contents_utf8())
        .ok_or_else(|| {
            tracing::warn!("File not found: {}", path);
            actix_web::error::ErrorNotFound("File not found")
        })?;

    let mut html = String::new();
    let parser = Parser::new_ext(file, pulldown_cmark::Options::all());
    pulldown_cmark::html::push_html(&mut html, parser);

    let mut context = Context::new();
    context.insert("content", &html);

    let tera = req.app_data::<tera::Tera>().ok_or_else(|| {
        tracing::error!("Tera context missing");
        actix_web::error::ErrorInternalServerError("Tera context missing")
    })?;

    match tera.render("markdown.html", &context) {
        Ok(rendered) => Ok(HttpResponse::Ok().content_type("text/html").body(rendered)),
        Err(err) => {
            tracing::error!("Failed to render template: {}", err);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

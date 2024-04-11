// Markdown rendering service
//
// This service renders markdown files from the /content directory into HTML and
// then packages them into the `markdown.html` template.

use actix_web::{web, HttpResponse};
use include_dir::{include_dir, Dir};
use pulldown_cmark::Parser;
use tera::Context;

const CONTENT: Dir = include_dir!("content");

pub fn markdown_service() -> impl actix_web::dev::HttpServiceFactory {
    web::resource("/m/{filename}").route(web::get().to(markdown_handler))
}

async fn markdown_handler(req: actix_web::HttpRequest) -> Option<actix_web::HttpResponse> {
    let _span = tracing::info_span!("markdown_handler");

    let path = req.match_info().query("filename");
    let file = CONTENT.get_file(path)?.contents_utf8()?;

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, Parser::new(file));

    let mut context = Context::new();
    context.insert("content", &html);

    let tera = req.app_data::<tera::Tera>()?;
    let rendered = tera.render("markdown.html", &context);

    if let Err(e) = rendered {
        tracing::error!("Failed to render template: {}", e);
        return Some(HttpResponse::InternalServerError().finish());
    }

    Some(
        HttpResponse::Ok()
            .content_type("text/html")
            .body(rendered.unwrap()),
    )
}

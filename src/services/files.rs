#![allow(dead_code)]

use actix_web::{dev::HttpServiceFactory, HttpResponse};
use include_dir::{include_dir, Dir};

const FILES: Dir = include_dir!("static");

/// A static file service that serves files baked into the binary
pub fn baked_files() -> impl HttpServiceFactory {
    actix_web::web::resource("/s/{filename}").route(actix_web::web::get().to(baked_files_handler))
}

async fn baked_files_handler(req: actix_web::HttpRequest) -> actix_web::Result<HttpResponse> {
    let filename = req.match_info().query("filename");
    let file = FILES.get_file(filename).ok_or_else(|| {
        tracing::warn!("File not found: {}", filename);
        actix_web::error::ErrorNotFound("File not found")
    })?;

    Ok(HttpResponse::Ok()
        .content_type(mime_guess::from_path(filename).first_or_octet_stream())
        .body(file.contents()))
}

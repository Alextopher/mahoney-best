#![allow(dead_code)]

use actix_files::Files;
use actix_web::HttpResponse;
use include_dir::{include_dir, Dir};

/// A static file services that serves files from the static directory
pub fn live_files() -> Files {
    Files::new("/s", "static/")
}

const STATIC_FILES: Dir = include_dir!("static");

/// A static file service that serves files baked into the binary
pub fn baked_files() -> impl actix_web::dev::HttpServiceFactory {
    actix_web::web::resource("/s/{filename}").route(actix_web::web::get().to(baked_files_handler))
}

async fn baked_files_handler(req: actix_web::HttpRequest) -> actix_web::Result<HttpResponse> {
    let filename = req.match_info().query("filename");
    let file = STATIC_FILES.get_file(filename).ok_or_else(|| {
        tracing::warn!("File not found: {}", filename);
        actix_web::error::ErrorNotFound("File not found")
    })?;

    Ok(HttpResponse::Ok()
        .content_type(mime_guess::from_path(filename).first_or_octet_stream())
        .body(file.contents()))
}

#![allow(dead_code)]

use actix_files::Files;
use include_dir::{include_dir, Dir};

/// A static file services that serves files from the static directory
pub fn live_files() -> Files {
    Files::new("/s", "static/")
}

const STATIC_FILES: Dir = include_dir!("static");

/// A static file service that serves files baked into the binary
pub fn baked_files() -> impl actix_web::dev::HttpServiceFactory {
    actix_web::web::resource("/m/{filename}").route(actix_web::web::get().to(baked_files_handler))
}

async fn baked_files_handler(req: actix_web::HttpRequest) -> actix_web::HttpResponse {
    let filename = req.match_info().query("filename");
    if let Some(file) = STATIC_FILES.get_file(filename) {
        actix_web::HttpResponse::Ok()
            .content_type(mime_guess::from_path(filename).first_or_octet_stream())
            .body(file.contents())
    } else {
        actix_web::HttpResponse::NotFound().finish()
    }
}

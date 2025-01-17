use actix_web::{dev::HttpServiceFactory, get, HttpResponse};
use include_dir::{include_dir, Dir};
use log::warn;

const FILES: Dir = include_dir!("static");

/// A static file service that serves files baked into the binary
pub fn baked_files() -> impl HttpServiceFactory {
    actix_web::web::scope("/s").service(baked_files_handler)
}

#[get("/{filename:.*}")]
async fn baked_files_handler(req: actix_web::HttpRequest) -> actix_web::Result<HttpResponse> {
    let filename = req.match_info().query("filename");
    let file = FILES.get_file(filename).ok_or_else(|| {
        warn!("File not found: {}", filename);
        actix_web::error::ErrorNotFound("File not found")
    })?;

    Ok(HttpResponse::Ok()
        .content_type(mime_guess::from_path(filename).first_or_octet_stream())
        .body(file.contents()))
}

pub fn get_file(filename: &str) -> Option<&'static str> {
    FILES.get_file(filename).and_then(|f| f.contents_utf8())
}

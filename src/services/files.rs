use std::{fs, io::Write, path::Path};

use actix_files::Files;
use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{
    dev::HttpServiceFactory,
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    get, post,
    web::{self, redirect},
    Either, HttpResponse, Responder,
};
use futures_util::StreamExt;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use tempfile::NamedTempFile;

/// A file upload/download service that serves files from the filesystem
pub fn file_service() -> impl HttpServiceFactory {
    // Make upload directory if it doesn't exist
    if !Path::new("uploads").exists() {
        std::fs::create_dir("uploads").expect("Failed to create uploads directory");
    }

    // Static file service
    web::scope("/f")
        .service(redirect("", "/f/"))
        .service(index)
        .service(upload_file)
        .service(Files::new("/", "uploads"))
}

#[get("/")]
async fn index(user: Option<Identity>) -> Either<HttpResponse, Markup> {
    // 302 redirect to login if not authenticated
    if user.is_none() {
        return Either::Left(
            HttpResponse::Found()
                .append_header(("location", "/u/?redirect=/f"))
                .finish(),
        );
    }

    Either::Right(html! {
        (DOCTYPE)
        h1 { "File Service" }
        p { "Upload and download files here" }
        form method="post" enctype="multipart/form-data" {
            input type="file" name="file" multiple;
            input type="button" value="Upload" onclick="uploadFiles()";
        }

        (PreEscaped("<script src=\"/s/files.js\"></script>"))
    })
}

#[post("/")]
async fn upload_file(
    user: Option<Identity>,
    mut payload: Multipart,
) -> actix_web::Result<impl Responder> {
    // Requires authentication
    if user.is_none() {
        return Err(ErrorUnauthorized("Authentication required"));
    }

    // Steam multipart files to disk
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(ErrorNotFound)?;

        // Stream to temp file
        let mut tmp = NamedTempFile::new().map_err(ErrorInternalServerError)?;

        // Following a successful upload we move the file to the uploads directory
        let filename = field
            .content_disposition()
            .get_filename()
            .and_then(|f| Path::new(f).file_name())
            .map(|f| f.to_string_lossy().to_string())
            .ok_or(ErrorBadRequest("Filename missing"))?;

        // Write file to disk
        while let Some(chunk) = field.next().await {
            let chunk = chunk.map_err(ErrorBadRequest)?;
            tmp.write_all(&chunk).map_err(ErrorInternalServerError)?;
        }

        // Persist the temporary file by copying it into the `uploads` directory
        let mut persisted =
            fs::File::create_new(format!("uploads/{}", filename)).map_err(|e| match e.kind() {
                std::io::ErrorKind::AlreadyExists => ErrorBadRequest("File already exists"),
                _ => ErrorInternalServerError(e),
            })?;

        // Copy the temporary file to the persisted file
        std::io::copy(&mut tmp, &mut persisted).map_err(ErrorInternalServerError)?;
    }

    Ok(HttpResponse::Ok().finish())
}

use std::{
    collections::VecDeque,
    hash::Hasher,
    sync::{Arc, RwLock},
};

use crate::{components, services::baked::get_file};
use actix_multipart::form::{bytes::Bytes, text::Text, MultipartForm};
use actix_web::{
    dev::HttpServiceFactory,
    get,
    http::StatusCode,
    post,
    web::{self, redirect},
    Either, HttpResponse, Responder,
};
use autopixel::autopixel;
use twox_hash::XxHash64;

pub struct ArtCache {
    max_size: usize,
    cache: RwLock<VecDeque<(u64, (String, Arc<[u8]>))>>,
}

impl ArtCache {
    pub fn new(max_size: usize) -> Self {
        ArtCache {
            max_size,
            cache: RwLock::new(VecDeque::new()),
        }
    }

    /// Removes the oldest element from the cache if it is full
    fn insert(&self, hash: u64, p5js: String, image: Vec<u8>) {
        let mut guard = self.cache.write().unwrap();
        if guard.len() >= self.max_size {
            guard.pop_front();
        }
        guard.push_back((hash, (p5js, image.into())));
    }

    /// Returns the p5js and image if it is in the cache
    fn get(&self, hash: u64) -> Option<(String, Arc<[u8]>)> {
        self.cache
            .read()
            .unwrap()
            .iter()
            .find(|(h, _)| *h == hash)
            .map(|(_, v)| v)
            .cloned()
    }
}

pub fn autopixel_service() -> impl HttpServiceFactory {
    let sketches = web::scope("/sketches")
        .service(get_image)
        .service(get_javascript)
        .route("/{hash}", web::route().to(sketches_index));

    web::scope("/pixel")
        .service(redirect("", "/pixel/"))
        .service(index)
        .service(upload)
        .service(sketches)
}

async fn sketches_index(hash: web::Path<String>) -> impl Responder {
    let n = u64::from_str_radix(&hash.into_inner(), 16).unwrap();
    components::pixel_art_view(n)
}

#[get("/{hash}.png")]
async fn get_image(cache: web::Data<Arc<ArtCache>>, hash: web::Path<String>) -> impl Responder {
    let n = u64::from_str_radix(&hash.into_inner(), 16).unwrap();
    let (_, img) = match cache.get(n) {
        Some((p5js, img)) => (p5js, img),
        None => return HttpResponse::NotFound().finish(),
    };
    HttpResponse::Ok()
        .content_type("image/png")
        .body(img.to_vec())
}

#[get("/{hash}.js")]
async fn get_javascript(
    cache: web::Data<Arc<ArtCache>>,
    hash: web::Path<String>,
) -> impl Responder {
    let n = u64::from_str_radix(&hash.into_inner(), 16).unwrap();
    let (js, _) = match cache.get(n) {
        Some((p5js, img)) => (p5js, img),
        None => return HttpResponse::NotFound().finish(),
    };
    HttpResponse::Ok().content_type("text/javascript").body(js)
}

#[get("/")]
async fn index() -> HttpResponse {
    let file = get_file("autopixel/index.html").unwrap();
    HttpResponse::Ok().content_type("text/html").body(file)
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "15MB")]
    file: Bytes,
    colors: Text<usize>,
    size: Text<usize>,
}

// receives uploaded file, runs autopixel on it,
#[post("/upload")]
async fn upload(
    cache: web::Data<Arc<ArtCache>>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> impl Responder {
    // SHA2 hash of the file
    let mut hasher = XxHash64::default();
    hasher.write(&form.file.data);
    hasher.write_usize(form.colors.clone());
    hasher.write_usize(form.size.clone());
    let hash = hasher.finish();

    // Create a buf-reader from the file
    let reader = std::io::Cursor::new(form.file.data);

    // Run autopixel on the file
    let result = autopixel(reader, form.size.into_inner(), form.colors.into_inner());

    let (p5js, image) = match result {
        Ok((p5js, image)) => (p5js, image),
        // 400 Bad image
        Err(_) => return Either::Left(("Bad image", StatusCode::BAD_REQUEST)),
    };

    // Encode the image to png
    let image = autopixel::encode_png(&image);

    // Insert the result into the cache
    cache.insert(hash, p5js, image);
    Either::Right(redirect("/pixel", format!("/pixel/sketches/{:x}", hash)))
}

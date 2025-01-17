use actix_web::{
    middleware::{Compress, Logger},
    web::{redirect, Data},
    App, HttpServer,
};
use mahoney_best::{
    components,
    config::Config,
    services::{self, ArtCache},
};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cache = Arc::new(ArtCache::new(128));
    let config = Arc::new(Config::load());

    let app = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(cache.clone()))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .service(services::baked_files())
            .service(services::markdown_service())
            .service(services::user_service())
            .service(services::file_service())
            .service(services::autopixel_service())
            .service(
                actix_web::web::resource("/robots.txt")
                    .to(|| async { components::robots(&["t", "r", "u", "f", "w"]) }),
            )
            .service(redirect("/", "/m/home.md"))
    });

    app.bind("0.0.0.0:8080")?.run().await
}

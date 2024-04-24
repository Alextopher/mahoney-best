use mahoney_best::{components, config::Config, services};

use std::sync::Arc;

use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    middleware::{Compress, Logger},
    web::{redirect, Data},
    App,
};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::generate();
    let config = Arc::new(Config::load());

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app = actix_web::HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .wrap(Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(Compress::default())
            .service(services::baked_files())
            .service(services::markdown_service())
            .service(services::user_service())
            .service(services::file_service())
            .service(
                actix_web::web::resource("/robots.txt")
                    .to(|| async { components::robots(&["t", "r", "u", "f", "w"]) }),
            )
            .service(redirect("/", "/m/home.md"))
    });

    log::info!("Starting server on http://0.0.0.0:8080");
    app.bind("0.0.0.0:8080")?.run().await
}

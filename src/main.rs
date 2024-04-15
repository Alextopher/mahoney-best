mod services;

use actix_web::{middleware::Logger, App};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app = actix_web::HttpServer::new(move || {
        App::new()
            .service(services::baked_files())
            .service(services::markdown_service())
            .service(
                // redirect '/' to '/m/home.md'
                actix_web::web::resource("/").to(|_req: actix_web::HttpRequest| async move {
                    actix_web::HttpResponse::MovedPermanently()
                        .append_header((actix_web::http::header::LOCATION, "/m/home.md"))
                        .finish()
                }),
            )
            .wrap(Logger::default())
    });

    log::info!("Starting server on http://127.0.0.1:8080");
    app.bind("127.0.0.1:8080")?.run().await
}

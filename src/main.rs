mod services;

use actix_web::{middleware::Logger, App};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let tera = tera::Tera::new("templates/**/*").expect("Failed to compile templates");
    // assert that markdown.html template exists
    tera.get_template("markdown.html")
        .expect("markdown.html template not found");

    let app = actix_web::HttpServer::new(move || {
        App::new()
            .app_data(tera.clone())
            .service(services::live_files())
            .service(services::markdown_service())
            .wrap(Logger::default())
    });

    log::info!("Starting server on http://127.0.0.1:8080");
    app.bind("127.0.0.1:8080")?.run().await
}

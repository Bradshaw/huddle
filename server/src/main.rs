mod config;

mod models;

mod errors;

mod db;

mod handlers;

mod schema;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

use crate::{config::Config, handlers::*};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        Env::default()
            .default_filter_or("debug")
            .default_write_style_or("auto"),
    );

    let config = Config::from_env().unwrap();

    let server = HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .wrap(Logger::default())
            .service(
                web::resource("/counter")
                    .route(web::post().to(increment_count))
                    .route(web::get().to(get_count)),
            )
    })
    .bind(config.server_addr.clone())?
    .run();

    println!("HTTP server listening at {}", config.server_addr);

    server.await
}

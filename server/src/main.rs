use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use serde::Serialize;
use std::sync::Mutex;
struct AppStateWithCounter {
    count: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[derive(Serialize)]
struct AppStatistics {
    status: AppStatus
}

#[derive(Serialize)]
enum AppStatus {
    Okay,
}

#[derive(Serialize)]
struct SerializableAppState {
    count: i32,
}

async fn index() -> String {
    serde_json::to_string(&AppStatistics{status: AppStatus::Okay }).unwrap()
}

async fn bump(data: web::Data<AppStateWithCounter>) -> String {
    let mut count = data.count.lock().unwrap(); // <- get counter's MutexGuard
    *count += 1; // <- access counter inside MutexGuard

    serde_json::to_string(&SerializableAppState {count: *count}).unwrap()
}

async fn get_count(data: web::Data<AppStateWithCounter>) -> String {
    let count = data.count.lock().unwrap(); // <- get counter's MutexGuard
    serde_json::to_string(&SerializableAppState { count: *count }).unwrap()
}

fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0:3001".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        Env::default()
        .default_filter_or("debug")
        .default_write_style_or("auto")
    );

    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        count: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .wrap(Logger::default())
            .app_data(counter.clone())
            .route("/", web::get().to(index))
            .route("/get-count", web::get().to(get_count))
            .route("/bump", web::get().to(bump))
    })
    .bind(&address())?
    .run()
    .await
}

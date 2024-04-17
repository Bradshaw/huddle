use actix_web::{web, App, HttpServer};
use serde::Serialize;
use std::sync::Mutex;
struct AppStateWithCounter {
    count: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[derive(Serialize)]
struct SerializableAppState {
    count: i32,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        count: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .route("/get-count", web::get().to(get_count))
            .route("/bump", web::get().to(bump))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

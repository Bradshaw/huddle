use actix_web::{Error, HttpResponse};
use crate::db;

pub async fn get_count() -> Result<HttpResponse, Error> {
        let counter = db::get_count().await?;

    Ok(HttpResponse::Ok().json(counter))
}

pub async fn increment_count() -> Result<HttpResponse, Error> {
    let counter = db::increment_count().await?;

    Ok(HttpResponse::Ok().json(counter))
}
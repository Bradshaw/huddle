use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{db, errors::HuddleError};

pub async fn initialize_db(db_pool: &Pool) -> Result<(), Error> {
    let client: Client = db_pool.get().await.map_err(HuddleError::PoolError)?;

    Ok(db::initialize_db(&client).await?)
}

pub async fn get_count(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(HuddleError::PoolError)?;

    let counter = db::get_count(&client).await?;

    Ok(HttpResponse::Ok().json(counter))
}

pub async fn increment_count(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(HuddleError::PoolError)?;

    let counter = db::increment_count(&client).await?;

    Ok(HttpResponse::Ok().json(counter))
}
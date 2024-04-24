use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::HuddleError, models::Counter};

pub async fn initialize_db(client: &Client) -> Result<(), HuddleError> {
    Ok(client
        .batch_execute(include_str!("../sql/schema.sql"))
        .await?)
}


pub async fn get_count(client: &Client) -> Result<Counter, HuddleError> {
    let stmt = include_str!("../sql/get_count.sql");
    let stmt = stmt.replace("$table_fields", &Counter::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    Ok(client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Counter::from_row_ref(row).unwrap())
        .next()
        .unwrap())
}

pub async fn increment_count(client: &Client) -> Result<Counter, HuddleError> {
    let stmt = include_str!("../sql/increment_count.sql");
    let stmt = stmt.replace("$table_fields", &Counter::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    Ok(client
        .query(&stmt, &[],)
        .await?
        .iter()
        .map(|row| Counter::from_row_ref(row).unwrap())
        .next()
        .unwrap())
}
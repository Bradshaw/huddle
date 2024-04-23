use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::MyError, models::{Counter, User}};

pub async fn initialize_db(client: &Client) -> Result<(), MyError> {
    Ok(client
        .batch_execute(include_str!("../sql/schema.sql"))
        .await?)
}


pub async fn get_count(client: &Client) -> Result<Counter, MyError> {
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

pub async fn get_users(client: &Client) -> Result<Vec<User>, MyError> {
    let stmt = include_str!("../sql/get_users.sql");
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>();

    Ok(results)
}

pub async fn increment_count(client: &Client) -> Result<Counter, MyError> {
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

pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
    let stmt = include_str!("../sql/add_user.sql");
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &user_info.email,
                &user_info.first_name,
                &user_info.last_name,
                &user_info.username,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}
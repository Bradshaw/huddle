mod config {
    use serde::Deserialize;
    #[derive(Debug, Default, Deserialize)]
    pub struct ExampleConfig {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "users")] // singular 'user' is a keyword..
    pub struct User {
        pub email: String,
        pub first_name: String,
        pub last_name: String,
        pub username: String,
    }
}

mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

mod db {
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    use crate::{errors::MyError, models::User};

    pub async fn initialize_db(client: &Client) -> Result<(), MyError> {
        Ok(client
            .batch_execute(include_str!("../sql/schema.sql"))
            .await?)
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

    pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
        let _stmt = include_str!("../sql/add_user.sql");
        let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

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
}

mod handlers {
    use actix_web::{web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};

    use crate::{db, errors::MyError, models::User};

    pub async fn initialize_db(db_pool: &Pool) -> Result<(), Error> {
        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        Ok(db::initialize_db(&client).await?)
    }

    pub async fn get_users(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let users = db::get_users(&client).await?;

        Ok(HttpResponse::Ok().json(users))
    }

    pub async fn add_user(
        user: web::Json<User>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
        let user_info: User = user.into_inner();

        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let new_user = db::add_user(&client, user_info).await?;

        Ok(HttpResponse::Ok().json(new_user))
    }
}

mod server {
    use actix_web::{web, middleware::Logger, App, HttpServer};
    use deadpool_postgres::Config;
    use env_logger::Env;
    use tokio_postgres::NoTls;

    use crate::{config::ExampleConfig, handlers::*};

    #[inline]
    fn get_env(key: &str) -> Option<String> {
        std::env::var(key).ok()
    }

    #[inline]
    fn get_env_or(key: &str, or: String) -> String {
        get_env(key).unwrap_or(or)
    }

    fn address() -> String {
        get_env_or("ADDRESS", "0.0.0.0:3001".into())
    }

    
    pub async fn go() -> std::io::Result<()> {
        env_logger::init_from_env(
            Env::default()
                .default_filter_or("debug")
                .default_write_style_or("auto"),
        );

        let mut pg_config = Config::new();
        pg_config.host = get_env("DB_HOST");
        pg_config.dbname = get_env("DB_DBNAME");
        pg_config.user = get_env("DB_USER");
        pg_config.password = get_env("DB_PASSWORD");

        let config = ExampleConfig{
            server_addr: address(),
            pg: pg_config,
        };

        let pool = config.pg.create_pool(None, NoTls).unwrap();

        initialize_db(&pool).await.unwrap();

        let server = HttpServer::new(move || {
            // move counter into the closure
            App::new()
                .wrap(Logger::default())
                .app_data(web::Data::new(pool.clone()))
                .service(
                    web::resource("/users")
                        .route(web::post().to(add_user))
                        .route(web::get().to(get_users)),
                )
        })
        .bind(config.server_addr.clone())?
        .run();

        println!("HTTP server listening at {}", config.server_addr);

        server.await
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::go().await
}
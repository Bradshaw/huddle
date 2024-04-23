use serde::Deserialize;
#[derive(Debug, Default, Deserialize)]
pub(crate) struct Config {
    pub server_addr: String,
    pub pg: deadpool_postgres::Config,
}

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

impl Config {
    pub(crate) fn from_env() -> Self {
        let mut pg_config = deadpool_postgres::Config::new();
        pg_config.host = get_env("DB_HOST");
        pg_config.dbname = get_env("DB_DBNAME");
        pg_config.user = get_env("DB_USER");
        pg_config.password = get_env("DB_PASSWORD");
        Self {
            server_addr: address(),
            pg: pg_config,
        }
    }
}
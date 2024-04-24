use serde::Deserialize;

use crate::errors::HuddleError;
#[derive(Debug, Default, Deserialize)]
pub(crate) struct Config {
    pub server_addr: String,
    pub database: DbConfig,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct DbConfig {
    host: String,
    dbname: String,
    user: String,
    password: String,
}

impl DbConfig {
    pub fn new(    host: String,
        dbname: String,
        user: String,
        password: String) -> Self {
        Self { host, dbname, user, password}
    }

    pub fn address(&self) -> String {
        let Self {host, dbname, user, password} = self;
        format!("postgres://{user}:{password}@{host}/{dbname}")
    }
}

#[inline]
fn get_env(key: &str) -> Result<String, HuddleError> {
    std::env::var(key).map_err(|_| HuddleError::MissingConfigurationError(key.into()))
}

#[inline]
fn get_env_or(key: &str, or: String) -> String {
    get_env(key).unwrap_or(or)
}

fn address() -> String {
    get_env_or("ADDRESS", "0.0.0.0:3001".into())
}

impl Config {
    pub(crate) fn from_env() -> Result<Self, HuddleError> {
        Ok(Self {
            server_addr: address(),
            database: DbConfig::new(
                get_env("DB_HOST")?,
                get_env("DB_DBNAME")?,
                get_env("DB_USER")?,
                get_env("DB_PASSWORD")?,
            )
        })
    }
}
use diesel::{pg::PgConnection, prelude::*};
use crate::{config::Config, errors::HuddleError, models::Counter, schema::{self}};

pub fn establish_connection() -> PgConnection {
    let config = Config::from_env().unwrap();
    PgConnection::establish(&config.database.address())
        .unwrap_or_else(|_| panic!("Error connecting to {}", config.database.address()))
}

pub async fn get_count() -> Result<Counter, HuddleError> {
    use self::schema::counter::dsl::*;
    #[allow(unused_variables)]
    let connection = &mut establish_connection();
    
    let results = counter
        .limit(1)
        .select(Counter::as_select())
        .load(connection)
        .expect("Error loading count");

    Ok(results[0])
}

pub async fn increment_count() -> Result<Counter, HuddleError> {
    use self::schema::counter::dsl::*;
    use crate::schema::counter;

    #[allow(unused_variables)]
    let connection = &mut establish_connection();

    let results = diesel::update(counter::table)
        .set(count.eq(count + 1))
        .returning(Counter::as_returning())
        .get_result(connection)
        .expect("Error updating count");
    Ok(results)
}
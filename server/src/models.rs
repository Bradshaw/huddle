use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize, Clone, Copy)]
#[diesel(table_name = crate::schema::counter)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Counter {
    pub id: i32,
    pub count: i32,
}
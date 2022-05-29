use std::env;

use diesel::{Connection, PgConnection};

#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("Could not read DATABASE_URL from env");

    PgConnection::establish(&database_url).expect("Error connecting to database")
}

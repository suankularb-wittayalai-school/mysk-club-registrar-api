#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use std::env;

mod routes;
mod structs;
mod utils;
use routes::health::*;
use routes::index::*;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let database_url =
        env::var("POSTGRES_DATABASE_URI").expect("POSTGRES_DATABASE_URI must be set");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    rocket::build()
        .mount("/", routes![get_index])
        .mount("/", routes![health_check])
        .manage(pool)
        .launch()
        .await?;

    Ok(())
}

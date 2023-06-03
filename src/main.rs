#[macro_use]
extern crate rocket;

mod routes;
mod structs;
mod utils;
use routes::health::*;
use routes::index::*;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![get_index])
        .mount("/", routes![health_check])
        .launch()
        .await?;

    Ok(())
}

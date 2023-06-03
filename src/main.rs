#[macro_use]
extern crate rocket;

mod routes;
use routes::index::*;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![get_index])
        .launch()
        .await?;

    Ok(())
}

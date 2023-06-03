// #[macro_use]
// extern crate rocket;

#[get("/")]
pub fn get_index() -> &'static str {
    "Hello, world!"
}

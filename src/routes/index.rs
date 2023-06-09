use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

use actix_web::{get, HttpResponse, Responder};

use crate::structs::auth::*;

#[get("/test-auth")]
pub async fn get_user(user: User) -> impl Responder {
    HttpResponse::Ok().json(user)
}

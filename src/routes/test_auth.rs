use actix_web::{get, HttpResponse, Responder};

use crate::structs::{auth::User, student::Student};

#[get("/test-auth")]
pub async fn get_user(student: Student) -> impl Responder {
    HttpResponse::Ok().json(student)
}

use actix_web::{get, HttpResponse, Responder};

use crate::structs::student::Student;

#[get("/test-auth")]
pub async fn get_user(student: Student) -> impl Responder {
    HttpResponse::Ok().json(student)
}

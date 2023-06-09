use actix_web::{get, web, HttpResponse, Responder};

use crate::AppState;
use crate::structs::*;

#[get("/health-check")]
pub async fn health_check(data: web::Data<AppState>) -> impl Responder {
    let pool = &data.db;
    let database_connection = match pool.acquire().await {
        Ok(_) => true,
        Err(_) => false,
    };

    // calculate database response time
    let start = std::time::Instant::now();

    let _ = match sqlx::query("SELECT 1")
        .execute(pool)
        .await
        {
            Ok(_) => true,
            Err(_) => false,
        };

    let database_response_time = start.elapsed().as_millis().to_string();

    let health_check_response =
        health::HealthCheckResponse::new(database_connection, database_response_time);
    let response = common::ResponseType::new(health_check_response, None, None);

    HttpResponse::Ok().json(response)
}
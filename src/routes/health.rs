// mod structs;
use rocket::serde::json::Json;
use rocket::State;

use sqlx::{Pool, Postgres};

use crate::structs::*;

#[get("/health-check")]
pub async fn health_check(
    pool: &State<Pool<Postgres>>,
) -> Json<common::ResponseType<health::HealthCheckResponse>> {
    let database_connection = match pool.acquire().await {
        Ok(_) => true,
        Err(_) => false,
    };

    // calculate database response time
    let start = std::time::Instant::now();

    let _ = sqlx::query("SELECT 1")
        .execute(pool.inner())
        .await
        .expect("Failed to execute query");

    let database_response_time = start.elapsed().as_millis().to_string();

    let health_check_response =
        health::HealthCheckResponse::new(database_connection, database_response_time);
    let response = common::ResponseType::new(health_check_response, None, None);

    Json(response)
}

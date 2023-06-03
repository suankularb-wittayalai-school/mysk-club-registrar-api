// mod structs;
use rocket::serde::json::Json;

use crate::structs::*;

#[get("/health-check")]
pub fn health_check() -> Json<common::ResponseType<health::HealthCheckResponse>> {
    // get memory consumption in megabytes
    let health_check_response = health::HealthCheckResponse::new(true, "0.0001".to_string());
    let response = common::ResponseType::new(health_check_response, None, None);

    Json(response)
}

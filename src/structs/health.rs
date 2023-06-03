use rocket::serde::{Deserialize, Serialize};

mod health;

#[derive(Serialize, Deserialize)]
pub struct HealthCheckResponse {
    response_time: String,
    database_connection: bool,
    database_response_time: String,
    memory_consumption: f32,
}

// use crate::utils::memory::get_memory_usage;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HealthCheckResponse {
    server_time: String,
    database_connection: bool,
    database_response_time: String,
}

impl HealthCheckResponse {
    pub fn new(database_connection: bool, database_response_time: String) -> Self {
        // let (total_memory, used_memory) = get_memory_usage();

        HealthCheckResponse {
            server_time: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            database_connection,
            database_response_time,
            // memory_consumption: used_memory as f32 / 1024.0 / 1024.0,
        }
    }
}

mod structs;

use structs::*;

#[get("/")]
pub fn get_index() -> common::ResponseType<health: HealthCheckResponse> {
    let health_check_response = HealthCheckResponse {
        response_time: "0.0001".to_string(),
        database_connection: true,
        database_response_time: "0.0002".to_string(),
        memory_consumption: 0.0003,
    };

    common::ResponseType::new(health_check_response)
}

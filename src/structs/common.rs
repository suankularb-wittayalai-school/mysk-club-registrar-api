use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorType {
    id: String,
    error_type: String, // TODO: enum
    detail: String,
    source: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PaginationType {
    first: String,
    last: String,
    next: Option<String>,
    prev: Option<String>,
    size: u32,
    total: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MetadataType {
    timestamp: DateTime<Utc>,
    pagination: PaginationType,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseType<T> {
    api_version: String,
    data: Option<T>,
    error: Option<ErrorType>,
    meta: Option<MetadataType>,
}

impl<T> ResponseType<T> {
    pub fn new(data: T, error: Option<ErrorType>, meta: Option<MetadataType>) -> Self {
        let version = env!("CARGO_PKG_VERSION").to_string();

        ResponseType {
            api_version: version,
            data: Some(data),
            error,
            meta,
        }
    }
}

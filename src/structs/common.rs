use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

mod common;

#[derive(Serialize, Deserialize)]
pub struct ErrorType {
    id: String,
    error_type: String, // TODO: enum
    detail: String,
    source: String,
}

#[derive(Serialize, Deserialize)]
pub struct PaginationType {
    first: String,
    last: String,
    next: Option<String>,
    prev: Option<String>,
    size: u32,
    total: u32,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataType {
    timestamp: DateTime<Utc>,
    pagination: PaginationType,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseType<T> {
    api_version: String,
    data: Option<T>,
    error: Option<ErrorType>,
    meta: Option<MetadataType>,
}

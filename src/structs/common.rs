use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorType {
    id: String,
    error_type: String, // TODO: enum
    detail: String,
    source: String,
}

impl ErrorType {
    pub fn new(id: String, error_type: String, detail: String, source: String) -> Self {
        ErrorType {
            id,
            error_type,
            detail,
            source,
        }
    }
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

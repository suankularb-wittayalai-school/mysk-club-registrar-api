use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct MultiLangString {
    #[serde(rename = "en-US")]
    pub en: Option<String>,
    pub th: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct FlexibleMultiLangString {
    #[serde(rename = "en-US")]
    pub en: Option<String>,
    pub th: Option<String>,
}

impl std::fmt::Display for MultiLangString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ en: {:?}, th: {} }}", self.en, self.th)
    }
}

impl MultiLangString {
    pub fn new(en: Option<String>, th: String) -> MultiLangString {
        MultiLangString { en, th }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FetchLevel {
    Default,
    Compact,
    IdOnly,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct FilterConfig<T> {
    pub data: Option<T>,
    pub q: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct SortingConfig<T> {
    // by is array of key of T as string
    pub by: Option<Vec<T>>,
    pub ascending: bool,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PaginationConfig {
    pub p: u32,
    pub size: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct RequestType<T, Queryable, Sortable> {
    pub data: Option<T>,
    pub pagination: Option<PaginationConfig>,
    pub filter: Option<FilterConfig<Queryable>>,
    pub sorting: Option<SortingConfig<Sortable>>,
    pub fetch_level: Option<FetchLevel>,
    pub descendant_fetch_level: Option<FetchLevel>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ErrorType<T> {
    pub id: String,
    pub code: u32,
    pub error_type: T, // TODO: enum
    pub detail: String,
    pub source: String,
}

impl<T> ErrorType<T> {
    pub fn to_status_code(&self) -> StatusCode {
        match self.code {
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            403 => StatusCode::FORBIDDEN,
            404 => StatusCode::NOT_FOUND,
            405 => StatusCode::METHOD_NOT_ALLOWED,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Display for ErrorType<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ id: {}, code: {}, error_type: {}, detail: {}, source: {} }}",
            self.id, self.code, self.error_type, self.detail, self.source
        )
    }
}

impl std::error::Error for ErrorType<String> {
    fn description(&self) -> &str {
        &self.detail
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PaginationType {
    first: String,
    last: String,
    next: Option<String>,
    prev: Option<String>,
    size: u32,
    total: u32,
}
impl std::fmt::Display for PaginationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ first: {}, last: {}, next: {:?}, prev: {:?}, size: {}, total: {} }}",
            self.first, self.last, self.next, self.prev, self.size, self.total
        )
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct MetadataType {
    timestamp: DateTime<Utc>,
    pagination: PaginationType,
}

impl std::fmt::Display for MetadataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ timestamp: {}, pagination: {} }}",
            self.timestamp, self.pagination
        )
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ResponseType<T, E = ErrorType<String>> {
    api_version: String,
    data: Option<T>,
    error: Option<E>,
    meta: Option<MetadataType>,
}

impl<T, E> ResponseType<T, E> {
    pub fn new(data: T, error: Option<E>, meta: Option<MetadataType>) -> Self {
        let version = env!("CARGO_PKG_VERSION").to_string();

        ResponseType {
            api_version: version,
            data: Some(data),
            error,
            meta,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ErrorResponseType {
    api_version: String,
    error: ErrorType<String>,
    data: Option<String>, // always None
    meta: Option<MetadataType>,
}

impl ErrorResponseType {
    pub fn new(error: ErrorType<String>, meta: Option<MetadataType>) -> Self {
        let version = env!("CARGO_PKG_VERSION").to_string();

        ErrorResponseType {
            api_version: version,
            error,
            data: None,
            meta,
        }
    }
}

impl std::fmt::Display for ErrorResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ api_version: {}, error: {}, data: {:?}, meta: {:?} }}",
            self.api_version, self.error, self.data, self.meta
        )
    }
}

// implement as Actix web error response type

impl error::ResponseError for ErrorResponseType {
    fn status_code(&self) -> StatusCode {
        // convert error type to status code
        self.error.to_status_code()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let body = serde_json::to_string(&self).unwrap();

        actix_web::HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(body)
    }
}

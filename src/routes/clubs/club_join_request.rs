use actix_web::{get, patch, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::structs::{
    clubs::{Club, ClubSortableField, QueryableClub, UpdatableClub},
    common::{ErrorResponseType, ErrorType, MetadataType, RequestType, ResponseType},
    student::Student,
};

use crate::AppState;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde_qs;
use uuid::Uuid;

use crate::structs::{
    // clubs::{Club, ClubSortableField, QueryableClub, UpdatableClub}
    club_request::{ClubRequestSortableField, ClubRequestTable, QueryableClubRequest},
    common::{ErrorResponseType, ErrorType, MetadataType, RequestType, ResponseType},
    student::Student,
};

use crate::AppState;

#[get("/clubs/{club_id}/join_requests")]
pub async fn query_club_requests(
    data: web::Data<AppState>,
    request: HttpRequest,
) -> impl Responder {
    let pool = &data.db;

    let request_query = serde_qs::from_str::<
        RequestType<ClubRequestTable, QueryableClubRequest, ClubRequestSortableField>,
    >(&request.query_string());

    let request_query = match request_query {
        Ok(request_query) => request_query,
        Err(e) => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 400,
                    error_type: "bad_request".to_string(),
                    detail: e.to_string(),
                    source: "/clubs/{club_id}/join_requests".to_string(),
                },
                None::<MetadataType>,
            );

            return HttpResponse::BadRequest().json(response);
        }
    };

    let club_request = match ClubRequestTable::query(pool, &request_query).await {
        Ok(club_request) => club_request,
        Err(e) => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 500,
                    error_type: "internal_server_error".to_string(),
                    detail: e.to_string(),
                    source: "/clubs/{club_id}/join_requests".to_string(),
                },
                None::<MetadataType>,
            );

            return HttpResponse::InternalServerError().json(response);
        }
    };

    let response: ResponseType<Vec<ClubRequestTable>, _> =
        ResponseType::new(club_request, None::<String>, None::<MetadataType>);

    HttpResponse::Ok().json(response)
}

use std::fmt::format;

use actix_web::{get, patch, web, HttpRequest, HttpResponse, Responder};
use serde_qs;
use uuid::Uuid;

use crate::structs::{
    // clubs::{Club, ClubSortableField, QueryableClub, UpdatableClub}
    club_request::{
        ClubRequest, ClubRequestSortableField, QueryableClubRequest, UpdatableClubRequest,
    },
    clubs::{Club, SubmissionStatus},
    common::{ErrorResponseType, ErrorType, FetchLevel, MetadataType, RequestType, ResponseType},
    student::Student,
};

use crate::AppState;

#[get("/join_requests/{join_request_id}")]
pub async fn get_club_request_by_id(
    data: web::Data<AppState>,
    request: HttpRequest,
    join_request_id: web::Path<Uuid>,
) -> impl Responder {
    let pool = &data.db;
    let join_request_id = join_request_id.into_inner();

    let request_query = serde_qs::from_str::<
        RequestType<ClubRequest, QueryableClubRequest, ClubRequestSortableField>,
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
                    source: "/clubs/join_requests".to_string(),
                },
                None::<MetadataType>,
            );

            return HttpResponse::BadRequest().json(response);
        }
    };

    let club_request = match ClubRequest::get_by_id(
        pool,
        join_request_id,
        request_query.fetch_level,
        request_query.descendant_fetch_level,
    )
    .await
    {
        Ok(club_request) => club_request,
        Err(e) => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 500,
                    error_type: "internal_server_error".to_string(),
                    detail: e.to_string(),
                    source: "/clubs/join_requests".to_string(),
                },
                None::<MetadataType>,
            );

            return HttpResponse::InternalServerError().json(response);
        }
    };

    let response: ResponseType<ClubRequest, _> =
        ResponseType::new(club_request, None::<String>, None::<MetadataType>);

    HttpResponse::Ok().json(response)
}

#[patch("/join_requests/{join_request_id}")]
pub async fn approve_or_reject_club_request(
    data: web::Data<AppState>,
    request: HttpRequest,
    join_request_id: web::Path<Uuid>,
    student: Student,
    request_body: web::Json<
        RequestType<UpdatableClubRequest, QueryableClubRequest, ClubRequestSortableField>,
    >,
) -> impl Responder {
    let pool = &data.db;
    let join_request_id = join_request_id.into_inner();

    let club_request = match ClubRequest::get_by_id(
        pool,
        join_request_id,
        Some(FetchLevel::Default),
        Some(FetchLevel::IdOnly),
    )
    .await
    {
        Ok(club_request) => club_request,
        Err(e) => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 500,
                    error_type: "internal_server_error".to_string(),
                    detail: e.to_string(),
                    source: format!("/join_requests/{join_request_id}"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::InternalServerError().json(response);
        }
    };

    // check if the user is the staff of the club
    let student_id = match student {
        Student::IdOnly(student) => student.id,
        Student::Compact(student) => student.id,
        Student::Default(student) => student.id,
    };

    let club_id = match club_request {
        ClubRequest::Default(club_request) => match club_request.club {
            Club::IdOnly(club) => club.id,
            Club::Compact(club) => club.id,
            Club::Default(club) => club.id,
        },
        _ => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 500,
                    error_type: "internal_server_error".to_string(),
                    detail: "club_request is fetch unexpectedly".to_string(),
                    source: format!("/join_requests/{join_request_id}"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::InternalServerError().json(response);
        }
    };

    // make sure the student is club staff
    let res = sqlx::query!(
        r#"
        SELECT COUNT(id) FROM club_staffs WHERE student_id = $1 AND club_id = $2
        "#,
        student_id as i64,
        club_id
    )
    .fetch_one(pool)
    .await;

    // dbg!(&res, student_id, club_id, &request);

    // if the student is not club staff, return 403
    if let Ok(res) = res {
        if res.count.unwrap_or(0) == 0 {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 403,
                    error_type: "forbidden".to_string(),
                    detail: "the student is not club staff".to_string(),
                    source: format!("/join_requests/{join_request_id}"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::Forbidden().json(response);
        }
    }

    let res = match &request_body.data {
        Some(data) => match data.membership_status {
            SubmissionStatus::Approved => {
                ClubRequest::approve_request(
                    pool,
                    join_request_id,
                    request_body.fetch_level.clone(),
                    request_body.descendant_fetch_level.clone(),
                )
                .await
            }
            SubmissionStatus::Declined => {
                ClubRequest::deny_request(
                    pool,
                    join_request_id,
                    request_body.fetch_level.clone(),
                    request_body.descendant_fetch_level.clone(),
                )
                .await
            }
            SubmissionStatus::Pending => {
                let response: ErrorResponseType = ErrorResponseType::new(
                    ErrorType {
                        id: Uuid::new_v4().to_string(),
                        code: 400,
                        error_type: "bad_request".to_string(),
                        detail: "membership_status can not be pending".to_string(),
                        source: format!("/join_requests/{join_request_id}"),
                    },
                    None::<MetadataType>,
                );

                return HttpResponse::BadRequest().json(response);
            }
        },
        None => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 400,
                    error_type: "bad_request".to_string(),
                    detail: "data is required".to_string(),
                    source: format!("/join_requests/{join_request_id}"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::BadRequest().json(response);
        }
    };

    let club_request = match res {
        Ok(club_request) => club_request,
        Err(e) => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 500,
                    error_type: "internal_server_error".to_string(),
                    detail: e.to_string(),
                    source: format!("/join_requests/{join_request_id}"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::InternalServerError().json(response);
        }
    };

    let response: ResponseType<ClubRequest, _> =
        ResponseType::new(club_request, None::<String>, None::<MetadataType>);

    HttpResponse::Ok().json(response)
}

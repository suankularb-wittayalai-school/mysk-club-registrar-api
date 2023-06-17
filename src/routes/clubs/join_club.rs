use std::fmt::format;

use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde_qs;
use uuid::Uuid;

use crate::structs::{
    club_request::{
        ClubRequest, ClubRequestSortableField, CreatableClubRequest, QueryableClubRequest,
    },
    clubs::{Club, SubmissionStatus},
    common::{ErrorResponseType, ErrorType, FetchLevel, MetadataType, RequestType, ResponseType},
    student::Student,
};

use crate::utils::date::get_current_academic_year;

use crate::AppState;

#[post("/clubs/{club_id}/join")]
pub async fn join_club_by_id(
    data: web::Data<AppState>,
    club_id: web::Path<Uuid>,
    student: Student,
    request: web::Json<RequestType<String, QueryableClubRequest, ClubRequestSortableField>>,
) -> impl Responder {
    let pool = &data.db;
    let club_id = club_id.into_inner();

    // check if the club exists by Club::get_by_id

    // check if the student is in the club by SELECT COUNT(id) FROM club_members WHERE club_id = club_id AND student_id = student_id AND membership_status = 'approved
    // if yes, return 409
    // if no, insert into club_requests

    let club_id = match Club::get_by_id(pool, club_id, Some(FetchLevel::IdOnly), None).await {
        Ok(club) => match club {
            Club::IdOnly(club) => club.id,
            _ => {
                let response: ErrorResponseType = ErrorResponseType::new(
                    ErrorType {
                        id: Uuid::new_v4().to_string(),
                        code: 500,
                        error_type: "internal_server_error".to_string(),
                        detail: "club is fetch unexpectedly".to_string(),
                        source: format!("/clubs/{club_id}/join"),
                    },
                    None::<MetadataType>,
                );

                return HttpResponse::NotFound().json(response);
            }
        },
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

    let student_id = match student {
        Student::IdOnly(student) => student.id,
        Student::Compact(student) => student.id,
        Student::Default(student) => student.id,
    };

    let club_request_count = sqlx::query!(
        r#"
        SELECT COUNT(id) FROM club_members WHERE club_id = $1 AND student_id = $2 AND year = $3 AND (membership_status = 'approved' OR membership_status = 'pending')
        "#,
        club_id,
        student_id as i64,
        get_current_academic_year() as i64
    ).fetch_one(pool).await;

    if let Ok(club_request_count) = club_request_count {
        if club_request_count.count.unwrap_or(0) > 0 {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 409,
                    error_type: "conflict".to_string(),
                    detail: "student is already in the club".to_string(),
                    source: format!("/clubs/{club_id}/join"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::Conflict().json(response);
        }
    }

    let club_request = CreatableClubRequest {
        club_id,
        student_id: student_id as i64,
        year: Some(get_current_academic_year() as i64),
    };

    let res = ClubRequest::create(
        pool,
        club_request,
        request.fetch_level.clone(),
        request.descendant_fetch_level.clone(),
    )
    .await;

    match res {
        Ok(club_request) => {
            let response: ResponseType<ClubRequest, _> =
                ResponseType::new(club_request, None::<String>, None::<MetadataType>);

            HttpResponse::Ok().json(response)
        }
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

            HttpResponse::InternalServerError().json(response)
        }
    }
}

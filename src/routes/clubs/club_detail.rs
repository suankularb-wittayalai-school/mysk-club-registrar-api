use actix_web::{get, patch, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::structs::{
    clubs::{Club, ClubSortableField, QueryableClub, UpdatableClub},
    common::{ErrorResponseType, ErrorType, MetadataType, RequestType, ResponseType},
    student::Student,
};

use crate::AppState;

// #[derive(Deserialize)]
// pub struct ClubDetailPath {
//     club_id: Uuid,
// }

#[get("/clubs/{club_id}")]
pub async fn get_club_by_id(
    data: web::Data<AppState>,
    club_id: web::Path<Uuid>,
    request_query: web::Query<RequestType<Club, QueryableClub, ClubSortableField>>,
) -> impl Responder {
    let pool = &data.db;
    let club_id = club_id.into_inner();

    let club = Club::get_by_id(
        pool,
        club_id,
        request_query.fetch_level.clone(),
        request_query.descendant_fetch_level.clone(),
    )
    .await;

    match club {
        Ok(club) => {
            let response: ResponseType<Club, _> =
                ResponseType::new(club, None::<String>, None::<MetadataType>);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 404,
                    error_type: "entity_not_found".to_string(),
                    detail: e.to_string(),
                    source: "/clubs/{club_id}".to_string(),
                },
                None::<MetadataType>,
            );

            return HttpResponse::NotFound().json(response);
        }
    }
}

#[patch("/clubs/{club_id}")]
pub async fn update_club_by_id(
    data: web::Data<AppState>,
    club_id: web::Path<Uuid>,
    student: Student,
    request: web::Json<RequestType<UpdatableClub, QueryableClub, ClubSortableField>>,
) -> impl Responder {
    let pool = &data.db;
    let club_id = club_id.into_inner();

    let data = match &request.data {
        Some(data) => data,
        None => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 400,
                    error_type: "bad_request".to_string(),
                    detail: "request body is empty".to_string(),
                    source: format!("/clubs/{club_id}"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::BadRequest().json(response);
        }
    };

    let student_id = match student {
        Student::IdOnly(student) => student.id,
        Student::Compact(student) => student.id,
        Student::Default(student) => student.id,
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
                    source: format!("/clubs/{club_id}"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::Forbidden().json(response);
        }
    }

    let club = Club::update_by_id(
        pool,
        club_id,
        data,
        request.fetch_level.clone(),
        request.descendant_fetch_level.clone(),
    )
    .await;

    match club {
        Ok(club) => {
            let response: ResponseType<Club, _> =
                ResponseType::new(club, None::<String>, None::<MetadataType>);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 404,
                    error_type: "entity_not_found".to_string(),
                    detail: e.to_string(),
                    source: format!("/clubs/{club_id}"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::NotFound().json(response);
        }
    }
}

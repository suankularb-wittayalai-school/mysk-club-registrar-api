use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::structs::{
    clubs::{Club, ClubSortableField, QueryableClub, UpdatableClub},
    common::{ErrorResponseType, ErrorType, MetadataType, RequestType, ResponseType, FetchLevel},
    contacts::{Contact, CreateContact},
    student::Student,
};

use crate::AppState;

#[post("/clubs/{club_id}/contacts")]
pub async fn create_contact_for_club(
    data: web::Data<AppState>,
    club_id: web::Path<Uuid>,
    student: Student,
    request: web::Json<RequestType<CreateContact, QueryableClub, ClubSortableField>>,
) -> impl Responder {
    let pool = &data.db;
    let club_id = club_id.into_inner();

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

    let club = Club::get_by_id(pool, club_id, Some(FetchLevel::IdOnly), None).await;

    let club = match club {
        Ok(club) => club,
        Err(e) => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 404,
                    error_type: "not_found".to_string(),
                    detail: format!("club with id {} not found", club_id),
                    source: format!("/clubs/{club_id}"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::NotFound().json(response);
        }
    };

    let data = match &request.data {
        Some(data) => data,
        None => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 400,
                    error_type: "bad_request".to_string(),
                    detail: "request body is empty".to_string(),
                    source: format!("/clubs/{club_id}/contacts"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::BadRequest().json(response);
        }
    };

    let contact = Contact::create(pool, data, FetchLevel::IdOnly).await;

    let contact = match contact {
        Ok(contact) => contact,
        Err(e) => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 500,
                    error_type: "internal_server_error".to_string(),
                    detail: e.to_string(),
                    source: format!("/clubs/{club_id}/contacts"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::InternalServerError().json(response);
        }
    };

    // insert into club_contacts
    let res = sqlx::query!(
        r#"
        INSERT INTO club_contacts (club_id, contact_id) VALUES ($1, $2)
        "#,
        club_id,
        match contact {
            Contact::IdOnly(contact) => contact.id,
            Contact::Compact(contact) => contact.id,
            Contact::Default(contact) => contact.id,
        } as i64
    )
    .execute(pool)
    .await;

    match res {
        Ok(_) => {
            let club = Club::get_by_id(pool, club_id, request.fetch_level.clone(), request.descendant_fetch_level.clone()).await;
            
            let club = match club {
                Ok(club) => club,
                Err(e) => {
                    let response: ErrorResponseType = ErrorResponseType::new(
                        ErrorType {
                            id: Uuid::new_v4().to_string(),
                            code: 404,
                            error_type: "not_found".to_string(),
                            detail: format!("club with id {} not found", club_id),
                            source: format!("/clubs/{club_id}"),
                        },
                        None::<MetadataType>,
                    );

                    return HttpResponse::NotFound().json(response);
                }
            };

            let response: ResponseType<Club, MetadataType> = ResponseType::new(
                club,
                None::<MetadataType>,
                None,
            );

            return HttpResponse::Ok().json(response);
        },
        Err(e) => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 500,
                    error_type: "internal_server_error".to_string(),
                    detail: format!("failed to create contact for club with id {}", club_id),
                    source: format!("/clubs/{club_id}/contacts"),
                },
                None::<MetadataType>,
            );

            return HttpResponse::InternalServerError().json(response);
        }
    }
}

use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::structs::{
    clubs::{Club, ClubSortableField, QueryableClub, UpdatableClub},
    common::{ErrorResponseType, ErrorType, MetadataType, RequestType, ResponseType},
    student::Student,
};

use crate::AppState;

#[post("/clubs/{club_id}/contacts")]
pub async fn create_contact_for_club(
    data: web::Data<AppState>,
    club_id: web::Path<Uuid>,
    student: Student,
    request: web::Json<RequestType<UpdatableClub, QueryableClub, ClubSortableField>>,
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

    let club = Club::get_by_id(pool, club_id, None, None).await;

    todo!()
}

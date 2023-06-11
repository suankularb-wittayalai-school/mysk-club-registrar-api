use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

use crate::structs::{
    clubs::Club,
    common::{ErrorResponseType, ErrorType, FetchLevel, MetadataType, ResponseType},
};

use crate::AppState;

// #[derive(Deserialize)]
// pub struct ClubDetailPath {
//     club_id: Uuid,
// }

#[get("/clubs/{club_id}")]
pub async fn get_club_by_id(data: web::Data<AppState>, club_id: web::Path<Uuid>) -> impl Responder {
    let pool = &data.db;
    let club_id = club_id.into_inner();

    let club = Club::get_by_id(pool, club_id, None, Some(FetchLevel::IdOnly)).await;

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

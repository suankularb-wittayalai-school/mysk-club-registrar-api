use actix_web::{get, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::structs::{
    clubs::{Club, ClubSortableField, QueryableClub},
    common::{ErrorResponseType, ErrorType, MetadataType, RequestType, ResponseType},
};

use crate::AppState;

#[get("/clubs")]
pub async fn query_clubs(
    data: web::Data<AppState>,
    request_query: web::Query<RequestType<QueryableClub, ClubSortableField>>,
) -> impl Responder {
    let pool = &data.db;

    println!("{:?}", request_query);

    // to fetch with query params
    // and use nested json query params
    // http://localhost:8080/clubs?fetch_level=1&descendant_fetch_level=1&filter=

    let clubs = match Club::query(pool, &request_query).await {
        Ok(clubs) => clubs,
        Err(e) => {
            let response: ErrorResponseType = ErrorResponseType::new(
                ErrorType {
                    id: Uuid::new_v4().to_string(),
                    code: 500,
                    error_type: "internal_server_error".to_string(),
                    detail: e.to_string(),
                    source: "/clubs".to_string(),
                },
                None::<MetadataType>,
            );

            return HttpResponse::InternalServerError().json(response);
        }
    };

    let response: ResponseType<Vec<Club>, _> =
        ResponseType::new(clubs, None::<String>, None::<MetadataType>);

    HttpResponse::Ok().json(response)
}

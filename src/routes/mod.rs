use actix_web::web;

pub(crate) mod clubs;
pub(crate) mod health;
pub(crate) mod index;
pub(crate) mod test_auth;

use utoipa_swagger_ui::SwaggerUi;

use utoipa::OpenApi;

use crate::structs::{classroom, clubs as clubsType, common, contacts, student};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "MySK Club Registry API",
        description = "API to interact with clubs data built especially for Activity Day"
    ),
    components(schemas(
        student::IdOnlyStudent,
        student::CompactStudent,
        student::DefaultStudent,
        classroom::IdOnlyClassroom,
        classroom::CompactClassroom,
        classroom::DefaultClassroom,
        contacts::IdOnlyContact,
        contacts::CompactContact,
        contacts::DefaultContact,
        contacts::ContactType,
        clubsType::IdOnlyClub,
        clubsType::CompactClub,
        clubsType::DefaultClub,
        clubsType::Club,
        contacts::Contact,
        classroom::Classroom,
        student::Student,
        common::MultiLangString,
    ))
)]
struct ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index::get_index);
    cfg.service(health::health_check);
    cfg.service(test_auth::get_user);
    cfg.service(clubs::club_detail::get_club_by_id);
    cfg.service(clubs::club_detail::update_club_by_id);
    cfg.service(clubs::clubs::query_clubs);
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}

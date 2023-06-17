use actix_web::web;

pub(crate) mod clubs;
pub(crate) mod health;
pub(crate) mod index;
pub(crate) mod test_auth;
// pub(crate) mod

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use crate::structs::{
    auth, classroom, club_request, clubs as clubsType, common, contacts, student,
};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "JWT Token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("Bearer")
                    .build(),
            ),
        )
    }
}

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
        clubsType::ActivityDayHouse,
        clubsType::IdOnlyClub,
        clubsType::CompactClub,
        clubsType::DefaultClub,
        clubsType::Club,
        contacts::Contact,
        classroom::Classroom,
        student::Student,
        common::MultiLangString,
        auth::User,
        auth::UserRoles,
        club_request::ClubRequestTable,
    )),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index::get_index);
    cfg.service(health::health_check);
    cfg.service(test_auth::get_user);
    cfg.service(clubs::club_detail::get_club_by_id);
    cfg.service(clubs::club_detail::update_club_by_id);
    cfg.service(clubs::clubs::query_clubs);
    cfg.service(clubs::club_contact::create_contact_for_club);
    cfg.service(clubs::club_join_request::query_club_requests);
    cfg.service(clubs::club_join_request_detail::get_club_request_by_id);
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}

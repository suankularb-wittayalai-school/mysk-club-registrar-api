use actix_web::web;

pub(crate) mod clubs;
pub(crate) mod health;
pub(crate) mod index;
pub(crate) mod test_auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index::get_index);
    cfg.service(health::health_check);
    cfg.service(test_auth::get_user);
    cfg.service(clubs::club_detail::get_club_by_id);
    cfg.service(clubs::clubs::query_clubs);
}

use actix_web::web;

pub(crate) mod health;
pub(crate) mod index;
pub(crate) mod test_auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index::get_index);
    cfg.service(health::health_check);
    cfg.service(test_auth::get_user);
}

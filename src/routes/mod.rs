use actix_web::web;

pub(crate) mod health;
pub(crate) mod index;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index::get_index);
    cfg.service(health::health_check);
}

mod user_services;
use actix_web::web::{self, ServiceConfig};

use crate::persistence::repository::Repository;

pub fn service<R: Repository>(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/v1").configure(user_services::service::<R>));
}

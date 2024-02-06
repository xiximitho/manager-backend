use actix_web::web;

use crate::handlers;

pub fn users_config(config: &mut web::ServiceConfig) {
    config
        .route("/users", web::get().to(handlers::get_users))
        .route("/users/{id}", web::get().to(handlers::get_user_by_id))
        .route("/users", web::post().to(handlers::add_user));
}

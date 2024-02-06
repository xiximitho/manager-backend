use actix_web::web;

#[path = "../handlers/users.rs"]
mod users;

pub fn users_config(config: &mut web::ServiceConfig) {
    config
        .route("/users", web::get().to(users::get_users))
        .route("/users/{id}", web::get().to(users::get_user_by_id))
        .route("/users", web::post().to(users::add_user));
}

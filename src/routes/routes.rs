mod login;
//mod logout;

use actix_web::web::{get, post, scope, ServiceConfig};
use actix_web::Responder;

pub async fn ping() -> impl Responder {
    "ping"
}
#[path = "../handlers/users.rs"]
mod users;
pub fn users_config(app: &mut ServiceConfig) {
    app.service(
        scope("v1/auth")
            .route("login", get().to(login::login))
            .route("login", post().to(login::login))
            .route("logout", get().to(login::logout))
            .route("ping", get().to(ping))
            .route("users", get().to(users::get_users)),
    );
    /*
    config
        .route("/users", web::get().to(users::get_users))
        .route("/users/{id}", web::get().to(users::get_user_by_id))
        .route("/users", web::post().to(users::add_user))
        .route("login", web::get().to(login::login))
        .route("login", web::post().to(login::login));*/
}
/*
#[path = "../handlers/patients.rs"]
mod patients;
pub fn patients_config(config: &mut web::ServiceConfig) {
    config
        .route("/patients", web::get().to(patients::get_patients))
        /*.route("/patients/{id}", web::get().to(users::get_user_by_id))*/
        .route("/patients", web::post().to(patients::add_patient));
}
*/

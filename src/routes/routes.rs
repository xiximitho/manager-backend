use actix_web::web;
mod login;
#[path = "../handlers/users.rs"]
mod users;

pub fn users_config(config: &mut web::ServiceConfig) {
    config
        .route("/users", web::get().to(users::get_users))
        .route("/users/{id}", web::get().to(users::get_user_by_id))
        .route("/users", web::post().to(users::add_user))
        .route("login", web::get().to(login::login));
}

#[path = "../handlers/patients.rs"]
mod patients;
pub fn patients_config(config: &mut web::ServiceConfig) {
    config
        .route("/patients", web::get().to(patients::get_patients))
        /*.route("/patients/{id}", web::get().to(users::get_user_by_id))*/
        .route("/patients", web::post().to(patients::add_patient));
}

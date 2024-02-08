use crate::json_serialization::{login::Login, login_response::LoginResponse};
use crate::jwt::JwToken;
use crate::models::users::User;
use crate::schema::users;
use crate::Pool;
use actix_web::{http::header::ContentType, web, HttpResponse};
use diesel::prelude::*;

pub async fn login(credentials: web::Json<Login>, pool: web::Data<Pool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();

    let password = credentials.password.clone();
    let users = users::table
        .filter(users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&mut conn)
        .unwrap();
    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap();
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap();
    }
    match users[0].clone().verify(password) {
        true => {
            let user_id = users[0].clone().id;
            let token = JwToken::new(user_id);
            let raw_token = token.encode();
            let response = LoginResponse {
                token: raw_token.clone(),
            };
            let body = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok()
                .append_header(("token", raw_token))
                .content_type(ContentType::json())
                .body(body)
        }
        false => HttpResponse::Unauthorized().await.unwrap(),
    }
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            "<html>\
                <script>\
                    localStorage.removeItem('user-token'); \
                    window.location.replace(
                        document.location.origin);\
                </script>\
              </html>",
        )
}

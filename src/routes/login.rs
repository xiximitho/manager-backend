use crate::json_serialization::login::Login;
use crate::jwt::JwToken;
use crate::models::users::User;
use crate::schema::users;
use crate::Pool;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn login(credentials: web::Json<Login>, pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().unwrap();

    let password = credentials.password.clone();
    let users = users::table
        .filter(users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&mut conn)
        .unwrap();
    if users.len() == 0 {
        return HttpResponse::NotFound();
    } else if users.len() > 1 {
        return HttpResponse::Conflict();
    }
    match users[0].verify(password) {
        true => {
            let token = JwToken::new(users[0].id);
            let raw_token = token.encode();
            HttpResponse::Ok()
                .append_header(("token", raw_token))
                .take()
        }
        false => HttpResponse::Unauthorized(),
    }
}
/* find(user_id).get_result::<User>(&mut conn);

let users = users::table
    .filter(users::columns::username.eq(credentials.username.clone()))
    .load::<User>(&db.connection)
    .unwrap();
if users.len() == 0 {
    return HttpResponse::NotFound();
} else if users.len() > 1 {
    return HttpResponse::Conflict();
}
match users[0].verify(password) {
    true => {
        let token = JwToken::new(users[0].id);
        let raw_token = token.encode();
        HttpResponse::Ok()
            .append_header(("token", raw_token))
            .take()
    }
    false => HttpResponse::Unauthorized(),
}*/

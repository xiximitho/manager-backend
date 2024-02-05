use super::models::User;
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Responder};
use diesel::{QueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn get_users(db: web::Data<Pool>) -> impl Responder {
    let results = users
    .limit(5)
    .select(User::as_select())
    .load(&mut db.get().unwrap())
    .expect("Erro ao carregar");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.first_name);
        println!("-----------\n");
        println!("{}", user.email);
    }

    format!("hello from get users")
}
/*
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().content_type("application/json").body("aa"))
        .map_err(|_| error)?)
}
 */
fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let items = users.load::<User>(&mut conn)?;
    Ok(items)
}
/*
pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_user_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().body(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
*/
/*
fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&mut conn)
}*/
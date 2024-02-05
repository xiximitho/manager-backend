use super::models::User;
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::QueryDsl;
use std::vec::Vec;

pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    match get_all_users(db).await {
        Ok(v_users) => {
            let json_result = serde_json::to_string(&v_users);

            match json_result {
                Ok(json) => Ok(HttpResponse::Ok().json(json)),
                Err(_) => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let items = users.load::<User>(&mut conn)?;
    Ok(items)
}

pub async fn get_user_by_id(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    
    match db_get_user_by_id(db, user_id.into_inner()).await {
        Ok(user_db_l) => {
            let json_result = serde_json::to_string(&user_db_l);
            match json_result {
                Ok(json) => Ok(HttpResponse::Ok().json(json)),
                Err(_) => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let user = users.find(user_id).get_result::<User>(&mut conn)?;
    Ok(user)
}
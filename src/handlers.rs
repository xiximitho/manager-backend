use super::models::User;
use super::schema::users::dsl::*;
use super::Pool;
use crate::{diesel::RunQueryDsl, models::NewUser};
use actix_web::{http::StatusCode, web, Error, HttpResponse};
use diesel::{insert_into, QueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::vec::Vec;

pub async fn get_users(db: web::Data<Pool>) -> HttpResponse {
    match get_all_users(db).await {
        Ok(vec_users) => {
            let json_result = serde_json::to_string(&json!({ "User": &vec_users}));

            match json_result {
                Ok(json) => HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json),
                Err(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Error in convert JSON"),
            }
        }
        Err(error) => {
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(error.to_string())
        }
    }
}

async fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    users.load::<User>(&mut conn)
}

pub async fn get_user_by_id(db: web::Data<Pool>, user_id: web::Path<i32>) -> HttpResponse {
    match db_get_user_by_id(db, user_id.into_inner()).await {
        Ok(user_db_l) => {
            let json_result = serde_json::to_string_pretty(&json!({ "User": &user_db_l}));
            match json_result {
                Ok(json) => HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(error) => {
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(error.to_string())
        }
    }
}

async fn db_get_user_by_id(
    pool: web::Data<Pool>,
    user_id: i32,
) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&mut conn)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// Handler for POST /users
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    match add_single_user(db, item) {
        Ok(added) => {
            let json_result = serde_json::to_string(&added);
            match json_result {
                Ok(json) => Ok(HttpResponse::Created()
                    .content_type("application/json")
                    .body(json)),
                Err(_) => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

fn add_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(users).values(&new_user).get_result(&mut conn)?;
    Ok(res)
}

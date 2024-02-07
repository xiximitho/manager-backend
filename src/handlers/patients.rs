use crate::models::patients::Patient;
use crate::schema::patients::dsl::*;
use crate::Pool;
use crate::{diesel::RunQueryDsl, models::patients::NewPatient};
use actix_web::{http::StatusCode, web, Error, HttpResponse};
use diesel::insert_into;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::vec::Vec;

pub async fn get_patients(db: web::Data<Pool>) -> HttpResponse {
    match get_patients_db(db).await {
        Ok(vec_users) => {
            let json_result = serde_json::to_string(&json!({ "Patient": &vec_users}));

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

async fn get_patients_db(pool: web::Data<Pool>) -> Result<Vec<Patient>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    patients.load::<Patient>(&mut conn)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputPatient {
    pub name: String,
    pub date_of_birth: chrono::NaiveDate,
    pub contact_number: String,
    pub email: Option<String>,
    pub address: Option<String>,
    pub attendant_id: i32,
}

// Handler for POST /users
pub async fn add_patient(
    db: web::Data<Pool>,
    item: web::Json<InputPatient>,
) -> Result<HttpResponse, Error> {
    match add_single_patient(db, item) {
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

fn add_single_patient(
    db: web::Data<Pool>,
    item: web::Json<InputPatient>,
) -> Result<Patient, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let new_patient = NewPatient {
        name: &item.name,
        date_of_birth: &item.date_of_birth,
        contact_number: &item.contact_number,
        email: item.email.as_deref(),
        address: item.address.as_deref(),
        attendant_id: &item.attendant_id,
    };
    let res = insert_into(patients)
        .values(&new_patient)
        .get_result(&mut conn)?;
    Ok(res)
}

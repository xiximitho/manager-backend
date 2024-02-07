use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::patients)]
pub struct Patient {
    pub id: i32,
    pub name: String,
    pub date_of_birth: chrono::NaiveDate,
    pub contact_number: String,
    pub email: Option<String>,
    pub address: Option<String>,
    pub attendant_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::patients)]
pub struct NewPatient<'a> {
    pub name: &'a str,
    pub date_of_birth: &'a chrono::NaiveDate,
    pub contact_number: &'a str,
    pub email: Option<&'a str>,
    pub address: Option<&'a str>,
    pub attendant_id: &'a i32,
}

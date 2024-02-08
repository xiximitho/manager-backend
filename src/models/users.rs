use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Clone, Serialize, Identifiable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub unique_id: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub unique_id: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn verify(&self, password: String) -> bool {
        println!("{}", password.as_str());
        println!("hash {}", &self.password);
        verify(password.as_str(), &self.password).unwrap()
    }
}

impl NewUser {
    pub fn new(
        username: String,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
    ) -> NewUser {
        let hashed_password: String = hash(password.as_str(), DEFAULT_COST).unwrap();
        let uuid = Uuid::new_v4().to_string();
        return NewUser {
            username,
            email,
            password: hashed_password,
            unique_id: uuid,
            created_at: chrono::Local::now().naive_local(),
            first_name,
            last_name,
        };
    }
}

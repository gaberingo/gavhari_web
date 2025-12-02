use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub name: Option<String>,
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}

impl Users {
    pub fn verify(self, pass: &str) -> bool {
        let parsed = match PasswordHash::new(&self.password) {
            Ok(p) => p,
            Err(_) => return false,
        };

        // Verifikasi password
        Argon2::default()
            .verify_password(pass.as_bytes(), &parsed)
            .is_ok()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

impl CreateUser {
    pub fn new(username: String, pass: String) -> Self {
        CreateUser {
            username,
            password: pass,
            email: None,
        }
    }
}

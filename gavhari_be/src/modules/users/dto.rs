use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateUser {
    pub name: String,
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: i32,
    pub username: String,
    pub role: String,
}

use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};

use crate::{
    db::{DbPooled, Users},
    modules::users::CreateUser,
};

use super::dto::*;

pub fn user_login_verified(conn: &mut DbPooled, uname: &str, pass: &str) -> Option<Users> {
    use crate::schema::users::dsl::*;
    users
        .filter(username.eq(uname))
        .first::<Users>(conn)
        .ok()
        .filter(|u| u.verify(pass))
}

impl CreateUser {
    pub fn user_register(&self, conn: &mut DbPooled) -> Result<(), diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let mut new_user = self.clone();
        let salt = SaltString::generate(&mut OsRng);
        let hash_pass = Argon2::default()
            .hash_password(new_user.password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        new_user.password = hash_pass;
        diesel::insert_into(users)
            .values(new_user)
            .execute(conn)
            .map(|_| ())
    }
}

// User Session
impl From<&Users> for UserSession {
    fn from(value: &Users) -> Self {
        Self {
            user_id: value.id,
            username: value.username.clone(),
            role: "admin".to_string(),
        }
    }
}

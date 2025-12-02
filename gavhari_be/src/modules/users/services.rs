use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};

use crate::{
    db::{CreateUser, DbPooled, Users},
    modules::users::CreateUserForm,
};

pub fn user_login_verified(conn: &mut DbPooled, uname: &str, pass: &str) -> bool {
    use crate::schema::users::dsl::*;
    users
        .filter(username.eq(uname))
        .first::<Users>(conn)
        .ok()
        .map(|u| u.verify(pass))
        .unwrap_or(false)
}

pub fn user_registration(
    conn: &mut DbPooled,
    form: CreateUserForm,
) -> Result<Users, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let salt = SaltString::generate(&mut OsRng);
    let hash_pass = Argon2::default()
        .hash_password(form.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let new_user = CreateUser::new(form.username, hash_pass);

    diesel::insert_into(users).values(new_user).get_result(conn)
}

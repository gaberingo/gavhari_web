use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};

use crate::db::{DbPooled, models::Users};

pub fn user_login_verified(conn: &mut DbPooled, username: &str, pass: &str) -> bool {
    use crate::schema::users::dsl::*;
    users
        .filter(name.eq(username))
        .first::<Users>(conn)
        .ok()
        .map(|u| u.verify(pass))
        .unwrap_or(false)
}

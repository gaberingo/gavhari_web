use bcrypt::verify;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub password: String,
}

impl Users {
    pub fn verify(self, pass: &str) -> bool {
        verify(pass, &self.password).unwrap_or(false)
    }
}

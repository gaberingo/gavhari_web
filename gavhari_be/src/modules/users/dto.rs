use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateUserForm {
    pub name: String,
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}

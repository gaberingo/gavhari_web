use actix_session::Session;
use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, get, post, web};
use serde_json::json;

use super::{dto::*, services::user_login_verified};
use crate::{db::DbPool, modules::users::services::user_registration};

#[get("")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"user":"Gaberingo"}))
}

#[post("/login")]
async fn user_login(
    sess: Session,
    user_login: web::Form<LoginForm>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    match user_login_verified(&mut conn, &user_login.username, &user_login.password) {
        true => {
            let user_session = json!({"user_id": 32});
            sess.insert("user_session", user_session).unwrap();
            HttpResponse::SeeOther().json(json!({"message":"Test"}))
        }
        false => HttpResponse::Unauthorized().json(json!({"message":"Wrong username or password"})),
    }
}

#[post("/register")]
async fn user_register(
    sess: Session,
    register_data: web::Form<CreateUserForm>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<HttpResponse> {
    let data_form = register_data.into_inner();
    let mut conn = pool.get().unwrap();
    let result = web::block(move || user_registration(&mut conn, data_form))
        .await?
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(json!({
        "status":"success",
        "id":result.id,
        "username":result.username
    })))
}

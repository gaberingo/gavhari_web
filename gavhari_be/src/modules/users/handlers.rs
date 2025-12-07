use actix_session::Session;
use actix_web::{
    HttpResponse, Responder, error::ErrorInternalServerError, get, http::header, post, web,
};
use serde_json::json;

use super::{dto::*, services::user_login_verified};
use crate::db::DbPool;

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
    if let Some(user) = user_login_verified(&mut conn, &user_login.username, &user_login.password) {
        let user_session = UserSession::from(&user);
        if let Err(e) = sess.insert("user_session", user_session) {
            return HttpResponse::InternalServerError().body(e.to_string());
        }
        HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/"))
            .finish()
    } else {
        HttpResponse::Unauthorized().json(json!({"message":"Wrong username or password"}))
    }
}

#[post("/register")]
async fn user_register(
    sess: Session,
    register_data: web::Form<CreateUser>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<HttpResponse> {
    let mut conn = pool.get().unwrap();
    web::block(move || register_data.user_register(&mut conn))
        .await?
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(json!({
        "status":"success"
    })))
}

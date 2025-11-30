use actix_session::Session;
use actix_web::{HttpResponse, Responder, get, post, web};
use serde_json::json;

use super::{dto::*, services::user_login_verified};
use crate::{auth::SessionData, db::DbPool};

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

#[post("/create")]
async fn user_create(
    sess: Session,
    create_data: web::Form<CreateUserForm>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    HttpResponse::Ok().json(json!("test"))
}

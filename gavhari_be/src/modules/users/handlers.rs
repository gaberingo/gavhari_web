use actix_session::Session;
use actix_web::{HttpResponse, Responder, get, post, web};
use serde_json::json;

use super::dto::*;
use crate::auth::SessionData;

#[get("")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"user":"Gaberingo"}))
}

#[post("/login")]
async fn user_login(sess: Session, user_login: web::Form<LoginForm>) -> impl Responder {
    let user_sess = 32;
    dbg!(user_login);
    let session = sess.get::<SessionData>("session_data").ok().flatten();
    match session {
        Some(sess) => {
            println!("{:?}", json!(sess));
        }
        None => {
            println!("SESSION NOT FOUND");
        }
    }
    sess.insert("user_session", user_sess).unwrap();
    let entries = sess.entries();
    dbg!(entries);
    HttpResponse::Ok().json(json!({"message":"Login sukses"}))
}

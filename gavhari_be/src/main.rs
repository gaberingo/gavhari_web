mod auth;
mod db;
mod modules;
mod schema;

use std::rc::Rc;

use actix_session::Session;
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::cookie::SameSite;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, cookie::Key, get, middleware::Logger, web,
};
use db::establish_connection;
use dotenvy::dotenv;
use env_logger::Env;
use serde_json::json;

#[get("/")]
async fn index(sess: Session) -> impl Responder {
    let sess_data = sess.get::<auth::UserSession>("user_session").ok().flatten();
    match sess_data {
        Some(s) => HttpResponse::Ok().json(json!(s)),
        None => HttpResponse::Ok().json(json!({"msg":"Session not found"})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::new().default_filter_or("debug"));

    let secret_key = Key::generate();

    let db_pool = establish_connection();
    let redis_store = RedisSessionStore::new("redis://:gavharipass@127.0.0.1:6379")
        .await
        .expect("Cannot connect redis");

    HttpServer::new(move || {
        App::new()
            // Logger
            .wrap(Logger::default())
            // Pastikan Database Terhubung
            .wrap(db::DbCheck {
                pool: Rc::new(db_pool.clone()),
            })
            // Pastikan session ada
            .wrap(auth::SessionGuard)
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_secure(false)
                    .cookie_name(String::from("user_session"))
                    .cookie_http_only(true) // Security improvement
                    .cookie_same_site(actix_web::cookie::SameSite::Lax) // Security improvement
                    .build(),
            )
            .app_data(web::Data::new(db_pool.clone())) // PERBAIKAN: gunakan web::Data::new
            .service(index)
            .configure(modules::module_conf)
            .default_service(web::to(HttpResponse::NotFound))
    })
    .bind(("127.0.0.1", 8000))?
    .workers(4)
    .run()
    .await
}

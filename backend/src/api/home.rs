use actix_web::{Responder, get, HttpResponse};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Gavhari Web API!")
}
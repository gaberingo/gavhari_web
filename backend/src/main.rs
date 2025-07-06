mod api;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(api::home::home)
    })
    .bind(("127.0.0.1", 8111))?
    .run()
    .await
}
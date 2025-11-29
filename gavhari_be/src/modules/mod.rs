mod users;
use actix_web::web::{self, ServiceConfig};

pub fn module_conf(app: &mut ServiceConfig) {
    app.service(web::scope("/api/v1").configure(users::users_factory));
}

mod dto;
mod handlers;
mod services;

use actix_web::web::{self, ServiceConfig};
use handlers::*;

pub use dto::*;

pub fn users_factory(app: &mut ServiceConfig) {
    app.service(
        web::scope("/users")
            .service(get_users)
            .service(user_register),
    );
    app.service(user_login);
}

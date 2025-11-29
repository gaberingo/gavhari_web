use std::{
    future::{Ready, ready},
    rc::Rc,
};

use actix_web::{
    Error, HttpResponse,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;
use serde_json::json;

use crate::db::DbPool;

pub struct DbCheck {
    pub pool: Rc<DbPool>,
}

pub struct DbCheckMiddleware<S> {
    service: S,
    pool: Rc<DbPool>,
}

impl<S, B> Transform<S, ServiceRequest> for DbCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = DbCheckMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(DbCheckMiddleware {
            service,
            pool: self.pool.clone(),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for DbCheckMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let pool = self.pool.clone();
        let fut = self.service.call(req);

        Box::pin(async move {
            match pool.get() {
                Ok(_) => {
                    println!("DB Check Success !!");
                    let res = fut.await?;
                    Ok(res.map_into_left_body())
                }
                Err(e) => {
                    println!("DB Check Failed: {}", e);

                    let error_body = HttpResponse::ServiceUnavailable().json(json!({
                        "error": "Database connection error",
                        "message": e.to_string()
                    }));

                    let (req2, _) = fut.await?.into_parts();

                    let res = ServiceResponse::new(req2, error_body.map_into_right_body());
                    Ok(res)
                }
            }
        })
    }
}

use actix_session::Session;
use actix_web::{
    Error, FromRequest,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use chrono::Utc;
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};

use crate::auth::dto::UserSession;

pub struct SessionGuard;
pub struct SessionGuardMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for SessionGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SessionGuardMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SessionGuardMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for SessionGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = Session::extract(req.request()).into_inner().ok();
        if let Some(sess) = session {
            match sess.get::<UserSession>("user_session") {
                Ok(Some(mut sess_data)) => {
                    dbg!("Session Ditemukan : {}", &sess_data);
                    sess_data.update_last_access();

                    if let Err(e) = sess.insert("user_session", &sess_data) {
                        println!("Error Updating session: {}", e);
                    } else {
                        println!("Session Updating - Visit Count : {}", sess_data.visit_count);
                    }
                }
                Ok(None) => {
                    println!("Creating Session !");
                    let new_sess = UserSession::new();
                    if let Err(e) = sess.insert("user_session", &new_sess) {
                        println!("Error Create Session: {}", e);
                    } else {
                        println!("New session created");
                        dbg!(new_sess);
                    }
                }
                Err(e) => {
                    println!("Error reading session : {:?}", e);
                }
            }
        }
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

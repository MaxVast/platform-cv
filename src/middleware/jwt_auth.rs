/*use actix_http::HttpMessage;
use actix_web::{dev::{ServiceRequest, Service, ServiceResponse, Transform}, Error, web};
use actix_web::http::header::COOKIE;
use actix_service::forward_ready;
use futures::future::{ok, LocalBoxFuture, Ready};
use futures::FutureExt;

use crate::utils::token_utils::{decode_token, verify_token};
use crate::config::db::Pool;

pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware { service })
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let pool = req.app_data::<web::Data<Pool>>().cloned();
        let headers = req.headers().clone();

        // Clone the request parts (excluding the payload)
        let req_parts = req.into_parts();
        let (head, payload) = req_parts;

        // Reconstruct the ServiceRequest with cloned headers and payload
        let req = ServiceRequest::from_parts(head, payload);

        // Call the service
        let srv = self.service.call(req);

        Box::pin(async move {
            let response = srv.await?;

            if let Some(pool) = pool {
                if let Some(cookie) = headers.get(COOKIE) { // Use cloned headers
                    if let Ok(cookie_str) = cookie.to_str() {
                        if let Some(token) = cookie_str.split("; ").find_map(|s| {
                            if s.starts_with("token=") {
                                Some(s.trim_start_matches("token="))
                            } else {
                                None
                            }
                        }) {
                            if let Ok(token_data) = decode_token(token.to_string()) {
                                if let Ok(_username) = verify_token(&token_data, &pool) {
                                    req.extensions_mut().insert(token_data);
                                }
                            }
                        }
                    }
                }
            }
            Ok(response.map_into_left_body())
        })
    }
}
*/
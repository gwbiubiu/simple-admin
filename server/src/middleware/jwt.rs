use crate::models::Claims;
use crate::pkg::dictionary::AUTH_TOKEN;
use crate::AppState;
use actix_web::web;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures::{
    future::{ready, Ready},
    FutureExt, TryFutureExt,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::future::Future;
use std::pin::Pin;

pub struct JwtMiddleware;


impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService {
            service,
            write_list_paths: vec!["/api/v1/login"],
        }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
    write_list_paths: Vec<&'static str>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if self
            .write_list_paths
            .iter()
            .any(|path| req.path().starts_with(path))
        {
            return self
                .service
                .call(req)
                .map_ok(ServiceResponse::map_into_left_body)
                .boxed_local();
        }
        let token = req
            .cookie(AUTH_TOKEN)
            .map(|cookie| cookie.value().to_string());
        if let Some(token_str) = token {
            let app_date = req.app_data::<web::Data<AppState>>().unwrap();
            let secret = app_date.config.jwt.signing_key.as_bytes();
            let is_black_token = app_date.redis_adaptor.is_token_in_black_list(token_str.clone()).unwrap_or(true);
            if is_black_token {
                return Box::pin(async {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized().finish().map_into_right_body(),
                    ))
                });
            }

            match decode::<Claims>(
                token_str.as_str(),
                &DecodingKey::from_secret(secret),
                &Validation::default(),
            ) {
                Ok(token_data) => {
                    req.extensions_mut().insert(token_data.claims);
                    return self
                        .service
                        .call(req)
                        .map_ok(ServiceResponse::map_into_left_body)
                        .boxed_local();
                }
                Err(_) => {
                    return Box::pin(async move {
                        Ok(req.into_response(
                            HttpResponse::Unauthorized().finish().map_into_right_body(),
                        ))
                    });
                }
            }
        }
        return Box::pin(async {
            Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
        });
    }
}

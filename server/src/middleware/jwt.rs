use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error, HttpMessage,
};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use crate::{AppState, RedisAdaptor};
use std::sync::Arc;
use crate::pkg::dictionary::AUTH_TOKEN;
use actix_web::web;


pub struct JwtMiddleware{
    redis_adaptor: Arc<RedisAdaptor>,
}

impl  JwtMiddleware {
    pub fn new(redis_adaptor: Arc<RedisAdaptor>) -> Self {
        Self {
            redis_adaptor
        }
    }
    
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService { service, write_list_paths: vec!["/api/v1/login"], redis_adaptor :  self.redis_adaptor.clone() }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
    write_list_paths: Vec<&'static str>,
    redis_adaptor: Arc<RedisAdaptor>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if self.write_list_paths.iter().any(|path| req.path().starts_with(path)) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }
        let token = req.cookie(AUTH_TOKEN).map(|cookie| cookie.value().to_string());
        if let Some(token_str) = token {
            let app_date  =  req.app_data::<web::Data<AppState>>().unwrap();
            let secret = app_date.config.jwt.signing_key.as_bytes();
            match decode::<Claims>(
                token_str.as_str(),
                &DecodingKey::from_secret(secret),
                &Validation::default(),
            ) {
                Ok(token_data) => {
                    let token = token_str.clone();
                    req.extensions_mut().insert(token_data.claims);
                    let redis_adaptor = self.redis_adaptor.clone();
                    let fut: <S as Service<ServiceRequest>>::Future = self.service.call(req);
                    return Box::pin(async move {
                        let is_in_black =  redis_adaptor.is_token_in_black_list(token).await?;
                        if is_in_black {
                            return Err(ErrorUnauthorized("Token is in black list"));
                        }
                        let res = fut.await?;
                        Ok(res)
                    });
                }
                Err(_) => {
                    return Box::pin(async move {
                        Err(ErrorUnauthorized("Invalid token"))
                    });
                }
            }
        }
        Box::pin(async move { Err(ErrorUnauthorized("No token found")) })
    }
}
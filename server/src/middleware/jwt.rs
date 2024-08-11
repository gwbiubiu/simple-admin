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

const AUTH_HEADER: &str = "Authorization";
const BEARER_PREFIX: &str = "Bearer ";

pub struct JwtMiddleware;

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
        ready(Ok(JwtMiddlewareService { service }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
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
        let write_list_paths = vec!["/api/v1/login", "/api/v1/user"];
        if write_list_paths.iter().any(|path| req.path().starts_with(path)) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }
        let token = req
            .headers()
            .get(AUTH_HEADER)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix(BEARER_PREFIX));

        if let Some(token) = token {
            let secret = b"your_secret_key"; // 在实际应用中，应从配置或环境变量中获取
            match decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret),
                &Validation::default(),
            ) {
                Ok(token_data) => {
                    req.extensions_mut().insert(token_data.claims);
                    let fut = self.service.call(req);
                    return Box::pin(async move {
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
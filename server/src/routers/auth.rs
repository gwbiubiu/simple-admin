use actix_web::{Error,  HttpResponse, post, web,cookie::Cookie};
use crate::{AppState, models, Response, Status};
use crate::services::Auth;

pub fn auth_router(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}


#[post("/login")]
async fn login(data: web::Data<AppState>, login: web::Json<models::Login>) -> anyhow::Result<HttpResponse, Error> {
    let login = login.into_inner();
    let jwt = &data.config.jwt;
    let token = Auth::login(data.clone(), login).await?;
    let cookie = Cookie::build("auth", token.token.clone())
                .secure(jwt.secure)
                .http_only(jwt.http_only)
                .finish();
    let resp = Response {
        status: Status::SUCCESS,
        code: 200,
        message: "".to_string(),
        data: Some(token),
    };
    Ok(actix_web::HttpResponse::Ok().cookie(cookie).json(resp))
}
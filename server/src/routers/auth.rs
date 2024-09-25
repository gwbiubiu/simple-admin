use actix_web::{Error,  HttpResponse, post, get, web, HttpRequest};
use crate::AppState;
use crate::services::Auth;
use crate::models::{self, success_json, success_none};

pub fn auth_router(cfg: &mut web::ServiceConfig) {
    cfg
    .service(login)
    .service(logout);

}


#[post("/login")]
async fn login(data: web::Data<AppState>, login: web::Json<models::Login>) -> anyhow::Result<HttpResponse, Error> {
    let login = login.into_inner();
    let token = Auth::login(data.clone(), login).await?;
    Ok(success_json(token))
}

#[get("/logout")]
async fn logout(data: web::Data<AppState>, req: HttpRequest) -> anyhow::Result<HttpResponse, Error> {
    Auth::logout(data.clone(),req).await?;
    Ok(success_none())
}
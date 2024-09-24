use actix_web::{Error,  HttpResponse, post, web};
use crate::AppState;
use crate::services::Auth;
use crate::models::{self, success_json};

pub fn auth_router(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}


#[post("/login")]
async fn login(data: web::Data<AppState>, login: web::Json<models::Login>) -> anyhow::Result<HttpResponse, Error> {
    let login = login.into_inner();
    let token = Auth::login(data.clone(), login).await?;
    Ok(success_json(token))
}
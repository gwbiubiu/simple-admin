use actix_web::{Error,  HttpResponse, post, web};
use crate::{AppState,  models, success_json};
use crate::services::Auth;

pub fn auth_router(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}


#[post("/login")]
async fn login(data: web::Data<AppState>, login: web::Json<models::Login>) -> anyhow::Result<HttpResponse, Error> {
    let conn = &data.conn;
    let login = login.into_inner();
    let token = Auth::login(conn, login).await?;
    Ok(success_json(&token))
}
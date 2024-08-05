use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web, Error, ResponseError, put};
use crate::global::AppState;
use crate::{models, success_json};
use crate::services::User;
use anyhow::Result;
use crate::errors::AppError;

pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("user")
            //.route("", web::post().to(create_user))
            .service(get_user_by_id)
            .service(create_user)
            .service(update_user_status)
    );
}


// async fn create_user(web::Json(user): web::Json<CreateUser>) -> actix_web::Result<impl Responder> {
//     Ok(web::Json(user))
// }
//
#[get("/{id}")]
async fn get_user_by_id(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ret = User::find_user_by_id(conn, id.into_inner()).await.map_err(|e| AppError::SystemError("find user failed".to_string()))?;
    return Ok(HttpResponse::Ok().json(ret));
}

#[post("")]
async fn create_user(data: web::Data<AppState>, user: web::Json<models::CreateUser>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let user = user.into_inner();
    let id = User::create_user(conn, user).await?;
    Ok(success_json(id))
}

#[put("/{id}/status")]
async fn update_user_status(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ret = User::update_user_status(conn, id.into_inner()).await?;
    Ok(success_json(ret))
}

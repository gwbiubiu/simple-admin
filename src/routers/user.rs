use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web, Error, ResponseError};
use crate::global::AppState;
use crate::{models};
use crate::services::User;
use anyhow::Result;

pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("user")
            //.route("", web::post().to(create_user))
            .service(get_user_by_id)
            .service(create_user)
    );
}


// async fn create_user(web::Json(user): web::Json<CreateUser>) -> actix_web::Result<impl Responder> {
//     Ok(web::Json(user))
// }
//
#[get("/{id}")]
async fn get_user_by_id(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    format!("user id is {}", id)
}

#[post("")]
async fn create_user(data: web::Data<AppState>, user: web::Json<models::CreateUser>) -> HttpResponse {
    let conn = &data.conn;
    let user = user.into_inner();
    let ret = User::create_user(conn, user).await.unwrap();
    HttpResponse::Ok().json(ret)
}

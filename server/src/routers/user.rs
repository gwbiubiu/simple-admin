use actix_web::{get, HttpResponse, post, web, Error, put, delete};
use crate::global::AppState;
use crate::services::User;
use anyhow::Result;
use crate::errors::AppError;
use crate::models::{self, Page, PageResponse, success_json};

pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("user")
            .service(get_user_by_id)
            .service(create_user)
            .service(update_user_status)
            .service(get_user_list)
            .service(add_user_roles)
            .service(delete_user_role)
    );
}


// async fn create_user(web::Json(user): web::Json<CreateUser>) -> actix_web::Result<impl Responder> {
//     Ok(web::Json(user))
// }
//
#[get("/{id:\\d+}")]
async fn get_user_by_id(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let user_info = User::find_user_by_id(conn, id.into_inner()).await?;
    return Ok(success_json(user_info));
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

#[get("/list")]
async fn get_user_list(data: web::Data<AppState>, query: web::Query<models::QueryUsers>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let query_inner = query.into_inner();
    let (users, total) = User::get_user_list(conn, query_inner.clone()).await.map_err(|e| AppError::SystemError(e.to_string()))?;
    let page = Page::new(query_inner.page.page, query_inner.page.size, total);
    Ok(success_json(PageResponse::new(page, users)))
}

#[post("/{id}/roles")]
async fn add_user_roles(data: web::Data<AppState>, id: web::Path<i32>, roles: web::Json<Vec<i32>>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let user_role = models::AddUserRole {
        user_id: id.into_inner(),
        role_id: roles.into_inner(),
    };
    let ret = User::add_user_roles(conn, user_role).await?;
    Ok(success_json(ret))
}

#[delete("/{user_id:\\d+}/role/{role_id:\\d+}")]
async fn delete_user_role(data: web::Data<AppState>, path: web::Path<(i32, i32)>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let (user_id, role_id) = path.into_inner();
    let user_role = models::DeleteUserRole {
        user_id: user_id,
        role_id: role_id,
    };
    let ret = User::delete_user_role(conn, user_role).await?;
    Ok(success_json(ret))
}
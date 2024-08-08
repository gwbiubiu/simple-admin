use actix_web::{delete, Error, get, HttpResponse, post, put, web};
use anyhow::Result;
use crate::{AppState, models, services, success_json};
use crate::models::role;

pub fn role_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/role")
            .service(role_list)
            .service(create_role)
            .service(update_role)
            .service(delete_role)
            .service(get_role_by_id)
    );
}


#[get("/list")]
async fn role_list(data: web::Data<AppState>, query: web::Query<role::QueryRole>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let query = query.into_inner();
    let id = 1;
    Ok(success_json(id))
}

#[post("")]
async fn create_role(data: web::Data<AppState>, role: web::Json<role::CreateRole>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let role = role.into_inner();
    let id = role::Role::create_role(db, role).await?;
    Ok(success_json(id))
}

#[get("/{id:\\d+}")]
async fn get_role_by_id(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let id = 1;
    Ok(success_json(id))
}


#[put("/{id}")]
async fn update_role(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let id = 1;
    Ok(success_json(id))
}

#[delete("/{id}")]
async fn delete_role(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let id = 1;
    Ok(success_json(id))
}

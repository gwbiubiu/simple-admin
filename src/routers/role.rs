use actix_web::{delete, Error, get, HttpResponse, post, put, web};
use anyhow::Result;
use crate::{AppState, services, success_json};
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
    let roles = services::role::Role::list_role(db, query.clone()).await?;
    Ok(success_json((roles.0, query.page.new_with_total(roles.1))))
}

#[post("")]
async fn create_role(data: web::Data<AppState>, role: web::Json<role::CreateRole>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let role = role.into_inner();
    let id = services::role::Role::create_role(db, role).await?;
    Ok(success_json(id))
}

#[get("/{id:\\d+}")]
async fn get_role_by_id(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let role = services::role::Role::get_role_by_id(db, id.into_inner()).await?;
    Ok(success_json(role))
}


#[put("/{id}")]
async fn update_role(data: web::Data<AppState>, id: web::Path<i32>, role: web::Json<role::UpdateRole>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let resp = services::role::Role::update_role(db, id.into_inner(), role.into_inner()).await?;
    Ok(success_json(resp))
}

#[delete("/{id}")]
async fn delete_role(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let resp = services::role::Role::delete_role(db, id.into_inner()).await?;
    Ok(success_json(resp))
}


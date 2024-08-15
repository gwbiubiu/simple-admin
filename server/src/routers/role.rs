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
            .service(add_role_apis)
    );
}


#[get("/list")]
async fn role_list(data: web::Data<AppState>, query: web::Query<role::QueryRole>) -> Result<HttpResponse, Error> {
    let query = query.into_inner();
    let roles = services::role::Role::list_role(&data.conn, query.clone()).await?;
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
    let resp = services::role::Role::delete_role(&data.conn, id.into_inner()).await?;
    Ok(success_json(resp))
}


#[post("/{id}/api_router")]
async fn add_role_apis(data: web::Data<AppState>, role_id: web::Path<i32>, api_ids: web::Json<Vec<i32>>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let role_api = role::AddRoleApi {
        role_id: role_id.into_inner(),
        api_ids: api_ids.into_inner(),
    };
    let resp = services::role::Role::role_add_apis(db, role_api).await?;
    Ok(success_json(resp))
}
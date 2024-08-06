use actix_web::{delete, Error, get, HttpResponse, post, put, web};
use anyhow::Result;
use crate::{AppState, models, services, success_json};

pub fn api_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api_router")
            .service(api_list)
            .service(create_api)
            .service(update_api)
            .service(delete_api)
    );
}


#[get("/list")]
async fn api_list() -> Result<HttpResponse, Error> {
    let id = 1;
    Ok(success_json(id))
}

#[post("")]
async fn create_api(data: web::Data<AppState>, api: web::Json<models::CreateApi>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let api = api.into_inner();
    let id = services::Api::create_api(db, api).await?;
    Ok(success_json(id))
}

#[get("/{id:\\+d}")]
async fn get_api_by_id() -> Result<HttpResponse, Error> {
    let id = 1;
    Ok(success_json(id))
}


#[put("/{id}")]
async fn update_api() -> Result<HttpResponse, Error> {
    let id = 1;
    Ok(success_json(id))
}

#[delete("/{id}")]
async fn delete_api() -> Result<HttpResponse, Error> {
    let id = 1;
    Ok(success_json(id))
}

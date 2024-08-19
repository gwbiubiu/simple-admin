use actix_web::{delete, Error, get, HttpResponse, post, put, web};
use anyhow::Result;
use crate::{AppState, models, services, success_json};
use crate::models::{Page, PageResponse};

pub fn api_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api_router")
            .service(api_list)
            .service(create_api)
            .service(update_api)
            .service(delete_api)
            .service(get_api_by_id)
    );
}


#[get("/list")]
async fn api_list(data: web::Data<AppState>, query: web::Query<models::QueryApiList>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let query_inner = query.into_inner();
    let (apis, total) = services::Api::list(db, query_inner.clone()).await?;
    let page = Page::new(query_inner.page.page, query_inner.page.size, total);
    Ok(success_json(PageResponse::new(page, apis)))
}

#[post("")]
async fn create_api(data: web::Data<AppState>, api: web::Json<models::CreateApi>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let api = api.into_inner();
    let id = services::Api::create_api(db, api).await?;
    Ok(success_json(id))
}

#[get("/{id:\\d+}")]
async fn get_api_by_id(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let api = services::Api::get_api_by_id(db, id.into_inner()).await?;
    Ok(success_json(api))
}


#[put("/{id}")]
async fn update_api(data: web::Data<AppState>, id: web::Path<i32>, api: web::Json<models::UpdateApi>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let resp = services::Api::update_api(db, id.into_inner(), api.into_inner()).await?;
    Ok(success_json(resp))
}

#[delete("/{id}")]
async fn delete_api(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let db = &data.conn;
    let resp = services::Api::delete_api_by_id(db, id.into_inner()).await?;
    Ok(success_json(resp))
}

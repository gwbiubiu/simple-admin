use sea_orm::DbConn;
use crate::errors::AppError;
use crate::{models, models::api};
use crate::models::ApiCategory;

pub struct Api;


impl Api {
    pub async fn create_api(db: &DbConn, api: models::CreateApi) -> Result<i32, AppError> {
        let id = api::Api::create_api(db, api).await?;
        Ok(id)
    }

    pub async fn get_api_by_id(db: &DbConn, id: i32) -> Result<models::Api, AppError> {
        let api = api::Api::get_api_by_id(db, id).await?;
        Ok(api)
    }

    pub async fn update_api(db: &DbConn, id: i32, api: models::UpdateApi) -> Result<bool, AppError> {
        let ret = api::Api::update_api(db, id, api).await?;
        Ok(ret)
    }


    pub async fn delete_api_by_id(db: &DbConn, id: i32) -> Result<bool, AppError> {
        let ret = api::Api::delete_api_by_id(db, id).await?;
        Ok(ret)
    }

    pub async fn list(db: &DbConn, query: models::QueryApiList) -> Result<(Vec<ApiCategory>, u64), AppError> {
        let resp = api::Api::get_api_list(db, query).await?;
        Ok(resp)
    }
}
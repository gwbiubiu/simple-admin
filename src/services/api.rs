use sea_orm::DbConn;
use crate::errors::AppError;
use crate::{models, models::api};


pub struct Api;


impl Api {
    pub async fn create_api(db: &DbConn, api: models::CreateApi) -> Result<i32, AppError> {
        let id = api::Api::create_api(db, api).await?;
        Ok(id)
    }
}
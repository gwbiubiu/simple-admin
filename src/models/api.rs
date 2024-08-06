use sea_orm::*;
use anyhow::Result;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use entities::api;
use crate::entities;
use crate::errors::{api::ApiError::ApiAlreadyExists, AppError, AppError::ApiError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApi {
    pub name: String,
    pub path: String,
    pub api_group: String,
    pub method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Api {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub api_group: String,
    pub method: String,
}


impl Api {
    pub async fn create_api(db: &DbConn, api: CreateApi) -> Result<i32, AppError> {
        let count = api::Entity::find()
            .filter(api::Column::Path.eq(&api.path))
            .filter(api::Column::Method.eq(&api.method))
            .count(db)
            .await?;
        if count > 0 {
            return Err(ApiError(ApiAlreadyExists));
        }
        let api = api::ActiveModel {
            name: Set(api.name.clone()),
            path: Set(api.path.clone()),
            api_group: Set(api.api_group.clone()),
            method: Set(api.method.clone()),
            ..Default::default()
        };
        let resp = api::Entity::insert(api)
            .exec(db)
            .await?;
        Ok(resp.last_insert_id)
    }
}
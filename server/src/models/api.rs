use sea_orm::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use entities::api;
use crate::entities;
use crate::errors::{AppError, AppError::ApiError};
use crate::errors::api::ApiError::{NotFound, ApiAlreadyExists};
use crate::models::Page;
use chrono::{DateTime, Utc};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApi {
    pub name: String,
    pub path: String,
    pub api_group: String,
    pub method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateApi {
    pub name: Option<String>,
    pub path: Option<String>,
    pub api_group: Option<String>,
    pub method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryApiList {
    pub name: Option<String>,
    pub api_group: Option<String>,
    #[serde(flatten)]
    pub page: Page,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Api {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub api_group: String,
    pub method: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,

}

impl From<api::Model> for Api {
    fn from(api: api::Model) -> Self {
        Self {
            id: api.id,
            name: api.name,
            path: api.path,
            api_group: api.api_group,
            method: api.method,
            create_time: api.create_time,
            update_time: api.update_time,
        }
    }
}


impl Api {
    pub async fn create_api(db: &DbConn, api: CreateApi) -> Result<i32, AppError> {
        let api = api::ActiveModel {
            name: Set(api.name.clone()),
            path: Set(api.path.clone()),
            api_group: Set(api.api_group.clone()),
            method: Set(api.method.clone()),
            ..Default::default()
        };
        let resp = api::Entity::insert(api)
            .exec(db)
            .await.
            map_err(|e| if e.to_string().contains("Duplicate entry") {
                ApiError(ApiAlreadyExists)
            } else {
                e.into()
            })?;
        Ok(resp.last_insert_id)
    }

    pub async fn get_api_by_id(db: &DbConn, id: i32) -> Result<Self, AppError> {
        let api = api::Entity::find_by_id(id).one(db).await?;
        match api {
            Some(api) => Ok(api.into()),
            None => return Err(ApiError(NotFound)),
        }
    }


    pub async fn update_api(db: &DbConn, id: i32, api: UpdateApi) -> Result<bool, AppError> {
        let old_api = api::Entity::find_by_id(id).one(db).await?;
        match old_api {
            Some(old_api) => {
                let mut update_api: api::ActiveModel = old_api.into();
                if let Some(name) = api.name {
                    update_api.name = Set(name);
                }
                if let Some(path) = api.path {
                    update_api.path = Set(path);
                }
                if let Some(api_group) = api.api_group {
                    update_api.api_group = Set(api_group);
                }
                if let Some(method) = api.method {
                    update_api.method = Set(method);
                }
                update_api.update(db).await.map_err(|e| if e.to_string().contains("duplicate key") {
                    ApiError(ApiAlreadyExists)
                } else {
                    e.into()
                })?;
                Ok(true)
            }
            None => return Err(ApiError(NotFound)),
        }
    }

    pub async fn delete_api_by_id(db: &DbConn, id: i32) -> Result<bool, AppError> {
        let delete_result = api::Entity::delete_by_id(id)
            .exec(db)
            .await?;
        Ok(delete_result.rows_affected > 0)
    }


    pub async fn get_api_list(db: &DbConn, query: QueryApiList) -> Result<(Vec<Self>, u64), AppError> {
        let mut query_builder = api::Entity::find();
        if let Some(name) = query.name {
            query_builder = query_builder.filter(api::Column::Name.contains(name));
        }
        if let Some(api_group) = query.api_group {
            query_builder = query_builder.filter(api::Column::ApiGroup.contains(api_group));
        }
        let total = query_builder.clone().count(db).await?;
        let api_list = query_builder
            .limit(query.page.size)
            .offset(query.page.page * query.page.size)
            .all(db)
            .await?.into_iter().map(|a| {
            a.into()
        }).collect::<Vec<Self>>();

        Ok((api_list, total))
    }
    
    pub async fn get_all_apis(db: &DbConn) -> Result<Vec<Api>, AppError> {
        let apis = api::Entity::find().all(db).await?;
        Ok(apis.into_iter().map(|a| a.into()).collect())
    }
}
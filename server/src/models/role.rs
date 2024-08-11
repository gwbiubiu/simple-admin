use sea_orm::*;
use anyhow::Result;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use crate::entities::{role, role_apis, user_role};
use crate::errors::{AppError, AppError::RoleError, roles::RoleError::RoleNotFound, roles::RoleError::RoleHasExists};
use crate::errors::roles::RoleError::CurrentRoleHasUser;
use crate::models::Page;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

impl From<role::Model> for Role {
    fn from(role: role::Model) -> Self {
        Self {
            id: role.id,
            name: role.name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRole {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRole {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRole {
    pub name: Option<String>,
    #[serde(flatten)]
    pub page: Page,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRoleApi {
    #[serde(default)]
    pub role_id: i32,
    pub api_ids: Vec<i32>,
}


impl Role {
    pub async fn create_role(db: &DbConn, role: CreateRole) -> Result<i32, AppError> {
        let role = role::ActiveModel {
            name: Set(role.name),
            ..Default::default()
        };
        let resp = role::Entity::insert(role)
            .exec(db)
            .await.
            map_err(|e| if e.to_string().contains("Duplicate entry") {
                RoleError(RoleHasExists)
            } else {
                e.into()
            })?;
        Ok(resp.last_insert_id)
    }
    pub async fn get_role_by_id(db: &DbConn, id: i32) -> Result<Self, AppError> {
        let role = role::Entity::find_by_id(id).one(db).await?;
        if let Some(role) = role {
            return Ok(role.into());
        }
        Err(RoleError(RoleNotFound))
    }
    pub async fn update_role(db: &DbConn, id: i32, role: UpdateRole) -> Result<bool, AppError> {
        let old_role = role::Entity::find_by_id(id).one(db).await?;
        if let Some(old_role) = old_role {
            if old_role.name == role.name {
                return Ok(false);
            }
            let mut new_role: role::ActiveModel = old_role.into();
            new_role.name = Set(role.name);
            role::Entity::update(new_role)
                .exec(db)
                .await
                .map_err(|e| if e.to_string().contains("Duplicate entry") {
                    RoleError(RoleHasExists)
                } else {
                    e.into()
                })?;
            return Ok(true);
        }
        Err(RoleError(RoleNotFound))
    }

    pub async fn delete_role(db: &DbConn, id: i32) -> Result<bool, AppError> {
        let count = user_role::Entity::find().
            filter(user_role::Column::RoleId.eq(id)).count(db).await?;
        if count > 0 {
            return Err(RoleError(CurrentRoleHasUser));
        }
        let resp = role::Entity::delete_by_id(id).exec(db).await?;
        return Ok(resp.rows_affected > 0);
    }


    pub async fn role_list(db: &DbConn, query: QueryRole) -> Result<(Vec<Self>, u64), AppError> {
        let mut query_builder = role::Entity::find();
        if let Some(name) = query.name {
            query_builder = query_builder.filter(role::Column::Name.contains(name));
        }

        let count = query_builder.clone().count(db).await?;
        let roles = query_builder
            .limit(query.page.size)
            .offset(query.page.page * query.page.size)
            .all(db)
            .await?;
        let roles = roles.into_iter().map(|role| Self {
            id: role.id,
            name: role.name,
        }).collect();
        Ok((roles, count))
    }


    pub async fn delete_role_api_by_role_id<T>(db: &T, role_id: i32) -> Result<bool, DbErr>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let resp = role_apis::Entity::delete_many()
            .filter(role_apis::Column::RoleId.eq(role_id))
            .exec(db)
            .await?;
        Ok(resp.rows_affected > 0)
    }
    pub async fn role_add_apis<T>(db: &T, role_api: AddRoleApi) -> Result<bool, DbErr>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let mut role_apis = vec![];
        for api_id in role_api.api_ids {
            role_apis.push(role_apis::ActiveModel {
                role_id: Set(role_api.role_id),
                api_id: Set(api_id),
                ..Default::default()
            });
        }
        let resp = role_apis::Entity::insert_many(role_apis).on_conflict(
            sea_query::OnConflict::columns([role_apis::Column::RoleId, role_apis::Column::ApiId])
                .update_column(role_apis::Column::RoleId)
                .to_owned())
            .exec(db).await;
        match resp {
            Ok(_) => Ok(true),
            Err(e) => match e {
                DbErr::RecordNotInserted => Ok(false),
                _ => Err(e.into()),
            },
        }
    }
}
use sea_orm::*;
use anyhow::Result;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use crate::entities;
use crate::entities::role;
use crate::errors::{AppError, AppError::RoleError, roles::RoleError::RoleNotFound, roles::RoleError::RoleHasExists};
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
    #[serde(flatten)]
    pub page: Page,
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

    pub async fn get_role_list(db: &DbConn) -> Result<Vec<Self>> {
        let roles = role::Entity::find().all(db).await?;
        Ok(roles.into_iter().map(|role| Self {
            id: role.id,
            name: role.name,
        }).collect())
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
}
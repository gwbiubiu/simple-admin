use sea_orm::*;
use anyhow::Result;
use crate::entities::role;
use crate::errors::{AppError, roles::RoleError::RoleNotFound};

pub struct Role {
    pub id: i32,
    pub name: String,
}

impl Role {
    pub async fn create_role(db: &DbConn, name: String) -> Result<i32> {
        let role = role::ActiveModel {
            name: Set(name),
            ..Default::default()
        };
        let resp = role::Entity::insert(role)
            .exec(db)
            .await?;
        Ok(resp.last_insert_id)
    }
    pub async fn get_role_by_id(db: &DbConn, id: i32) -> Result<Self> {
        let role = role::Entity::find_by_id(id).one(db).await?;
        let role = role.ok_or(AppError::RoleError(RoleNotFound))?;
        Ok(Self {
            id: role.id,
            name: role.name,
        })
    }

    pub async fn get_role_list(db: &DbConn) -> Result<Vec<Self>> {
        let roles = role::Entity::find().all(db).await?;
        Ok(roles.into_iter().map(|role| Self {
            id: role.id,
            name: role.name,
        }).collect())
    }

    pub async fn update_role_name(db: &DbConn, id: i32, name: String) -> Result<bool> {
        let role = role::ActiveModel {
            name: Set(name),
            ..Default::default()
        };
        let resp = role::Entity::update(role)
            .filter(role::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(true)
    }
}
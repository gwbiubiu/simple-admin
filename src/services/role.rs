use sea_orm::*;
use crate::errors::AppError;
use crate::models::role;


pub struct Role;

impl Role {
    pub async fn create_role(db: &DbConn, role: role::CreateRole) -> Result<i32, AppError> {
        let id = role::Role::create_role(db, role).await?;
        Ok(id)
    }

    pub async fn get_role_by_id(db: &DbConn, id: i32) -> Result<role::Role, AppError> {
        let role = role::Role::get_role_by_id(db, id).await?;
        Ok(role)
    }

    pub async fn update_role(db: &DbConn, id: i32, role: role::UpdateRole) -> Result<bool, AppError> {
        let resp = role::Role::update_role(db, id, role).await?;
        Ok(resp)
    }


    pub async fn delete_role(db: &DbConn, id: i32) -> Result<bool, AppError> {
        let resp = role::Role::delete_role(db, id).await?;
        Ok(resp)
    }

    pub async fn list_role(db: &DbConn, query: role::QueryRole) -> Result<(Vec<role::Role>, u64), AppError> {
        let resp = role::Role::role_list(db, query).await?;
        Ok(resp)
    }
}
use sea_orm::*;
use crate::errors::AppError;
use crate::models::role;


pub struct Role;
impl Role {
    pub async fn create_role(db: &DbConn, role: role::CreateRole) -> Result<i32, AppError> {
        let id = role::Role::create_role(db, role).await?;
        Ok(id)
    }
}
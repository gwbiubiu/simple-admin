use sea_orm::*;
use serde::{Serialize, Deserialize};
use crate::entities::user;
use crate::errors::AppError;
use crate::errors::user::UserError::NotFound;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub enabled: bool,
}


impl User {
    pub async fn get_user_by_id(db: &DbConn, id: i32) -> Result<Self> {
        let user = user::Entity::find_by_id(id).one(db).await?;
        let user = user.ok_or(AppError::UserError(NotFound))?;
        Ok(Self {
            id: user.id,
            username: user.username,
            email: user.email,
            password: None,
            enabled: user.enabled,
        })
    }
}
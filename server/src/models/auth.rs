
use serde::{Deserialize, Serialize};
use crate::entities::{user::Entity as User, user};
use anyhow::Result;
use sea_orm::*;
use crate::errors::{AppError, user::UserError::NotFound};
#[derive(Serialize)]
pub struct Token {
    pub token: String,
    pub expire: i64, // 过期时间秒
}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invite{
    pub email: String
}
pub struct Auth;

impl Auth {
    pub async fn find_user_by_username(db: &DbConn, login: &Login) -> Result<Option<user::Model>, AppError> {
        let user = User::find().
            filter(user::Column::Username.eq(&login.username)).
            one(db).await?;
        match user {
            Some(user) => Ok(Some(user)),
            None => Err(AppError::UserError(NotFound)),
        }
    }
}
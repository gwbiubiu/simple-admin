use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use actix_web::web;
use crate::entities::{user::Entity as User, user};
use anyhow::Result;
use sea_orm::*;
use crate::errors::{AppError, user::UserError::NotFound};
use crate::global::AppState;

pub const AUTH_TOKEN: &str = "auth_token";
pub const BLACK_AUTH_LIST: &str = "black_auth_list";

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

    #[warn(dependency_on_unit_never_type_fallback)]
    pub async fn add_token_to_black_list(app: web::Data<AppState>, token: &str) -> Result<(), AppError> {
        let key: String = format!("{}:{:x}", BLACK_AUTH_LIST, md5::compute(token));
        let mut redis_conn = app.redis_conn.lock().await;
        let jwt_expires_time = app.config.jwt.expires_time;
        redis_conn.set_ex(key,1_i32, jwt_expires_time).await?;
        Ok(())
    }
}
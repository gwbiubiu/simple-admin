use time::{Duration, OffsetDateTime};
use serde::{Deserialize, Serialize};
use crate::entities::{user::Entity as User, user};
use anyhow::Result;
use jsonwebtoken::{encode, EncodingKey, Header};
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
struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
}

impl Login {
    pub async fn find_user_by_username(&self, db: &DbConn) -> Result<Option<user::Model>, AppError> {
        let user = User::find().
            filter(user::Column::Username.eq(&self.username)).
            one(db).await?;
        match user {
            Some(user) => Ok(Some(user)),
            None => Err(AppError::UserError(NotFound)),
        }
    }

    pub fn generate_jwt(user_id: &str) -> Result<String> {
        let expiration = OffsetDateTime::now_utc() + Duration::hours(24);
        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiration.unix_timestamp(),
            iat: OffsetDateTime::now_utc().unix_timestamp(),
        };
        let secret = b"abc123123";  // 在实际应用中，应该使用环境变量或配置文件来存储密钥
        Ok(encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))?)
    }
}
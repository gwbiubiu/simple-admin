use sea_orm::*;
use crate::models;
use crate::entities;
use anyhow::Result;


pub struct User;

impl User {
    pub async fn create_user(db: &DbConn, user: models::CreateUser) -> Result<i64> {
        entities::user::ActiveModel {
            username: Set(user.username.to_owned()),
            email: Set(user.email.to_owned()),
            password: Set(user.password.to_owned()),
            ..Default::default()
        }.save(db).await?;
        Ok(1)
    }
}
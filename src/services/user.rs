use sea_orm::*;
use crate::models;
use crate::entities;
use anyhow::Result;
use argon2::Argon2;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHasher, SaltString};
use sea_orm::prelude::Expr;
use crate::errors::AppError;
use crate::errors::user::UserError;


pub struct User;

impl User {
    pub async fn create_user(db: &DbConn, user: models::CreateUser) -> Result<i32, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password = argon2.hash_password(user.password.to_owned().as_bytes(), &salt).unwrap().to_string();
        let user = entities::user::ActiveModel {
            username: Set(user.username.to_owned()),
            email: Set(user.email.to_owned()),
            password: Set(password),
            ..Default::default()
        };
        let rsp = entities::user::Entity::insert(user).exec(db).await?;
        Ok(rsp.last_insert_id)
    }

    pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<entities::user::Model>, AppError> {
        let user: Option<entities::user::Model> = entities::user::Entity::find_by_id(id).one(db).await?;
        let user = match user {
            Some(user) => user.into(),
            None => {
                return Err(AppError::UserError(UserError::NotFound));
            }
        };
        Ok(user)
    }


    pub async fn update_user_status(db: &DbConn, id: i32) -> Result<bool, AppError> {
        let update_result = entities::user::Entity::update_many()
            .filter(entities::user::Column::Id.eq(id))
            .col_expr(entities::user::Column::Enabled, Expr::cust("NOT enabled"))
            .exec(db)
            .await?;
        Ok(update_result.rows_affected > 0)
    }
}

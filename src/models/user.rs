use sea_orm::*;
use serde::{Serialize, Deserialize};
use crate::entities::user;
use crate::errors::AppError;
use crate::errors::user::UserError::NotFound;
use anyhow::Result;
use super::Page;

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

#[derive(Debug, Default, Deserialize, Clone)]
pub struct QueryUsers {
    pub username: Option<String>,
    #[serde(flatten)]
    pub page: Page,
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

    pub async fn get_user_list(db: &DbConn, query: QueryUsers) -> Result<(Vec<Self>, u64)> {
        let mut condition = Condition::all();
        if let Some(username) = query.username {
            condition = condition.add(user::Column::Username.contains(username));
        }
        let total_users = user::Entity::find().filter(condition.clone()).count(db).await?;
        let users = user::Entity::find()
            .filter(condition)
            .limit(query.page.size)
            .offset(query.page.page * query.page.size)
            .all(db)
            .await?;
        Ok((users.into_iter().map(|u| Self {
            id: u.id,
            username: u.username,
            email: u.email,
            password: None,
            enabled: u.enabled,
        }).collect(), total_users))
    }
}
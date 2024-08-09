use sea_orm::*;
use serde::{Serialize, Deserialize};
use crate::entities::{user, user_role};
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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUserRole {
    pub user_id: i32,
    pub role_id: Vec<i32>,
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

    pub async fn add_user_roles(db: &DatabaseConnection, user_role: AddUserRole) -> Result<bool, DbErr> {
        let mut user_roles = vec![];
        for role_id in user_role.role_id {
            user_roles.push(user_role::ActiveModel {
                user_id: ActiveValue::Set(user_role.user_id),
                role_id: ActiveValue::Set(role_id),
                ..Default::default()
            });
        }
        let resp = user_role::Entity::insert_many(user_roles).on_conflict(
            sea_query::OnConflict::columns([user_role::Column::UserId, user_role::Column::RoleId])
                .update_column(user_role::Column::UserId)
                .to_owned())
            .exec(db).await;
        match resp {
            Ok(_) => Ok(true),
            Err(e) => match e {
                DbErr::RecordNotInserted => Ok(false),
                _ => Err(e),
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user_roles() {
        let user_role = AddUserRole {
            user_id: 1,
            role_id: vec![1, 3],
        };
        let mut user_roles = vec![];
        for role_id in user_role.role_id {
            user_roles.push(user_role::ActiveModel {
                user_id: ActiveValue::Set(user_role.user_id),
                role_id: ActiveValue::Set(role_id),
                ..Default::default()
            });
        }
        assert_eq!(user_role::Entity::insert_many(user_roles).on_conflict(
            sea_query::OnConflict::columns([user_role::Column::UserId, user_role::Column::RoleId])
                .update_column(user_role::Column::UserId)
                .to_owned())
            .build(DbBackend::MySql)
            .to_string(), r#"INSERT INTO `user_roles` (`user_id`, `role_id`) VALUES (1, 1), (1, 3) ON DUPLICATE KEY UPDATE `user_id` = VALUES(`user_id`)"#,
            );
    }
}
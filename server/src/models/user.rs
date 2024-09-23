use sea_orm::*;
use serde::{Serialize, Deserialize};
use crate::entities::{user, user_role, role};
use crate::errors::AppError;
use crate::errors::user::UserError::NotFound;
use anyhow::Result;
use crate::errors::AppError::UserError;
use super::Page;
use chrono::{DateTime, Utc};

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
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserRole {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub roles: Vec<super::role::Role>,
}


impl User {
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
            create_time: u.create_time,
            update_time: u.update_time,
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

    pub async fn delete_user_role(db: &DatabaseConnection, user_role: DeleteUserRole) -> Result<bool, DbErr> {
        let resp = user_role::Entity::delete_many()
            .filter(user_role::Column::UserId.eq(user_role.user_id))
            .filter(user_role::Column::RoleId.eq(user_role.role_id))
            .exec(db).await;
        match resp {
            Ok(resp) => Ok(resp.rows_affected > 0),
            Err(e) => Err(e),
        }
    }

    pub async fn get_user_info(db: &DatabaseConnection, user_id: i32) -> Result<UserInfo, AppError> {
        let user_with_roles = user::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .find_with_related(role::Entity)
            .all(db)
            .await?;

        if user_with_roles.is_empty() {
            return Err(UserError(NotFound));
        }

        let (user, roles) = &user_with_roles[0];
        let user_info = UserInfo {
            id: user.id,
            username: user.username.to_owned(),
            email: user.email.to_owned(),
            roles: roles.iter().map(|r| super::role::Role {
                id: r.id,
                name: r.name.to_owned(),
                create_time: r.create_time,
                update_time: r.update_time,
            }).collect(),
        };
        Ok(user_info)
    }
}


impl From<user::Model> for User {
    fn from(user: user::Model) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            password: None,
            enabled: user.enabled,
            create_time: user.create_time,
            update_time: user.update_time,
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
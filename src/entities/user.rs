use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models;


#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub enabled: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}

impl From<models::User> for Model {
    fn from(user: models::User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            password: user.password,
            enabled: user.enabled,
        }
    }
}
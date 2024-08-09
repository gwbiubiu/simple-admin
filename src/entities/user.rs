use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub enabled: bool,
}


#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Role,
}


impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Role => Entity::has_many(super::role::Entity).into(),
        }
    }
}


impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_role::Relation::Role.def()
    }
    
    fn via() -> Option<RelationDef> {
        Some(super::user_role::Relation::User.def().rev())
    }
}


impl ActiveModelBehavior for ActiveModel {}


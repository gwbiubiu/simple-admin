use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}


impl Related<super::user::Entity> for Entity{
    fn to() -> RelationDef {
        super::user_role::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_role::Relation::Role.def().rev())
    }
}
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "role_apis")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub role_id: i32,
    pub api_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Role,
    Api,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Api => Entity::belongs_to(super::api::Entity)
                .from(Column::ApiId)
                .to(super::api::Column::Id)
                .into(),
            Self::Role => Entity::belongs_to(super::role::Entity)
                .from(Column::RoleId)
                .to(super::role::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
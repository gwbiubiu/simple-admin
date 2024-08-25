use std::collections::{HashMap, HashSet};
use log::error;
use sea_orm::*;
use crate::errors::AppError;
use crate::models::role::{self, AddRoleApi};
use crate::models::api;

pub struct Role;

impl Role {
    pub async fn create_role(db: &DbConn, role: role::CreateRole) -> Result<i32, AppError> {
        let id = role::Role::create_role(db, role).await?;
        Ok(id)
    }

    pub async fn get_role_by_id(db: &DbConn, id: i32) -> Result<role::Role, AppError> {
        let role = role::Role::get_role_by_id(db, id).await?;
        Ok(role)
    }

    pub async fn update_role(db: &DbConn, id: i32, role: role::UpdateRole) -> Result<bool, AppError> {
        let resp = role::Role::update_role(db, id, role).await?;
        Ok(resp)
    }


    pub async fn delete_role(db: &DbConn, id: i32) -> Result<bool, AppError> {
        let resp = role::Role::delete_role(db, id).await?;
        Ok(resp)
    }

    pub async fn list_role(db: &DbConn, query: role::QueryRole) -> Result<(Vec<role::Role>, u64), AppError> {
        let resp = role::Role::role_list(db, query).await?;
        Ok(resp)
    }


    pub async fn role_add_apis(db: &DbConn, role_api: AddRoleApi) -> Result<bool, AppError> {
        let result = db.transaction::<_, bool, DbErr>(|txn| {
            Box::pin(async move {
                role::Role::delete_role_api_by_role_id(txn, role_api.role_id).await?;
                role::Role::role_add_apis(txn, role_api).await?;
                Ok(true)
            })
        }).await;

        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                error!("Unexpected error during transaction: {:?}", e);
                Err(AppError::SystemError(e.to_string()))
            }
        }
    }

    pub async fn get_role_apis_group(db: &DbConn, role_id: i32) -> Result<Vec<role::RoleApiCategory>, AppError> {
        let role_apis = role::Role::get_role_apis(db, role_id).await?;
        let apis = api::Api::get_all_apis(db).await?;
        let mut role_apis_group = vec![];
        let api_set: HashSet<i32> = role_apis.into_iter().collect();
        let mut role_apis_map : HashMap<String, Vec<role::RoleHasApi>> = HashMap::new();
        //遍历role_api，按照名称放到map中
        for api in apis {
            let role_api = role::RoleHasApi {
                id: api.id,
                name: api.name,
                path: api.path,
                method: api.method,
                has: api_set.contains(&api.id),
            };
            if role_apis_map.contains_key(&api.api_group) {
                role_apis_map.get_mut(&api.api_group).unwrap().push(role_api);
            } else {
                role_apis_map.insert(api.api_group, vec![role_api]);
            }
        }
        for (k, v) in role_apis_map {
            role_apis_group.push(role::RoleApiCategory {
                api_group: k,
                items: v,
            });
        }
        role_apis_group.sort_by(|a, b: &role::RoleApiCategory| a.api_group.cmp(&b.api_group));

        Ok(role_apis_group)
    }
}
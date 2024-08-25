use serde::{Deserialize, Serialize};
use super::{get, post, Page, Response, Status};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Role {
    pub id: u32,
    pub name: String,
    pub create_time: DateTime<Utc>,
}


#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RoleListResp {
    pub page: Page,
    pub items: Vec<Role>,
}


#[derive(Clone, Deserialize, Eq, PartialEq)]
pub struct QueryRoleParams {
    pub page: u32,
    pub page_size: u32,
    pub name: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RoleApiCategory {
    pub api_group: String,
    pub items: Vec<RoleApi>,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RoleApi {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub has: bool,
}

impl QueryRoleParams {
    pub fn new() -> Self {
        Self {
            page: 0,
            page_size: 10,
            name: None,
        }
    }
}
pub async fn get_role_list(param: QueryRoleParams) -> Result<RoleListResp, String> {
    let mut url = format!("/api/v1/role/list?page={}&size={}", param.page, param.page_size);
    if let Some(name) = param.name {
        url.push_str(format!("&name={}", name).as_str());
    }
    let resp = get::<RoleListResp>(url.as_str()).await?;
    if resp.status == Status::SUCCESS {
        return Ok(resp.data.unwrap());
    }
    return Err(resp.message);
}

pub async fn get_role_apis_group(role_id: u32) -> Result<Vec<RoleApiCategory>, String> {
    let url = format!("/api/v1/role/{}/api_router_group", role_id);
    let resp = get::<Vec<RoleApiCategory>>(url.as_str()).await?;
    if resp.status == Status::SUCCESS {
        return Ok(resp.data.unwrap());
    }
    return Err(resp.message);
}

pub async fn role_api_update(role_id: u32, api_ids: Vec<u32>) -> Result<(), String> {
    let url = format!("/api/v1/role/{}/api_router", role_id);
    let resp:Response<()> = post(url.as_str(), api_ids).await?;
    if resp.status == Status::SUCCESS {
        return Ok(());
    }
    return Err(resp.message);
}
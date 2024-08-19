use serde::{Deserialize, Serialize};
use super::{get, Page, Status};
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
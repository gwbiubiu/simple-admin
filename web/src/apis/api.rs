use serde::{Deserialize, Serialize};
use super::{get, Page, Status};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Api {
    pub id: u32,
    pub name: String,
    pub create_time: DateTime<Utc>,
}


#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ApiListResp {
    pub page: Page,
    pub items: Vec<Api>,
}


#[derive(Clone, Deserialize, Eq, PartialEq)]
pub struct QueryApiParams {
    pub page: u32,
    pub page_size: u32,
    pub name: Option<String>,
}

impl QueryApiParams {
    pub fn new() -> Self {
        Self {
            page: 0,
            page_size: 10,
            name: None,
        }
    }
}
pub async fn get_api_list(param: QueryApiParams) -> Result<ApiListResp, String> {
    let mut url = format!("/api/v1/api_router/list?page={}&size={}", param.page, param.page_size);
    if let Some(api_name) = param.name {
        url.push_str(format!("&name={}", api_name).as_str());
    }
    let resp = get::<ApiListResp>(url.as_str()).await?;
    if resp.status == Status::SUCCESS {
        return Ok(resp.data.unwrap());
    }
    return Err(resp.message);
}

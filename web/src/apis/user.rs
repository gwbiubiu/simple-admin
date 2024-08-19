use serde::{Deserialize, Serialize};
use super::{get, Page, Status};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub create_time: DateTime<Utc>,
}


#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserListResp {
    pub page: Page,
    pub items: Vec<User>,
}


#[derive(Clone, Deserialize, Eq, PartialEq)]
pub struct QueryUserParams {
    pub page: u32,
    pub page_size: u32,
    pub username: Option<String>,
}

impl QueryUserParams {
    pub fn new() -> Self {
        Self {
            page: 0,
            page_size: 10,
            username: None,
        }
    }
}

pub async fn get_user_list(param: QueryUserParams) -> Result<UserListResp, String> {
    let mut url = format!("/api/v1/user/list?page={}&size={}", param.page, param.page_size);
    if let Some(username) = param.username {
        url.push_str(format!("&username={}", username).as_str());
    }
    match get::<UserListResp>(url.as_str()).await {
        Ok(resp) => {
            if resp.status == Status::SUCCESS {
                return Ok(resp.data.unwrap());
            }
            return Err(resp.message);
        }
        Err(_) => Err("Server Internal error".to_string()),
    }
}
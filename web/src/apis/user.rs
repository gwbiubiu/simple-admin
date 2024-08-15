use std::future::Future;
use serde::{Deserialize};
use super::{get, Page, Status};
use chrono::{DateTime, Utc};
#[derive(Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub create_time: DateTime<Utc>,
}

pub enum Msg {
    UserList(UserListResp),
    Error(String),
}


#[derive(Deserialize)]
pub struct UserListResp {
    pub page: Page,
    pub items: Vec<User>,
}


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

pub fn get_user_list(param: QueryUserParams) -> impl Future<Output=Msg> {
    async move {
        let mut url = format!("/api/v1/user/list?page={}&size={}", param.page, param.page_size);
        if let Some(username) = param.username {
            url.push_str(format!("&username={}", username).as_str());
        }
        match get::<UserListResp>(url.as_str()).await {
            Ok(resp) => {
                if resp.status == Status::SUCCESS {
                    return Msg::UserList(resp.data.unwrap());
                }
                return Msg::Error(resp.message);
            }
            Err(_) => Msg::Error("Server Internal error".to_string()),
        }
    }
}
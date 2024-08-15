use std::future::Future;
use serde::{Deserialize, Serialize};
use super::{post, Status};


pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    ToggleRememberMe,
    Submit,
    LoginSuccess(String),
    Error(String),
    HideError,
}

#[derive(Serialize)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RespToken {
    token: String,
    expire: i64,
}
impl Login {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
        }
    }
}

pub fn login(username: String, password: String) -> impl Future<Output=Msg> {
    async move {
        let param = Login::new(username, password);
        match post::<Login, RespToken>("/api/v1/login", param).await {
            Ok(resp) => {
                if resp.status == Status::SUCCESS {
                    return Msg::LoginSuccess(resp.data.unwrap().token);
                }
                return Msg::Error(resp.message);
            }
            Err(_) => Msg::Error("Server Internal error".to_string()),
        }
    }
}


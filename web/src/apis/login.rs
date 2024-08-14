use std::future::Future;
use serde::{Deserialize, Serialize};
use super::{post, Response};
use serde_json::Value;


pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    ToggleRememberMe,
    Submit,
    LoginSuccess(String),
    LoginFailed(String),
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
        match post::<Login, Value>("/api/v1/login", param).await {
            Ok(json_value) => {
                // 这里可以处理 token，例如存储到本地存储
                match serde_json::from_value::<Response<Value>>(json_value) {
                    Ok(resp) => {
                        match serde_json::from_value::<RespToken>(resp.data.unwrap()) {
                            Ok(token) => {
                                Msg::LoginSuccess(token.token)
                            }
                            Err(e) => Msg::LoginFailed(resp.message),
                        }
                    }
                    Err(e) => Msg::LoginFailed(format!("Failed to parse response: {}", e)),
                }
            }
            Err(_) => Msg::LoginFailed("Server Internal error".to_string()),
        }
    }
}


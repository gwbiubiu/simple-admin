use super::{post, get, Status, Response};
use gloo::console::log;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginParam {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RespToken {
   pub token: String,
   pub expire: i64,
}

pub async fn login(param: LoginParam) -> Result<RespToken, String> {
    let resp =  post::<_, RespToken>("/api/v1/login", param).await?;
    let resp_data = serde_json::to_string(&resp).unwrap();
    log!("login data: {}", resp_data);
    if resp.status == Status::SUCCESS {
        return Ok(resp.data.unwrap());
    }
    Err(resp.message)
}



pub async fn logout() -> Result<Response<()>, String> {
    let resp: crate::apis::Response<()> =  get::<()>("/api/v1/logout").await?;
    return Ok(resp);
}

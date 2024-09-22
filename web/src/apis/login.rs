use super::{post, Status};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginParam {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RespToken {
    token: String,
    expire: i64,
}

pub async  fn login(param: LoginParam) -> Result<RespToken, String> {
    match post::<_, RespToken>("/api/v1/login", param).await {
        Ok(resp) => {
            if resp.status == Status::SUCCESS {
                return Ok(resp.data.unwrap());
            }
            return Err(resp.message);
        }
        Err(_) => Err("Server Internal error".to_string()),
    }
}

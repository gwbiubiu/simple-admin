use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};


pub mod login;
mod user;

#[derive(Deserialize)]
pub enum Status {
    SUCCESS,
    FAIL,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Response<T> {
    pub status: Status,
    pub code: i64,
    pub message: String,
    pub data: Option<T>,
}


const BASE_URL: &str = "http://127.0.0.1:8080";

pub fn with_path(path: &str) -> String {
    format!("{}{}", BASE_URL, path)
}

pub async fn post<T, U>(path: &str, login: T) -> anyhow::Result<U>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let url = with_path(path);
    let resp = Request::post(url.as_str())
        .header("Content-Type", "application/json")
        .json(&login)?
        .send().await?;
    let resp = resp.json::<U>().await?;
    Ok(resp)
}

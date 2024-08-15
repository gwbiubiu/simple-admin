use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};


pub mod login;
pub mod user;

#[derive(Deserialize, PartialEq)]
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

#[derive(Deserialize, Clone)]
pub struct Page {
    pub page: u64,
    pub size: u64,
    pub total: u64,
}


const BASE_URL: &str = "http://127.0.0.1:8080";

pub fn with_path(path: &str) -> String {
    format!("{}{}", BASE_URL, path)
}

pub async fn post<T, U>(path: &str, data: T) -> anyhow::Result<Response<U>>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let url = with_path(path);
    let resp = Request::post(url.as_str())
        .header("Content-Type", "application/json")
        .json(&data)?
        .send().await?;
    let resp = resp.json().await?;
    Ok(resp)
}


pub async fn get<T>(path: &str) -> anyhow::Result<Response<T>>
where
    T: DeserializeOwned,
{
    let url = with_path(path);
    let resp = Request::get(url.as_str())
        .send().await?;
    let resp = resp.json().await?;
    Ok(resp)
}
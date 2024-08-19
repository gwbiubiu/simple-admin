use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub mod login;
pub mod user;
pub mod role;
pub mod api;

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

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct Page {
    pub page: u32,
    pub size: u32,
    pub total: u32,
}


const BASE_URL: &str = "http://127.0.0.1:8080";

pub fn with_path(path: &str) -> String {
    format!("{}{}", BASE_URL, path)
}

pub async fn post<T, U>(path: &str, data: T) -> Result<Response<U>, String>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let url = with_path(path);
    let resp = Request::post(url.as_str())
        .header("Content-Type", "application/json")
        .json(&data).map_err(|e| e.to_string())?
        .send().await.map_err(|e| e.to_string())?;
    let resp = resp.json().await.map_err(|e| e.to_string())?;
    Ok(resp)
}


pub async fn get<T>(path: &str) -> Result<Response<T>, String>
where
    T: DeserializeOwned,
{
    let url = with_path(path);
    let resp = Request::get(url.as_str())
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let resp = resp.json()
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp)
}
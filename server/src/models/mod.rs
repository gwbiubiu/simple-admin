pub mod user;
pub mod auth;
pub mod role;
pub mod api;
mod menu;


pub use user::*;

pub use auth::*;

pub use api::*;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct PageResponse<T>
where
    T: Serialize,
{
    pub page: Page,
    pub items: Vec<T>,
}

impl<T> PageResponse<T>
where
    T: Serialize,
{
    pub fn new(page: Page, items: Vec<T>) -> Self {
        Self { page, items }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Page {
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    pub page: u64,
    #[serde(default = "default_size", deserialize_with = "deserialize_number_from_string")]
    pub size: u64,
    #[serde(default)]
    pub total: u64,
}

impl Page {
    pub fn new(page: u64, size: u64, total: u64) -> Self {
        Self { page, size, total }
    }

    pub fn new_with_total(&self, total: u64) -> Self {
        Self {
            page: self.page,
            size: self.size,
            total,
        }
    }
}

fn deserialize_number_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}


fn default_size() -> u64 {
    10
}
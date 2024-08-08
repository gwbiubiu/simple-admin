pub mod user;
mod auth;
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
    #[serde(default)]
    pub page: u64,
    #[serde(default = "page_size")]
    pub size: u64,
    #[serde(default)]
    pub total: u64,
}

impl Page {
    pub fn new(page: u64, size: u64, total: u64) -> Self {
        Self { page, size, total }
    }
}

fn page_size() -> u64 {
    10
}
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
}

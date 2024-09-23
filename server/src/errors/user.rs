use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
    #[error("Invalid username or password ")]
    InvalidUserNameOrPassword,
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Invalid username")]
    InvalidUsername,
    #[error("Invalid user id")]
    InvalidUserId,
    #[error("Invalid user status")]
    InvalidUserStatus,
    #[error("Invalid user")]
    InvalidUser,
    #[error("Internal server error")]
    InternalServerError,
}
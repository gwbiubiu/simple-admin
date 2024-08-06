use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("User not found")]
    NotFound,
    #[error("Api same path and method has exists")]
    ApiAlreadyExists,
}
pub mod user;
pub mod api;

use actix_web::http::StatusCode;
use actix_web::ResponseError;
use thiserror::Error;
use user::UserError;
use crate::errors::api::ApiError;


#[derive(Error, Debug)]
pub enum AppError {
    #[error("BadRequest: {0}")]
    BadRequest(String),
    #[error("DatabaseError: {0}")]
    DatabaseError(#[from]sea_orm::DbErr),
    #[error("Internal Server Error")]
    SystemError(String),
    #[error("User Error")]
    UserError(UserError),
    #[error("Api Error")]
    ApiError(ApiError),
}

#[derive(Debug, serde::Serialize)]
struct ErrorResponse {
    message: String,
    code: u16,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::BAD_REQUEST,
            AppError::SystemError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let code = self.status_code();
        let message = match self {
            AppError::UserError(err) => err.to_string(),
            _ => self.to_string(),
        };
        actix_web::HttpResponse::build(code).json(ErrorResponse { message, code: code.as_u16() })
    }
}
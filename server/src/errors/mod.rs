pub mod user;
pub mod api;
pub mod roles;
mod menu;

use actix_web::http::StatusCode;
use actix_web::ResponseError;
use thiserror::Error;
use self::{user::UserError, api::ApiError, roles::RoleError};
use crate::models::{Response,Status};


#[allow(dead_code)]
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
    #[error("Role Error")]
    RoleError(RoleError),
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
            AppError::ApiError(err) => err.to_string(),
            AppError::RoleError(err) => err.to_string(),
            AppError::SystemError(msg) => msg.to_string(),
            _ => self.to_string(),
        };
        actix_web::HttpResponse::build(code).json(Response::<()> { status: Status::FAIL, message:message, code: 0 , data: None})
    }
}
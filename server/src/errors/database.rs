use thiserror::Error;
use super::AppError;


#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Mysql Database Error: {0}")]
    MysqlError(#[from] sea_orm::DbErr),
    #[error("Redis Error: {0}")]
    RedisError(#[from] redis::RedisError),
}


impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::DatabaseError(DatabaseError::MysqlError(err))
    }
}



impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::DatabaseError(DatabaseError::RedisError(err))
    }
}
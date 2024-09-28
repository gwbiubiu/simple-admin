
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::errors::AppError;
use redis::AsyncCommands;

pub const BLACK_AUTH_LIST: &str = "black_auth_list";

use redis::aio::MultiplexedConnection;
#[derive(Clone)]
pub struct RedisAdaptor{
    pub redis_conn: Arc<Mutex<MultiplexedConnection>>,
}


impl RedisAdaptor {
    pub fn new(redis_conn: Arc<Mutex<MultiplexedConnection>>) -> Self {
        Self {
            redis_conn
        }
    }

    pub async fn add_token_to_black_list(&self, token: &str, jwt_expires_time: u64) -> Result<(), AppError> {
        let key: String = format!("{}:{:x}", BLACK_AUTH_LIST, md5::compute(token));
        let mut redis_conn = self.redis_conn.lock().await;
        redis_conn.set_ex(key,1_i32, jwt_expires_time).await?;
        Ok(())
    }

    pub async  fn is_token_in_black_list(&self, token: String) -> Result<bool, AppError> {
        let key: String = format!("{}:{:x}", BLACK_AUTH_LIST, md5::compute(token));
        let mut redis_conn = self.redis_conn.lock().await;
        let exists: bool = redis_conn.exists(key).await?;
        Ok(exists)
    }
}

use crate::errors::AppError;
use redis::{Commands,  Connection};
use std::sync::RwLock;
pub const BLACK_AUTH_LIST: &str = "black_auth_list";


pub struct RedisAdaptor{
    pub redis_conn: RwLock<Box<Connection>>,
}


impl RedisAdaptor {
    pub fn new(redis_conn:Connection) -> Self {
        Self {
            redis_conn: RwLock::new(Box::new(redis_conn))
        }
    }

    pub fn add_token_to_black_list(&self, token: &str, jwt_expires_time: u64) -> Result<(), AppError> {
        let key: String = format!("{}:{:x}", BLACK_AUTH_LIST, md5::compute(token));
        let mut  conn = self.redis_conn.write().unwrap();
        let res  =  conn.set_ex(key,1_i32, jwt_expires_time)?;
        Ok(res)

    }

    pub  fn is_token_in_black_list(&self, token: String) -> Result<bool, AppError> {
        let key: String = format!("{}:{:x}", BLACK_AUTH_LIST, md5::compute(token));
        let mut conn = self.redis_conn.write().unwrap();        
        let exists: bool = conn.exists(key)?;
        Ok(exists)
    }
}
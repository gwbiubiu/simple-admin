use sea_orm::DatabaseConnection;
use crate::config::Config;
mod redis_adaptor;
pub use redis_adaptor::RedisAdaptor;
use std::sync::Arc;
pub struct AppState {
    pub conn: DatabaseConnection,
    pub redis_adaptor: Arc<RedisAdaptor>,
    pub config: Config,
}

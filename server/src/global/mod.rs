use sea_orm::DatabaseConnection;
use crate::config::Config;
mod redis_adaptor;
pub use redis_adaptor::RedisAdaptor;
pub struct AppState {
    pub conn: DatabaseConnection,
    pub redis_adaptor: RedisAdaptor,
    pub config: Config,
}

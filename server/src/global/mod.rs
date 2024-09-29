use sea_orm::DatabaseConnection;
use crate::config::Config;
mod redis_adaptor;
pub mod email;
pub use redis_adaptor::RedisAdaptor;
use email::EmailSender;
use std::sync::Arc;
pub struct AppState {
    pub conn: DatabaseConnection,
    pub redis_adaptor: RedisAdaptor,
    pub config: Config,
    pub email_sender: Arc<dyn EmailSender + Send + Sync>,
}

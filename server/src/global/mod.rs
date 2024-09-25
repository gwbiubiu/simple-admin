use sea_orm::DatabaseConnection;
use crate::config::Config;
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub conn: DatabaseConnection,
    pub redis_conn: Arc<Mutex<MultiplexedConnection>>,
    pub config: Config,
}

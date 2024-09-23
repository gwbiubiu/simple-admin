use sea_orm::DatabaseConnection;
use crate::config::Config;

#[derive(Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub config: Config,
}

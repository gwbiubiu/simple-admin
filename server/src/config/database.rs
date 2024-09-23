use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    mysql: Mysql,
}

#[derive(Debug, Deserialize)]
struct Mysql {
    host: String,
    port: u32,
    user: String,
    password: String,
    database: String,
}

impl Database {
    pub fn get_db_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.mysql.user,
            self.mysql.password,
            self.mysql.host,
            self.mysql.port,
            self.mysql.database
        )
    }
}
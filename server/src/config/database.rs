use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    mysql: Mysql,
    redis: Redis,
}

#[derive(Debug, Deserialize)]
struct Mysql {
    host: String,
    port: u32,
    user: String,
    password: String,
    database: String,
}


#[derive(Debug, Deserialize)]
struct Redis {
    host: String,
    port: u32,
    user: String,
    password: String,
    database: u8,
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
    //redis://[<username>][:<password>@]<hostname>[:port][/[<db>][?protocol=<protocol>]]
    pub fn get_redis_url(&self) -> String {
        format!(
            "redis://{}:{}@{}:{}/{}",
            self.redis.user,
            self.redis.password,
            self.redis.host,
            self.redis.port,
            self.redis.database
        )
    }
}

use std::fs::File;
use std::io::Read;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    database: Database,
}

#[derive(Debug, Deserialize)]
struct Database {
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

impl Default for Config {
    fn default() -> Self {
        let file_path = "./config.toml";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("Error opening file: {}", e),
        };
        let mut str = String::new();
        match file.read_to_string(&mut str) {
            Ok(_) => (),
            Err(e) => panic!("Error reading file: {}", e),
        };
        toml::from_str(&str).expect("Parsing the configuration file failed")
    }
}

impl Config {
    pub fn get_db_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.database.mysql.user,
            self.database.mysql.password,
            self.database.mysql.host,
            self.database.mysql.port,
            self.database.mysql.database
        )
    }
}

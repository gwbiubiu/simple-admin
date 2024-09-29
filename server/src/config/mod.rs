mod database;
mod jwt;
mod email;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use self::database::Database;
use self::jwt::Jwt;
use self::email::Email;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub jwt: Jwt,
    pub email: Email,
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


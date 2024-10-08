use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Jwt {
    pub signing_key: String,
    pub issuer: String,
    pub expires_time: u64,
    pub http_only: bool,
    pub secure: bool,
}
use serde::Deserialize;
use crate::global::email::{GmailSender,EmailSender};
use std::sync::Arc;
use std::boxed::Box;

#[derive(Debug, Deserialize)]
pub struct Email {
   pub gmail: Gmail
}
#[derive(Debug, Deserialize)]
pub struct Gmail{
    username: String,
    password: String,
    port: u16,
}

impl Gmail {
    pub fn get_gmail_sender(&self) -> Arc<dyn EmailSender + Send + Sync> {
        Arc::new(GmailSender::new(self.username.clone() , self.password.clone(), self.port))
    }
}
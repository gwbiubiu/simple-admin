mod gmail;
mod template;
use anyhow::Result;
pub use gmail::*;


pub trait EmailSender {
    fn send_register_email(&self, to:String, token: String)->Result<()>;
}

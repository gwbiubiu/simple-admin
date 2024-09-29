mod gmail;
use anyhow::Result;
pub use gmail::*;


pub trait EmailSender {
    fn send_register_email(&self, to:String)->Result<()>;
}

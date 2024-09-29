use anyhow::Result;

use super::EmailSender;

const GMAIL_SMTP_SERVER:&str = "smtp.gmail.com";
use lettre::transport::smtp::authentication::Credentials;
use lettre:: SmtpTransport;


pub struct GmailSender{
    transport: SmtpTransport,
}

impl GmailSender{
    pub fn new(username: String, password: String, port: u16)->Self{
        let creds = Credentials::new(username.to_owned(), password.to_owned());
        let mailer = SmtpTransport::relay(GMAIL_SMTP_SERVER)
        .unwrap()
        .credentials(creds)
        .port(port)
        .build();
        Self{
            transport: mailer
        }
    }
}

impl EmailSender for GmailSender{
    fn send_register_email(&self, to:String)->Result<()>{
        Ok(())
    }

}


#[cfg(test)]
mod tests{
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};
    
    #[test]
    fn test_email_adapter(){
        let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to("Hei <gwbiubiu@gmail.com>".parse().unwrap())
        .subject("Happy new year")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Be happy!"))
        .unwrap();

    let creds = Credentials::new("xxx".to_owned(), "xxx
".to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .port(587)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
    }
}
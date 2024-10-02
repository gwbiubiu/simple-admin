use anyhow::Result;
use super::EmailSender;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tera::Context;

const GMAIL_SMTP_SERVER: &str = "smtp.gmail.com";
pub struct GmailSender {
    transport: SmtpTransport,
}
use super::template::TEMPLATES;

impl GmailSender {
    pub fn new(username: String, password: String, port: u16) -> Self {
        let creds = Credentials::new(username.to_owned(), password.to_owned());
        let mailer = SmtpTransport::relay(GMAIL_SMTP_SERVER)
            .unwrap()
            .credentials(creds)
            .port(port)
            .build();
        Self { transport: mailer }
    }
}

impl EmailSender for GmailSender {
    fn send_register_email(&self, to: String, token: String) -> Result<()> {
        let mut context = Context::new();
        context.insert("link", "localhost:8081/link");
        let names: Vec<_> = TEMPLATES.get_template_names().collect();
        println!("{:?}", names);
        println!("{:?}", "register.html");
        let html_body = TEMPLATES.render("register.html", &context)?;

        println!("{:?}", html_body);

        println!("to name is {}", to);
        let email: Message = Message::builder()
            .from("gwbiubiu <gwbiubiu@gmail.com>".parse().unwrap())
            .to(to.parse().unwrap())
            .subject("Happy new year")
            .header(ContentType::TEXT_PLAIN)
            .body(html_body)
            .unwrap();
        self.transport.send(&email)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

    #[test]
    fn test_email_adapter() {
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to("Hei <gwbiubiu@gmail.com>".parse().unwrap())
            .subject("Happy new year")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Be happy!"))
            .unwrap();

        let creds = Credentials::new(
            "xxx".to_owned(),
            "xxx
"
            .to_owned(),
        );

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

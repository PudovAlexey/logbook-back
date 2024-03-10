use lettre::message::header::{ContentType, From};
use lettre:: {SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use crate::common::env::ENV;

pub struct Mailer {
   pub to: String,
   pub subject: String,
   pub body: String,
}

impl Mailer {
   pub fn new (params: Mailer) -> Self {
        Self {
            to: params.to,
            subject: params.subject,
            body: params.body,
        }
    }
   pub fn send(&self) {
        let email =  Message::builder()
            .from("pudo.aleksej177@gmail.com".parse().unwrap())
            // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to(self.to.parse().unwrap())
            .subject(self.body.clone())
            .header(ContentType::TEXT_PLAIN)
            .body(self.body.clone())
            .unwrap();

        let creds = Credentials::new(ENV::new().SMTP_USERNAME.to_owned(), ENV::new().SMTP_PASSWORD.to_owned());

        let mailer = SmtpTransport::relay(&ENV::new().SMTP_TRANSPORT)
            .unwrap()
            .credentials(creds)
            .build();

            match mailer.send(&email) {
                Ok(_) => println!("Email sent successfully!"),
                Err(e) => panic!("Could not send email: {e:?}"),
            }   
    }
}

use crate::common::env::ENV;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::{SmtpTransport, Transport};

pub struct Mailer {
    pub to: String,
    pub subject: String,
    pub header: ContentType,
    pub body: String,
}

impl Mailer {
    pub fn new(params: Mailer) -> Self {
        Self {
            to: params.to,
            subject: params.subject,
            header: params.header,
            body: params.body,
        }
    }
    pub fn send(&self) -> Result<String, String> {
        let email = Message::builder()
            .from(ENV::new().smtp_username.parse().unwrap())
            // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to(self.to.parse().unwrap())
            .subject(self.body.clone())
            .header(self.header.clone())
            .body(self.body.clone())
            .unwrap();

        let creds = Credentials::new(
            ENV::new().smtp_username.to_owned(),
            ENV::new().smtp_password.to_owned(),
        );

        let mailer = SmtpTransport::relay(&ENV::new().smtp_transport)
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => Ok(String::from("Email sent successfully!")),
            Err(e) => {
                println!("{}", e);
                Err(String::from("error"))
            }
        }
    }
}

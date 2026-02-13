use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::header::ContentType,
    transport::{self, smtp::authentication::Credentials},
};

use crate::shared::ports::mail::EmailService;

#[derive(Clone)]
pub struct SmtpEmailService {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
}

impl SmtpEmailService {
    pub fn new(smtp_host: String, smtp_user: String, smtp_pass: String) -> Self {
        let creds = Credentials::new(smtp_user.clone(), smtp_pass);

        //transport config
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
            .expect("Failed to build SMTP transport")
            .credentials(creds)
            .build();

        Self {
            transport,
            from_email: smtp_user,
        }
    }
}

#[async_trait::async_trait]
impl EmailService for SmtpEmailService {
    async fn send_email(&self, to: String, subject: String, body: String) -> Result<(), String> {
        let email = Message::builder()
            .from(
                self.from_email
                    .parse()
                    .map_err(|e| format!("Email from error: {}", e))?,
            )
            .to(to.parse().map_err(|e| format!("Email to error: {}"))?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body)
            .map_err(|e| format!("Body error: {}", e))?;

        match self.transport.send(email).await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error sending email: {:?}", e);
                Err(format!("SMTP Error: {}", e))
            }
        }
    }
}

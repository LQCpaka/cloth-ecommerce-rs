#[async_trait::async_trait]
pub trait EmailService: Send + Sync {
    async fn send_email(&self, to: String, subject: String, body: String) -> Result<(), String>;
}

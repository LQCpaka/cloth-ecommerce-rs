use async_trait::async_trait;
use resend_rs::{Resend, types::{CreateEmailBaseOptions, Tag}};
use crate::{
    shared::ports::mail::{MailService, EmailPayload, EmailResponse}
    error::AppError
};

#[derive(Clone)]
pub struct ResendMailService {
    client: Resend,
    from_email: String,

}


impl ResendMailService {
    pub fn new(api_key: &str, from_email: String) -> Self {
        let client = Resend::new(api_key);
        Self {client, from_email}
    }
}

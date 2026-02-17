use std::sync::Arc;

use crate::shared::ports::mail::{EmailPayload, MailService};

pub struct EmailConfig {
    pub email_to_send: String,
    pub name_to_send: String,
    pub email_service_clone: Arc<dyn MailService>,
    pub domain_url: String,
    pub verification_token: String,
}

pub async fn verification_email(email_config: EmailConfig) {
    tokio::spawn(async move {
        let link = format!(
            "http://{}/verify?token={}&email={}",
            email_config.domain_url, email_config.verification_token, email_config.email_to_send
        );

        let html_body = format!(
            r#"<!DOCTYPE html>
            <html>
              <body style="margin:0; padding:0; font-family:Arial, sans-serif;">
                <table width="100%" cellpadding="0" cellspacing="0" style="background:#f4f4f4; padding:20px;">
                  <tr>
                    <td align="center">
                      <table width="600" cellpadding="0" cellspacing="0" style="background:#ffffff; border-radius:8px; overflow:hidden;">
                        <tr>
                          <td style="padding:20px; text-align:center; background:#808080 ; color:#fff;">
                            <h2>Chào {}!</h2>
                          </td>
                        </tr>
                        <tr>
                          <td style="padding:20px; color:#333;">
                            <h2 style="text-align:center;">Đây là email kích hoạt tài khoản của bạn:</h2>
                            <p style="text-align:center;">
                              <a href="{}"
                                 style="display:inline-block; background:#808080 ; color:#fff; padding:12px 24px; text-decoration:none; border-radius:4px;">
                                 Kích hoạt ngay
                              </a>
                            </p>
                            <p style="font-size:12px; color:#777;text-align:center;">Link hết hạn sau 5 phút.</p>
                          </td>
                        </tr>
                      </table>
                    </td>
                  </tr>
                </table>
              </body>
            </html>
            "#,
            email_config.name_to_send, link
        );

        let payload = EmailPayload {
            to: vec![email_config.email_to_send],
            subject: "Kích hoạt tài khoản".to_string(),
            html_body,
            text_body: None,
            cc: None,
            bcc: None,
        };

        if let Err(e) = email_config.email_service_clone.send_email(payload).await {
            tracing::error!("Failed to send verification email (OTP DB): {:?}", e)
        }
    });
}

pub async fn resend_verification_email(email_config: EmailConfig) {
    tokio::spawn(async move {
        let link = format!(
            "http://{}/verify?token={}&email={}",
            email_config.domain_url, email_config.verification_token, email_config.email_to_send
        );

        let html_body = format!(
            r#"<!DOCTYPE html>
            <html>
                <body style="margin:0; padding:0; font-family:Arial, sans-serif;">
                <table width="100%" cellpadding="0" cellspacing="0" style="background:#f4f4f4; padding:20px;">
                    <tr>
                    <td align="center">
                        <table width="600" cellpadding="0" cellspacing="0" style="background:#ffffff; border-radius:8px; overflow:hidden;">
                        <tr>
                            <td style="padding:20px; text-align:center; background:#808080 ; color:#fff;">
                            <h2>Chào {}!</h2>
                            </td>
                        </tr>
                        <tr>
                            <td style="padding:20px; color:#333;">
                            <h2 style="text-align:center;">Đây là email kích hoạt tài khoản mới của bạn:</h2>
                            <p style="text-align:center;">
                                <a href="{}"
                                    style="display:inline-block; background:#808080 ; color:#fff; padding:12px 24px; text-decoration:none; border-radius:4px;">
                                    Kích hoạt ngay
                                </a>
                            </p>
                            <p style="font-size:12px; color:#777;text-align:center;">Vui lòng không xóa email này nếu bạn chưa hoàn tất việc xác thực tài khoản.</p>
                            <p style="font-size:12px; color:#777;text-align:center;">Link hết hạn sau 5 phút.</p>
                            </td>
                        </tr>
                        </table>
                    </td>
                    </tr>
                </table>
                </body>
            </html>
            "#,
            email_config.name_to_send, link
        );

        let payload = EmailPayload {
            to: vec![email_config.email_to_send],
            subject: "Kích hoạt tài khoản".to_string(),
            html_body,
            text_body: None,
            cc: None,
            bcc: None,
        };

        if let Err(e) = email_config.email_service_clone.send_email(payload).await {
            tracing::error!("Failed to send verification email (OTP DB): {:?}", e)
        }
    });
}

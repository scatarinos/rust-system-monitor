use crate::config::{Config, NotificationType, EmailConfig};
use log::{info, warn, error};
use reqwest::blocking::Client; // Or reqwest::Client for async
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use serde_json::Value;


/// Sends a notification based on the configuration provided.
/// This function checks the notification type specified in the configuration
/// and sends a notification accordingly. It supports console logging,
/// webhook notifications, and email notifications.
pub fn send_notification(config: &Config, message: &Value) {
    match config.notifications.notification_type {
        NotificationType::Console => {
            warn!("[ALERT] {}", message);
        }
        NotificationType::Webhook => {
            if let Some(url) = &config.notifications.webhook_url {
                if let Err(e) = send_webhook(url, &message) {
                    error!("Failed to send webhook: {}", e);
                } else {
                    info!("Webhook sent: {}", message);
                }
            } else {
                error!("Webhook URL not configured.");
            }
        }
        NotificationType::Email => {
            if let Some(email_cfg) = &config.notifications.email_config {
                if let Err(e) = send_email(email_cfg, &message.to_string()) {
                    error!("Failed to send email: {}", e);
                } else {
                    info!("Email sent to {}: {}", email_cfg.recipient, message);
                }
            } else {
                error!("Email configuration missing.");
            }
        }
    }
}

fn send_webhook(url: &str, message: &Value) -> Result<(), reqwest::Error> {
    let client = Client::new();
    // let payload = serde_json::json!({ "text": message }); // Example for Slack/Discord webhook
    client.post(url).json(&message).send()?;
    Ok(())
}

fn send_email(config: &EmailConfig, message_body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(config.from_address.parse()?)
        .to(config.recipient.parse()?)
        .subject("System Monitor Alert!")
        .body(String::from(message_body))?;

    let mailer = if let (Some(username), Some(password)) = (&config.username, &config.password) {
        SmtpTransport::relay(&config.smtp_server)?
            .port(config.smtp_port)
            .credentials(Credentials::new(username.clone(), password.clone()))
            .build()
    } else {
        SmtpTransport::relay(&config.smtp_server)?
            .port(config.smtp_port)
            .build()
    };

    mailer.send(&email)?;
    Ok(())
}
use std::fmt;
use serde::{Deserialize, Serialize, Deserializer};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
/// Represents the configuration for the system monitor application.
/// 
/// This struct contains the configuration settings for monitoring and
/// notifications, encapsulated in their respective sub-configurations.
///
/// # Fields
/// 
/// * `monitor` - Configuration settings related to monitoring functionality.
/// * `notifications` - Configuration settings related to notification functionality.
pub struct Config {
    pub monitor: MonitorConfig,
    pub notifications: NotificationConfig,
}

#[derive(Debug, Deserialize, Serialize)]
/// Configuration settings for the system monitor.
///
/// This struct defines the thresholds and intervals used by the system monitor
/// to check system resource usage and trigger alerts when thresholds are exceeded.
///
/// # Fields
/// - `check_interval_seconds`: The interval, in seconds, at which the system monitor
///   checks the resource usage.
/// - `cpu_threshold_percent`: The CPU usage threshold, as a percentage, that triggers
///   an alert when exceeded.
/// - `memory_threshold_percent`: The memory usage threshold, as a percentage, that
///   triggers an alert when exceeded.
/// - `disk_threshold_percent`: The disk usage threshold, as a percentage, that triggers
///   an alert when exceeded.
pub struct MonitorConfig {
    pub check_interval_seconds: u64,
    pub cpu_threshold_percent: f64,
    pub memory_threshold_percent: f64,
    pub disk_threshold_percent: f64,
}

#[derive(Debug, Deserialize, Serialize)]
/// Configuration for notifications in the system.
///
/// This struct defines the settings required to configure notifications,
/// including the type of notification, optional webhook URL, and optional
/// email configuration.
///
/// # Fields
/// - `notification_type`: Specifies the type of notification (e.g., webhook, email).
/// - `webhook_url`: An optional URL for webhook notifications. This is used
///   when the notification type is set to webhook.
/// - `email_config`: An optional configuration for email notifications. This
///   is used when the notification type is set to email.
pub struct NotificationConfig {
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub webhook_url: Option<String>,
    pub email_config: Option<EmailConfig>,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum NotificationType {
    Console,
    Webhook,
    Email,
}
/// Custom implementation of the `Deserialize` trait for the `NotificationType` enum.
/// 
/// This implementation allows deserialization of `NotificationType` from a string,
/// ignoring case sensitivity. The following string values are supported:
/// 
/// - `"console"`: Maps to `NotificationType::Console`
/// - `"webhook"`: Maps to `NotificationType::Webhook`
/// - `"email"`: Maps to `NotificationType::Email`
/// 
/// If the input string does not match any of the above values, a custom error is returned
/// indicating an unknown notification type.
/// 
/// # Errors
/// 
/// Returns a `serde::de::Error` if the input string does not match any of the expected
/// notification types.
impl<'de> Deserialize<'de> for NotificationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();

        match s.as_str() {
            "console" => Ok(NotificationType::Console),
            "webhook" => Ok(NotificationType::Webhook),
            "email" => Ok(NotificationType::Email),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown notification type: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for NotificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            NotificationType::Console => "Console",
            NotificationType::Webhook => "Webhook",
            NotificationType::Email => "Email",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailConfig {
    pub recipient: String,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub from_address: String,
}


impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;  // Read the entire file as a string
        let config = toml::from_str::<Config>(&content)?;  // Parse TOML into Config
        Ok(config)
    }
}
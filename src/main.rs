mod config;
mod metrics;
mod notify;

use sysinfo::{System};
use std::{thread, time::Duration};
use log::{info, warn, error, LevelFilter};
use env_logger::Builder;
use clap::Parser;
use chrono::Utc;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to config.toml
    #[arg(short, long, default_value = "./system_monitor_config.toml")]
    config: String,

    /// Override config values (format: key=value)
    #[arg(long = "set", value_name = "KEY=VALUE", action = clap::ArgAction::Append)]
    overrides: Vec<String>,    
}

fn apply_overrides(config: &mut config::Config, overrides: &[String]) {
    for ov in overrides {
        let parts: Vec<&str> = ov.splitn(2, '=').collect();
        if parts.len() != 2 {
            warn!("Invalid override '{}', expected key=value", ov);
            continue;
        }
        let key = parts[0].trim();
        let value = parts[1].trim();

        match key {
            "notifications.type" => {
                config.notifications.notification_type = match value.to_lowercase().as_str() {
                    "console" => config::NotificationType::Console,
                    "webhook" => config::NotificationType::Webhook,
                    "email" => config::NotificationType::Email,
                    _ => {
                        warn!("Unknown notification type: {}", value);
                        config.notifications.notification_type.clone()
                    }
                };
            }
            "monitor.check_interval_seconds" => {
                if let Ok(v) = value.parse() {
                    config.monitor.check_interval_seconds = v;
                    info!("Check interval seconds set to: {}", v);
                } else {
                    warn!("Invalid value for check_interval_seconds: {}", value);
                }
            }
            "monitor.cpu_threshold_percent" => {
                if let Ok(v) = value.parse() {
                    config.monitor.cpu_threshold_percent = v;
                    info!("CPU threshold percent set to: {}", v);
                } else {
                    warn!("Invalid value for cpu_threshold_percent: {}", value);
                }
            }
            "monitor.memory_threshold_percent" => {
                if let Ok(v) = value.parse() {
                    config.monitor.memory_threshold_percent = v;
                }
            }
            "monitor.disk_threshold_percent" => {
                if let Ok(v) = value.parse() {
                    config.monitor.disk_threshold_percent = v;
                }
            }
            _ => warn!("Unknown override key: {}", key),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config_path = args.config.as_str();

    println!("Using configuration file: {}", config_path);
    println!("Overrides: {:?}", args.overrides);


    // Initialize logger
    Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    warn!("Starting System Monitor...");

    // 1. Load Configuration
    let mut config = match config::Config::load(config_path) {
        Ok(cfg) => {
            info!("Configuration loaded successfully from: {:?}", config_path);
            cfg
        },
        Err(e) => {
            error!("Failed to load configuration: {}. Please ensure 'config.toml' exists and is valid.", e);
            return Err(e);
        }
    };
    apply_overrides(&mut config, &args.overrides);

    let mut sys = System::new_all(); // Initialize sysinfo system

    // 2. Monitoring Loop
    loop {
        // Refresh system information
        sys.refresh_all(); 

        let metrics = metrics::collect_metrics(&mut sys);
        info!("Current Metrics: CPU: {:.2}%, Mem: {:.2}%, Disk: {:.2}%",
              metrics.cpu_usage_percent,
              metrics.memory_usage_percent,
              metrics.disk_usage_percent);

        // Check thresholds and send notifications
        if metrics.cpu_usage_percent >= config.monitor.cpu_threshold_percent {
            let msg = format!(
                "High CPU Usage Alert! Current: {:.2}% (Threshold: {:.2}%)",
                metrics.cpu_usage_percent, config.monitor.cpu_threshold_percent
            );
            let payload = serde_json::json!({ "text": msg, "timestamp": Utc::now().to_rfc3339() });
            notify::send_notification(&config, &payload);
        }

        for (i, &core_usage) in metrics.cpu_usage_per_core.iter().enumerate() {
            if core_usage >= config.monitor.cpu_threshold_percent {
                let msg = format!(
                    "High CPU Usage Alert on Core {}! Current: {:.2}% (Threshold: {:.2}%)",
                    i + 1, core_usage, config.monitor.cpu_threshold_percent
                );
                let payload = serde_json::json!({ "text": msg, "timestamp": Utc::now().to_rfc3339() });
                notify::send_notification(&config, &payload);
            }
        }

        if metrics.memory_usage_percent >= config.monitor.memory_threshold_percent {
            let msg = format!(
                "High Memory Usage Alert! Current: {:.2}% (Threshold: {:.2}%)",
                metrics.memory_usage_percent, config.monitor.memory_threshold_percent
            );
            let payload = serde_json::json!({ "text": msg, "timestamp": Utc::now().to_rfc3339() });
            notify::send_notification(&config, &payload);
        }

        if metrics.disk_usage_percent >= config.monitor.disk_threshold_percent {
            let msg = format!(
                "High Disk Usage Alert! Current: {:.2}% (Threshold: {:.2}%)",
                metrics.disk_usage_percent, config.monitor.disk_threshold_percent
            );
            let payload = serde_json::json!({ "text": msg, "timestamp": Utc::now().to_rfc3339() });
            notify::send_notification(&config, &payload);
        }

        // Sleep for the configured interval
        thread::sleep(Duration::from_secs(config.monitor.check_interval_seconds));
    }
}
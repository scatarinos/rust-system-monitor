use system_monitor::{config::Config, metrics::collect_metrics};
use sysinfo::System;

#[test]
fn test_valid_config_load() {
    let toml_data = r#"
        [monitor]
        check_interval_seconds = 5
        cpu_threshold_percent = 75.0
        memory_threshold_percent = 65.0
        disk_threshold_percent = 90.0

        [notifications]
        type = "Console"
    "#;

    let config: Config = toml::from_str(toml_data).expect("Failed to parse config");
    assert_eq!(config.monitor.cpu_threshold_percent, 75.0);
    assert_eq!(config.notifications.notification_type.to_string(), "Console");
}

#[test]
fn test_metrics_within_expected_range() {
    let mut sys = System::new_all();
    let metrics = collect_metrics(&mut sys);

    assert!((0.0..=100.0).contains(&metrics.cpu_usage_percent));
    assert!((0.0..=100.0).contains(&metrics.memory_usage_percent));
    assert!((0.0..=100.0).contains(&metrics.disk_usage_percent));
}

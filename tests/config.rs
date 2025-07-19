use system_monitor::config::Config;

#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    #[test]
    fn test_load_from_toml_string() {
        let data = r#"
            [monitor]
            check_interval_seconds = 1
            cpu_threshold_percent = 80.0
            memory_threshold_percent = 50.0
            disk_threshold_percent = 95.0

            [notifications]
            type = "Console"
        "#;

        let config: Config = toml::from_str(data).unwrap();
        assert_eq!(config.monitor.check_interval_seconds, 1);
    }
}

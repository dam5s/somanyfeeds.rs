use somanyfeeds_server::worker::WorkerSettings;

#[test]
fn test_worker_settings_new() {
    let settings = WorkerSettings::new(60);
    assert_eq!(settings.interval_seconds, 60);
}

#[test]
fn test_worker_settings_new_zero_defaults_to_30() {
    let settings = WorkerSettings::new(0);
    assert_eq!(settings.interval_seconds, 30);
}

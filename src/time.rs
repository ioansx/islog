pub fn now_timestamp_nanos() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time should not go backwards")
        .as_nanos()
}

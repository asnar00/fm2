struct feature_OneHour;
impl feature_OneHour {
    // a team signs itself up in the first minutes of a session; the code need
    // not outlive the hour
    fn qr_ttl_ms() -> u64 {
        3600000
    }
}

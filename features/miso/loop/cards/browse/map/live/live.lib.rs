// the ephemeral presence store: one {lat, lon, t} per world key, in this
// process's memory and nowhere else. A verbatim library because the
// accessor's return type carries a comma the chain parser cannot read;
// the verbs that use it are chain functions in live.rs.
pub fn live_cell() -> &'static std::sync::Mutex<std::collections::HashMap<String, serde_json::Value>> {
    static LIVE: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<String, serde_json::Value>>>
        = std::sync::OnceLock::new();
    LIVE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

// per-feature logging: a feature says what it is doing, and is heard only
// when switched on. The path comes from the linker, never from the author.
// verbatim library — full Rust, outside the chain machinery. See logging.md.

thread_local! {
    // paths switched on for this user, refreshed once per turn
    static FM_LOG_ON: std::cell::RefCell<Vec<String>> =
        std::cell::RefCell::new(Vec::new());
    // lines gathered during the turn, drained into state at the end
    static FM_LOG_BUF: std::cell::RefCell<Vec<serde_json::Value>> =
        std::cell::RefCell::new(Vec::new());
}

/// arm the turn: read which node paths this user has switched on.
/// Parsed properly (not scanned) — this runs once per event, not per call.
pub fn fm_log_arm(state: &str) {
    let s: serde_json::Value = serde_json::from_str(state)
        .unwrap_or(serde_json::json!({}));
    let raw = s["feature_log"].as_str().unwrap_or("");
    let map: serde_json::Value = serde_json::from_str(raw)
        .unwrap_or(serde_json::Value::Null);
    let mut on: Vec<String> = Vec::new();
    if let Some(o) = map.as_object() {
        for (k, v) in o {
            if v.as_bool() == Some(true) {
                on.push(k.clone());
            }
        }
    }
    FM_LOG_ON.with(|c| *c.borrow_mut() = on);
}

/// is this node — or an ancestor of it — switched on? Absent means off,
/// the mirror image of /context-manager's ticks.
pub fn fm_log_on(path: &str) -> bool {
    FM_LOG_ON.with(|c| c.borrow().iter().any(|p| {
        path == p
            || (path.len() > p.len()
                && path.starts_with(p.as_str())
                && path.as_bytes()[p.len()] == b'/')
    }))
}

/// what `fm_log(msg)` becomes: the linker supplies the path
pub fn fm_log_at(path: &str, msg: String) {
    if !fm_log_on(path) {
        return;
    }
    FM_LOG_BUF.with(|c| c.borrow_mut().push(
        serde_json::json!({ "p": path, "m": msg })));
}

pub fn fm_log_drain() -> Vec<serde_json::Value> {
    FM_LOG_BUF.with(|c| c.borrow_mut().drain(..).collect())
}

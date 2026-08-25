// presence: which users' apps are in front of them right now, as the server
// can see it — a live /msg/wait long-poll is a page that is running and
// listening. Process-global, like the store lock.
// fully qualified on purpose: harden.lib.rs already `use`s Mutex in this crate
static FM_PRESENCE: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<String, u64>>> = std::sync::OnceLock::new();

fn fm_presence_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

pub fn presence_touch(key: String) {
    if key.is_empty() {
        return;
    }
    let m = FM_PRESENCE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(mut g) = m.lock() {
        g.insert(key, fm_presence_now());
    }
}

pub fn presence_recent(key: String, within_ms: u64) -> bool {
    let m = FM_PRESENCE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(g) = m.lock() {
        if let Some(t) = g.get(&key) {
            return fm_presence_now().saturating_sub(*t) <= within_ms;
        }
    }
    false
}

// the op log, and residency. verbatim library — the maps and the file work
// carry generics and commas the chain parser cannot express. see remember.md.
//
// fm:context-remember — the linker's hook: this token asks for nothing to be
// emitted, but it is what lets a composition without the op methods fail by
// name instead of as a rustc error inside this file.

/// where a user's log lives. `MISO_CONTEXT_DIR` overrides — which is how a test
/// server redirects its whole state — and the default sits beside the auth
/// state in the home directory. Deliberately not /tmp: /tmp does not survive a
/// reboot, and this week it demonstrably did not survive a Tuesday.
pub fn context_dir() -> String {
    match std::env::var("MISO_CONTEXT_DIR") {
        Ok(d) if !d.is_empty() => d,
        _ => format!("{}/.miso-context",
                     std::env::var("HOME").unwrap_or_default()),
    }
}

/// a user key as a filename. Every byte outside [A-Za-z0-9._-] is
/// percent-encoded, so `/` can never survive into a path and no key shape —
/// this rung's or a later one's — can name a file outside the directory.
pub fn context_log_file(user: &str) -> String {
    let mut safe = String::new();
    for b in user.bytes() {
        let c = b as char;
        if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
            safe.push(c);
        } else {
            safe.push_str(&format!("%{:02X}", b));
        }
    }
    format!("{}/{}.log", context_dir(), safe)
}

// ---- health: persistence may degrade, but the server may not go down --------

pub fn context_log_health() -> &'static std::sync::Mutex<(u64, String)> {
    static H: std::sync::OnceLock<std::sync::Mutex<(u64, String)>> =
        std::sync::OnceLock::new();
    H.get_or_init(|| std::sync::Mutex::new((0, String::new())))
}

/// a persistence failure is loud on stderr and visible at diag/context/log, and
/// it never becomes a failed request: the edit already happened in memory, and
/// refusing it after the fact would be a second lie on top of the first.
pub fn context_log_fail(what: String) {
    eprintln!("miso: context log: {}", what);
    let mut h = context_log_health().lock().unwrap_or_else(|p| p.into_inner());
    h.0 = h.0 + 1;
    h.1 = what;
}

// ---- the log ---------------------------------------------------------------

/// every record in a user's log, in arrival order. A line that is not a
/// well-formed record is announced and skipped rather than stopping the load:
/// a torn tail costs its own record and nothing before it.
pub fn context_log_read(user: &str) -> Vec<serde_json::Value> {
    let file = context_log_file(user);
    let raw = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(_) => return Vec::new(),   // no log yet is not a failure
    };
    let mut out = Vec::new();
    for (i, line) in raw.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let v: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                context_log_fail(format!("{}:{}: not JSON ({}) — skipped",
                                         file, i + 1, e));
                continue;
            }
        };
        if !v["path"].is_string() || !v["name"].is_string()
            || !v["op"].is_string() || v["value"].is_null() {
            context_log_fail(format!(
                "{}:{}: record needs path, name, op and value — skipped",
                file, i + 1));
            continue;
        }
        out.push(v);
    }
    out
}

/// fold a log to the shortest sequence that replays to the same world.
///
/// A `set` assigns, so it supersedes everything logged for that var before it.
/// An `add` accumulates, so consecutive adds fold into one add of their sum.
/// Per var that leaves at most a `set` followed by one `add`. Ops for different
/// vars never interact, so reordering across vars is safe and order within a
/// var is preserved — which is the whole argument that this is exact.
pub fn context_log_compact(records: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
    let mut order: Vec<String> = Vec::new();
    let mut by_var: std::collections::HashMap<String, Vec<serde_json::Value>> =
        std::collections::HashMap::new();
    for r in records {
        let key = format!("{}\u{1}{}", r["path"].as_str().unwrap_or(""),
                          r["name"].as_str().unwrap_or(""));
        if !by_var.contains_key(&key) {
            order.push(key.clone());
            by_var.insert(key.clone(), Vec::new());
        }
        let slot = by_var.get_mut(&key).expect("just inserted");
        if r["value"].is_array() {
            // a `counter` record: [epoch, sum] for a set, [epoch, delta] for an
            // add. Folding these by the rules above would rescue records that
            // replay is going to DROP — a stale reset, or an add from before
            // one. So a counter's records are folded by running apply_op's own
            // rule over them and emitting the single `set` that lands where
            // replay would: exact by construction rather than by argument.
            let epoch = r["value"][0].as_u64().unwrap_or(0);
            let n = r["value"][1].as_u64().unwrap_or(0);
            let (cur_e, cur_s) = match slot.first() {
                Some(p) => (p["value"][0].as_u64().unwrap_or(0),
                            p["value"][1].as_u64().unwrap_or(0)),
                None => (0u64, 0u64),
            };
            let (next_e, next_s) = match r["op"].as_str().unwrap_or("") {
                "set" if epoch >= cur_e => (epoch, n),
                "add" if epoch == cur_e => (cur_e, cur_s + n),
                _ => (cur_e, cur_s),   // dropped, exactly as replay would
            };
            let mut folded = r.clone();
            folded["op"] = serde_json::json!("set");
            folded["value"] = serde_json::json!([next_e, next_s]);
            slot.clear();
            slot.push(folded);
            continue;
        }
        if r["op"].as_str().unwrap_or("") == "set" {
            slot.clear();
            slot.push(r);
        } else if r["op"].as_str().unwrap_or("") == "add" {
            let delta = r["value"].as_u64().unwrap_or(0);
            let last_is_add = slot.last()
                .map(|p| p["op"].as_str().unwrap_or("") == "add")
                .unwrap_or(false);
            if last_is_add {
                let prev = slot.last_mut().expect("last_is_add");
                let sum = prev["value"].as_u64().unwrap_or(0) + delta;
                prev["value"] = serde_json::json!(sum);
            } else {
                slot.push(r);
            }
        } else {
            slot.push(r);   // an op word this build does not know: keep it
        }
    }
    let mut out = Vec::new();
    for key in order {
        if let Some(rs) = by_var.remove(&key) {
            for r in rs {
                out.push(r);
            }
        }
    }
    out
}

/// how long a log may get before the next append rewrites it compacted.
pub fn context_log_max() -> usize {
    match std::env::var("MISO_CONTEXT_LOG_MAX") {
        Ok(v) => v.parse().unwrap_or(512),
        Err(_) => 512,
    }
}

/// append one record, compacting first if the log has grown past its bound.
/// The whole file is rewritten through a temporary and renamed into place, so
/// a reader never sees a half-written log and a failure leaves the previous
/// one intact.
pub fn context_log_append(user: &str, record: serde_json::Value) {
    let file = context_log_file(user);
    if let Err(e) = std::fs::create_dir_all(context_dir()) {
        context_log_fail(format!("cannot create {} ({})", context_dir(), e));
        return;
    }
    let mut records = context_log_read(user);
    records.push(record);
    if records.len() > context_log_max() {
        records = context_log_compact(records);
    }
    let mut body = String::new();
    for r in &records {
        body.push_str(&r.to_string());
        body.push('\n');
    }
    let tmp = format!("{}.tmp{}", file, std::process::id());
    if let Err(e) = std::fs::write(&tmp, body) {
        context_log_fail(format!("cannot write {} ({})", tmp, e));
        return;
    }
    if let Err(e) = std::fs::rename(&tmp, &file) {
        context_log_fail(format!("cannot rename into {} ({})", file, e));
        let _ = std::fs::remove_file(&tmp);
    }
}

// ---- residency: which worlds are in memory, and how stale ------------------

pub fn context_now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// user key -> (last touch, that user's cell). Presence means "this world has
/// been replayed from its log and is current"; absence means the next touch
/// must rebuild it.
///
/// The cell is carried here rather than looked up again at eviction time so
/// that eviction never needs rung 5's table. Lock order is table (rung 5) then
/// residency then cell, always in that direction; eviction takes the last two,
/// which is a suffix of it, so the two paths cannot deadlock each other.
pub fn context_residency()
    -> &'static std::sync::Mutex<
        std::collections::HashMap<String, (u64, &'static std::sync::RwLock<Context>)>>
{
    static R: std::sync::OnceLock<
        std::sync::Mutex<
            std::collections::HashMap<String,
                                      (u64, &'static std::sync::RwLock<Context>)>>>
        = std::sync::OnceLock::new();
    R.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

pub fn context_resident_count() -> usize {
    context_residency().lock().unwrap_or_else(|p| p.into_inner()).len()
}

/// make this user's world current: on the first touch since it was last
/// evicted (or since the process started), replay their log into it.
///
/// The whole decision happens under the residency lock, so two threads meeting
/// a cold world cannot both replay it — the second waits and finds it warm. The
/// lock order is always residency then cell; nothing takes them the other way.
pub fn context_reside(user: &str, cell: &'static std::sync::RwLock<Context>) {
    let mut live = context_residency().lock().unwrap_or_else(|p| p.into_inner());
    if live.contains_key(user) {
        live.insert(user.to_string(), (context_now_ms(), cell));
        return;
    }
    let records = context_log_read(user);
    {
        let mut world = cell.write().unwrap_or_else(|p| p.into_inner());
        for r in &records {
            let path = r["path"].as_str().unwrap_or("").to_string();
            let name = r["name"].as_str().unwrap_or("").to_string();
            let op = r["op"].as_str().unwrap_or("").to_string();
            if let Err(e) = world.apply_op(&path, &name, &op, r["value"].clone()) {
                // a record for a var this composition no longer declares, or
                // whose merge has changed: announced, skipped, rest applied.
                context_log_fail(format!("{}: replaying {}/{} — {}",
                                         context_log_file(user), path, name,
                                         e.chars().take(120).collect::<String>()));
            }
        }
    }
    live.insert(user.to_string(), (context_now_ms(), cell));
}

/// how long a world may sit untouched before it is dropped from memory.
pub fn context_idle_ms() -> u64 {
    match std::env::var("MISO_CONTEXT_IDLE_MS") {
        Ok(v) => v.parse().unwrap_or(3600000),
        Err(_) => 3600000,
    }
}

/// drop every world idle past the threshold, except the one this request is
/// for. Returns the keys dropped. Reclaim is safe because recovery is total:
/// the log holds everything the world was built from.
pub fn context_evict_idle(except: &str) -> Vec<String> {
    let cutoff = context_now_ms().saturating_sub(context_idle_ms());
    let mut live = context_residency().lock().unwrap_or_else(|p| p.into_inner());
    let mut dropped: Vec<(String, &'static std::sync::RwLock<Context>)> = Vec::new();
    for (user, entry) in live.iter() {
        if user.as_str() != except && entry.0 <= cutoff {
            dropped.push((user.clone(), entry.1));
        }
    }
    let mut names: Vec<String> = Vec::new();
    for (user, cell) in dropped {
        let mut world = cell.write().unwrap_or_else(|p| p.into_inner());
        *world = Context::fresh();
        live.remove(&user);
        names.push(user);
    }
    names
}

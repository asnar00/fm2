// the context manager's hook: is `path` (or an ancestor) explicitly false
// in the user's feature_ticks map? Called by linker-emitted gates at the
// head of every chain-extending state fn — see context-manager.md.
// verbatim library: full Rust, outside the chain machinery.

thread_local! {
    // (raw escaped ticks slice, explicit-false paths) — reparsed only when
    // the slice changes; the common case is a pointer-fast string compare
    static FM_TICKS_CACHE: std::cell::RefCell<(String, Vec<String>)> =
        std::cell::RefCell::new((String::new(), Vec::new()));
}

pub fn fm_unticked(state: &str, path: &str) -> bool {
    // find the ticks KEY: the occurrence must be followed by ':' — the same
    // text also appears as a VALUE inside queued VarSet messages
    // (`"key":"feature_ticks"`), where the next char is ',' — skip those.
    // absent means everything on.
    let key = "\"feature_ticks\"";
    let mut from = 0;
    let at = loop {
        let i = match state[from..].find(key) {
            Some(i) => from + i,
            None => return false,
        };
        let after = state[i + key.len()..].trim_start();
        if after.starts_with(':') {
            break i + key.len();
        }
        from = i + key.len();
    };
    // scan the raw JSON-escaped string value: `:"..."` with escaped quotes
    let rest = &state[at..];
    let colon = match rest.find(':') {
        Some(i) => i,
        None => return false,
    };
    let mut chars = rest[colon + 1..].char_indices().peekable();
    let mut start = None;
    let mut end = None;
    let mut escaped = false;
    while let Some((i, c)) = chars.next() {
        match start {
            None => {
                if c == '"' {
                    start = Some(i + 1);
                } else if !c.is_whitespace() {
                    return false;   // not a string value (wrong shape): fail open
                }
            }
            Some(_) => {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == '"' {
                    end = Some(i);
                    break;
                }
            }
        }
    }
    let (s, e) = match (start, end) {
        (Some(s), Some(e)) => (s, e),
        _ => return false,
    };
    let raw = &rest[colon + 1 + s..colon + 1 + e];
    if !raw.contains("false") {
        return false;   // no explicit off anywhere: nothing to check
    }
    FM_TICKS_CACHE.with(|cell| {
        let mut cache = cell.borrow_mut();
        if cache.0 != raw {
            // unescape the JSON string, then read the map's explicit falses
            let quoted = format!("\"{}\"", raw);
            let inner: String =
                serde_json::from_str(&quoted).unwrap_or_default();
            let map: serde_json::Value =
                serde_json::from_str(&inner).unwrap_or(serde_json::Value::Null);
            let mut off = Vec::new();
            if let Some(obj) = map.as_object() {
                for (k, v) in obj {
                    if v.as_bool() == Some(false) {
                        off.push(k.clone());
                    }
                }
            }
            *cache = (raw.to_string(), off);
        }
        cache.1.iter().any(|p| {
            path == p || (path.len() > p.len()
                          && path.starts_with(p.as_str())
                          && path.as_bytes()[p.len()] == b'/')
        })
    })
}

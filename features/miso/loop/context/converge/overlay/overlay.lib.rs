// the shared layer, op identity, and the seen-set. verbatim library — the
// frozen-view accessor is generic over its closure's return type and the maps
// carry commas, neither of which the chain parser can express. see overlay.md.
//
// fm:context-overlay — the linker's hook: this token asks for the presence
// record, the resolved read, `clear`, and the scope lookup. Untick this node
// and every read goes back to being a raw `.value`.

/// the shared layer's key in rung 5's table. It is not a user key: those are
/// `phone:` or `local:` prefixed and this one is neither, so nothing a cookie
/// or a `?user=` parameter can spell will ever name it.
pub fn context_layer_key() -> &'static str {
    "_global"
}

pub fn context_layer_cell() -> std::sync::Arc<std::sync::RwLock<Context>> {
    context_of(context_layer_key())
}

// the turn's frozen view of the LAYER, taken beside rung 3's frozen view of the
// user's own world. Both are frozen at the same boundary, so a resolved read is
// consistent for the whole turn: a value cannot fall through mid-event.
thread_local! {
    static FM_LAYER_TURN: std::cell::RefCell<Option<Context>> =
        std::cell::RefCell::new(None);
}

pub fn context_layer_begin() {
    let cell = context_layer_cell();
    let frozen = cell
        .read()
        .unwrap_or_else(|p| p.into_inner())
        .clone();
    FM_LAYER_TURN.with(|t| {
        *t.borrow_mut() = Some(frozen);
    });
}

pub fn context_layer_end() {
    FM_LAYER_TURN.with(|t| {
        *t.borrow_mut() = None;
    });
}

/// read the layer. Inside a turn this is the frozen clone and takes no lock at
/// all; outside one it copies the live value. `f` answers None when the layer
/// has no value for the var, which is what makes the fall-through continue.
///
/// The one rule: never call a resolver from inside `edit_context` on the layer
/// itself — the write lock is held there and this would want a read. Every read
/// path on both places runs inside a turn, where no lock is taken.
pub fn context_layer<R>(f: impl FnOnce(&Context) -> Option<R>) -> Option<R> {
    let mut f = Some(f);
    let framed = FM_LAYER_TURN.with(|t| {
        let held = t.borrow();
        match held.as_ref() {
            Some(ctx) => Some((f.take().unwrap())(ctx)),
            None => None,
        }
    });
    match framed {
        Some(r) => r,
        None => {
            let layer = context_layer_cell();
            let copy = layer
                .read()
                .unwrap_or_else(|p| p.into_inner())
                .clone();
            (f.take().unwrap())(&copy)
        }
    }
}

/// write the layer, the counterpart of rung 3's `edit_context`.
///
/// A LOCAL edit of a global-scoped var has to land here, not in the caller's own
/// world: a global var's resolver reads the layer and never the user's field, so
/// an edit written anywhere else would be invisible until the server relayed it
/// back — which is exactly the optimistic-display-and-offline behaviour the loop
/// must not lose. The write is to the LIVE layer; rung 7a's re-freeze before the
/// paint is what makes it visible in the same frame.
pub fn edit_layer<R>(f: impl Fn(&mut Context) -> R) -> R {
    let out = {
        let cell = context_layer_cell();
        let mut live = cell.write().unwrap_or_else(|p| p.into_inner());
        f(&mut *live)
    };
    // read-your-own-writes for the layer, the exact twin of rung 3's for a
    // user's own world: the turn's OWN closure is replayed against the turn's
    // OWN frozen view, so a later link — or the paint — sees what an earlier
    // one wrote, while another device's edit stays invisible until the next
    // turn. Without this the layer's freshness depended on somebody calling
    // `context_layer_begin` again at the right moment, which meant it depended
    // on link order: an edit made by a node newer than /payload landed after
    // the re-freeze and the frame showed the old value.
    //
    // The replay must change the value and NOTHING else, so the mirror flag is
    // raised exactly as `edit_context` raises it and the op queue skips.
    context_mirror_set(true);
    FM_LAYER_TURN.with(|t| {
        if let Some(view) = t.borrow_mut().as_mut() {
            let _ = f(view);
        }
    });
    context_mirror_set(false);
    out
}

// ---- op identity ----------------------------------------------------------

/// this instance's nonce. The server mints its own; a wasm place has no clock
/// and no entropy source this composition can reach, so it asks the server for
/// one (CtxHello) and stamps nothing until the answer arrives.
pub fn context_instance() -> &'static std::sync::Mutex<(String, u64)> {
    static I: std::sync::OnceLock<std::sync::Mutex<(String, u64)>> =
        std::sync::OnceLock::new();
    I.get_or_init(|| std::sync::Mutex::new((String::new(), 0)))
}

pub fn context_instance_set(nonce: String) {
    let mut i = context_instance().lock().unwrap_or_else(|p| p.into_inner());
    if i.0.is_empty() {
        i.0 = nonce;
    }
}

pub fn context_instance_now() -> String {
    context_instance().lock().unwrap_or_else(|p| p.into_inner()).0.clone()
}

/// the next op id for this instance, or "" if this place does not know who it
/// is yet. Monotonic, so a retry of a stamped message carries the same id.
pub fn context_op_next_id() -> String {
    let mut i = context_instance().lock().unwrap_or_else(|p| p.into_inner());
    if i.0.is_empty() {
        return String::new();
    }
    i.1 = i.1 + 1;
    format!("{}.{}", i.0, i.1)
}

/// mint a nonce for a client that has just said hello. Server-side only: it
/// reads the clock, which a wasm place cannot.
pub fn context_mint_nonce() -> String {
    let mut i = context_instance().lock().unwrap_or_else(|p| p.into_inner());
    i.1 = i.1 + 1;
    let n = i.1;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    format!("{:x}{:x}{:x}", std::process::id(), t, n)
}

// ---- the seen-set ---------------------------------------------------------

pub fn context_seen_max() -> usize {
    match std::env::var("MISO_CONTEXT_SEEN_MAX") {
        Ok(v) => v.parse().unwrap_or(4096),
        Err(_) => 4096,
    }
}

pub fn context_seen()
    -> &'static std::sync::Mutex<(std::collections::VecDeque<String>,
                                  std::collections::HashSet<String>)>
{
    static S: std::sync::OnceLock<
        std::sync::Mutex<(std::collections::VecDeque<String>,
                          std::collections::HashSet<String>)>>
        = std::sync::OnceLock::new();
    S.get_or_init(|| std::sync::Mutex::new(
        (std::collections::VecDeque::new(), std::collections::HashSet::new())))
}

/// remember an op id, answering whether it is NEW. Bounded FIFO: at the bound
/// the oldest id is forgotten, and an op that old arriving again would be
/// applied twice — the residual named in overlay.md.
pub fn context_seen_mark(user: &str, id: &str) -> bool {
    if id.is_empty() {
        return true;   // an unstamped op is unguarded, by policy
    }
    let key = format!("{}\u{1}{}", user, id);
    let mut s = context_seen().lock().unwrap_or_else(|p| p.into_inner());
    if s.1.contains(&key) {
        return false;
    }
    s.0.push_back(key.clone());
    s.1.insert(key);
    while s.0.len() > context_seen_max() {
        if let Some(old) = s.0.pop_front() {
            s.1.remove(&old);
        }
    }
    true
}

/// forget everything remembered about one user's ops. Safe because it is all
/// derivable again: the ids come from their log, and `context_seen_prime`
/// re-reads it the next time this process sees them.
pub fn context_seen_forget(user: &str) {
    let prefix = format!("{}\u{1}", user);
    {
        let mut s = context_seen().lock().unwrap_or_else(|p| p.into_inner());
        s.0.retain(|k| !k.starts_with(&prefix));
        s.1.retain(|k| !k.starts_with(&prefix));
        s.0.shrink_to_fit();
        s.1.shrink_to_fit();
    }
    let mut primed = context_primed().lock().unwrap_or_else(|p| p.into_inner());
    primed.remove(user);
    primed.shrink_to_fit();
}

/// which users' logs have already been read into the seen-set.
pub fn context_primed() -> &'static std::sync::Mutex<std::collections::HashSet<String>> {
    static P: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashSet<String>>>
        = std::sync::OnceLock::new();
    P.get_or_init(|| std::sync::Mutex::new(std::collections::HashSet::new()))
}

/// prime the seen-set from a user's log the first time this process meets them,
/// so an op that was applied before a restart is not applied again after one.
/// Reads the log through the persistence node's own reader — one format, one
/// door, exactly as replay does.
pub fn context_seen_prime(user: &str) {
    {
        let primed = context_primed().lock().unwrap_or_else(|p| p.into_inner());
        if primed.contains(user) {
            return;
        }
    }
    for record in context_log_read(user) {
        if let Some(id) = record["id"].as_str() {
            let _ = context_seen_mark(user, id);
        }
    }
    let mut primed = context_primed().lock().unwrap_or_else(|p| p.into_inner());
    primed.insert(user.to_string());
}

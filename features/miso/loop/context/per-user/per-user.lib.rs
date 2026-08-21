// per-user contexts: the process's one world becomes a table keyed by user.
// verbatim library — the table's type carries generics and commas the chain
// parser cannot express, and the thread-local is the same shape rung 3 used.
// see per-user.md.

// who this thread is acting as, for the duration of one request. Empty means
// "no request identity" — startup, the wasm place, or an unauthenticated
// tunnel caller — and that case reads the process's own single world, exactly
// as it did before this rung.
thread_local! {
    static FM_CONTEXT_USER: std::cell::RefCell<String> =
        std::cell::RefCell::new(String::new());
}

pub fn context_user_set(who: String) {
    FM_CONTEXT_USER.with(|u| *u.borrow_mut() = who);
}

pub fn context_user_clear() {
    FM_CONTEXT_USER.with(|u| u.borrow_mut().clear());
}

pub fn context_user_now() -> String {
    FM_CONTEXT_USER.with(|u| u.borrow().clone())
}

/// the table: one context per user, materialised on first touch.
///
/// The entries are counted handles (`Arc<RwLock<Context>>`), and every caller
/// takes one by value. They were leaked `&'static` references when this table
/// was written, which kept `held_context()`'s signature exactly as rung 2 gave
/// it — at the price that a world could be reset but never freed. A handle
/// costs one atomic increment per touch and makes eviction mean it.
pub fn context_table()
    -> &'static std::sync::RwLock<
        std::collections::HashMap<String, std::sync::Arc<std::sync::RwLock<Context>>>>
{
    static TABLE: std::sync::OnceLock<
        std::sync::RwLock<
            std::collections::HashMap<String, std::sync::Arc<std::sync::RwLock<Context>>>>>
        = std::sync::OnceLock::new();
    TABLE.get_or_init(|| std::sync::RwLock::new(std::collections::HashMap::new()))
}

/// this user's context, created fresh on first touch.
///
/// The read guard is scoped explicitly: an `if let` would hold it across the
/// write below, and taking a write lock while holding this thread's read lock
/// is a deadlock, not a wait.
pub fn context_of(user: &str) -> std::sync::Arc<std::sync::RwLock<Context>> {
    {
        let table = context_table().read().unwrap_or_else(|p| p.into_inner());
        if let Some(cell) = table.get(user) {
            return cell.clone();
        }
    }
    let mut table = context_table().write().unwrap_or_else(|p| p.into_inner());
    table.entry(user.to_string()).or_insert_with(|| {
        std::sync::Arc::new(std::sync::RwLock::new(Context::fresh()))
    }).clone()
}

/// drop this process's handle to a user's world. The world itself goes when the
/// last handle does — a request already holding one finishes against the world
/// it started with, and the next touch builds a fresh one from the log.
///
/// The table lock is taken here and nowhere below, so a caller must hold
/// neither the residency lock nor a cell when it calls: the order is always
/// table, then residency, then the cell.
pub fn context_forget(user: &str) -> bool {
    context_table().write().unwrap_or_else(|p| p.into_inner())
        .remove(user).is_some()
}

/// how many users this process is holding a world for. Tooling only.
pub fn context_user_count() -> usize {
    context_table().read().unwrap_or_else(|p| p.into_inner()).len()
}

// the stash between /undo's link (where the pre-event snapshot is taken)
// and the outermost link (where every tool's edits are finally in the
// outbox). The loop is one thread; the Mutex is for the borrow checker.
pub static FM_UNDO_STASH: std::sync::Mutex<Option<(serde_json::Value, usize, String)>> =
    std::sync::Mutex::new(None);
pub static FM_UNDO_LATE: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

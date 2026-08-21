// the device's undo stack: what a local edit changed, and what it was before.
// verbatim library — a Vec of records, which the chain parser cannot express,
// and the same shape and the same reason as converge's outbox: thread-local,
// per-device, and BENEATH the context rather than a var in it (undo.md argues
// the choice).

/// how many steps back one device remembers. The ask says "the last change",
/// and a person pressing undo four times in a row is still asking about
/// something they can remember doing; ten is generous for that and small
/// enough that the stack is never a memory question.
pub const FM_UNDO_DEPTH: usize = 10;

thread_local! {
    static FM_UNDO_STACK: std::cell::RefCell<Vec<serde_json::Value>> =
        std::cell::RefCell::new(Vec::new());
}

/// push one step: `{tool, changes: [{path, name, merge, scope, prior}]}`.
/// The bound drops the OLDEST — the step most likely to be wanted is the one
/// just made, so a full stack must never evict the newest.
pub fn undo_push(entry: serde_json::Value) {
    FM_UNDO_STACK.with(|s| {
        let mut st = s.borrow_mut();
        st.push(entry);
        while st.len() > FM_UNDO_DEPTH {
            st.remove(0);
        }
    });
}

/// the newest step made while THIS tool was open, taken off the stack. Steps
/// belonging to other tools are stepped over rather than disturbed, so two
/// tools' histories never consume each other.
pub fn undo_take(tool: &str) -> Option<serde_json::Value> {
    FM_UNDO_STACK.with(|s| {
        let mut st = s.borrow_mut();
        let at = st.iter().rposition(|e| e["tool"].as_str().unwrap_or("") == tool)?;
        Some(st.remove(at))
    })
}

/// has this tool anything to undo? The button reads this to decide whether it
/// is live or dimmed.
pub fn undo_has(tool: &str) -> bool {
    FM_UNDO_STACK.with(|s| {
        s.borrow().iter().any(|e| e["tool"].as_str().unwrap_or("") == tool)
    })
}

/// the whole stack, oldest first — rigs and tooling only.
pub fn undo_stack_json() -> serde_json::Value {
    FM_UNDO_STACK.with(|s| serde_json::Value::Array(s.borrow().clone()))
}

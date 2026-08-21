// the write API a declared merge earns, and the outbox that carries what it
// produces. verbatim library — the trait impls are parameterised on the marker
// types, which is the whole point and which the chain parser cannot express.
// see converge.md.

// ops produced by a local edit, waiting for the turn to hand them to the
// state's `_send` outbox. Thread-local for the same reason rung 3's frozen
// view is: the client is one thread, and on the server a request is one.
thread_local! {
    static FM_CONTEXT_OUTBOX: std::cell::RefCell<Vec<serde_json::Value>> =
        std::cell::RefCell::new(Vec::new());
}

/// one op, addressed the way the snapshot addresses a var. The `op` word is
/// not the caller's choice — it comes from the marker impl that produced it.
pub fn context_op_queue(op: &str, path: &str, name: &str,
                        value: serde_json::Value) {
    FM_CONTEXT_OUTBOX.with(|o| o.borrow_mut().push(serde_json::json!({
        "type": "CtxOp",
        "data": { "path": path, "name": name, "op": op, "value": value }
    })));
}

/// take everything queued. The turn does this once, at its outermost link.
pub fn context_op_drain() -> Vec<serde_json::Value> {
    FM_CONTEXT_OUTBOX.with(|o| std::mem::take(&mut *o.borrow_mut()))
}

/// how many ops are waiting. Tooling only.
pub fn context_op_pending() -> usize {
    FM_CONTEXT_OUTBOX.with(|o| o.borrow().len())
}

/// last-write semantics. This impl is bounded on `MergeLastWrite`, so `set_at`
/// exists on a var that declared last-write and on no other: reaching for it on
/// a crdt-sum var is a compile error, not a convention.
pub trait VarLastWrite<T> {
    fn set_at(&mut self, path: &str, name: &str, v: T);
}

impl<T, S, I> VarLastWrite<T> for Var<T, S, MergeLastWrite, I>
where
    T: serde::Serialize,
    S: Permits<I>,
    I: VarInherit,
{
    fn set_at(&mut self, path: &str, name: &str, v: T) {
        self.value = v;
        // the declared SCOPE decides whether the op leaves this place, exactly
        // as SyncVar's `if self.scope != Scope::Local` does. A device-scoped
        // var is written locally and never shipped.
        if S::TAG != "device" {
            context_op_queue("set", path, name,
                serde_json::to_value(&self.value)
                    .unwrap_or(serde_json::Value::Null));
        }
    }
}

/// crdt-sum semantics, on `MergeCrdtSum` and nowhere else. The op carries the
/// DELTA, so two instances adding concurrently both count.
pub trait VarCrdtSum {
    fn add_at(&mut self, path: &str, name: &str, delta: u64);
}

impl<S, I> VarCrdtSum for Var<u64, S, MergeCrdtSum, I>
where
    S: Permits<I>,
    I: VarInherit,
{
    fn add_at(&mut self, path: &str, name: &str, delta: u64) {
        self.value = self.value + delta;
        if S::TAG != "device" {
            context_op_queue("add", path, name, serde_json::json!(delta));
        }
    }
}

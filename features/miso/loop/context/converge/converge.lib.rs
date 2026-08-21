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
    // rung 3's read-your-own-writes replays an edit against the turn's frozen
    // view; that replay is the same change to a second copy, not a second
    // change, so it must not put a second op on the wire.
    if in_context_mirror() {
        return;
    }
    FM_CONTEXT_OUTBOX.with(|o| o.borrow_mut().push(serde_json::json!({
        "type": "CtxOp",
        "data": { "path": path, "name": name, "op": op, "value": value }
    })));
}

/// take everything queued. The turn does this once, at its outermost link.
pub fn context_op_drain() -> Vec<serde_json::Value> {
    FM_CONTEXT_OUTBOX.with(|o| std::mem::take(&mut *o.borrow_mut()))
}

/// how many ops are waiting. Tooling, and the turn-end phase's early exit.
pub fn context_op_pending() -> usize {
    FM_CONTEXT_OUTBOX.with(|o| o.borrow().len())
}

/// what is waiting, without taking it. The drain happens at the turn's end
/// now, so a link that wants to know what this turn changed has to look in the
/// outbox rather than in `state["_send"]` — that is where the ops are while
/// the turn is still running. Reading is not draining: the phase still ships
/// exactly what was queued.
pub fn context_op_peek() -> Vec<serde_json::Value> {
    FM_CONTEXT_OUTBOX.with(|o| o.borrow().clone())
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

/// the counter kind speaks BOTH verbs, which is what makes it the only merge
/// that needs two impls. An add sums within the current epoch and ships the
/// epoch it was minted under; a set bumps the epoch, which is what makes every
/// add still in flight from before the reset droppable on arrival.
impl<S, I> VarCrdtSum for Var<Counter, S, MergeCounter, I>
where
    S: Permits<I>,
    I: VarInherit,
{
    fn add_at(&mut self, path: &str, name: &str, delta: u64) {
        let minted = self.value.epoch;
        self.value.sum = self.value.sum + delta;
        if S::TAG != "device" {
            // (epoch it was minted under, delta)
            context_op_queue("add", path, name,
                serde_json::to_value(&Counter::at(minted, delta))
                    .unwrap_or(serde_json::Value::Null));
        }
    }
}

impl<S, I> VarLastWrite<u64> for Var<Counter, S, MergeCounter, I>
where
    S: Permits<I>,
    I: VarInherit,
{
    fn set_at(&mut self, path: &str, name: &str, v: u64) {
        // a reset is a new epoch. Minting it locally means two resets racing
        // can mint the same number; the server takes the later arrival and the
        // loser's adds are dropped either way (converge.md names this).
        let next = Counter::at(self.value.epoch + 1, v);
        self.value = next;
        if S::TAG != "device" {
            context_op_queue("set", path, name,
                serde_json::to_value(&next).unwrap_or(serde_json::Value::Null));
        }
    }
}

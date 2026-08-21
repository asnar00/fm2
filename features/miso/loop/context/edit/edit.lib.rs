// the context's read and write primitives, plus the turn boundary that keeps
// an edit invisible to the event already in flight. verbatim library — full
// Rust (the accessors are generic over their closure's return type, which the
// chain parser could not express). see edit.md.
//
// the presence of `fm:context-set` in a composed node's Rust source is the
// linker's hook for Context::set_from_json() and the Clone impl the turn
// freeze below needs; it lives in edit.rs, so unticking this node removes both.

// the turn's frozen view of the context: a clone taken when the turn opens,
// dropped when it closes. thread-local because a turn belongs to one thread —
// one request thread on the server, the single wasm thread on the client.
thread_local! {
    static FM_CONTEXT_TURN: std::cell::RefCell<Option<Context>> =
        std::cell::RefCell::new(None);
}

// how many turns this thread has opened. Nesting is one turn, not many: the
// OUTERMOST freeze is the turn, and an inner begin/end pair rides it. Without
// this an inner begin re-froze from the live world (letting a foreign edit in
// mid-turn) and an inner end cleared the outer view (leaving the rest of the
// turn reading live). Nothing nests today; the day something does, it must
// nest honestly rather than quietly break the boundary law.
thread_local! {
    static FM_CONTEXT_DEPTH: std::cell::Cell<u32> = std::cell::Cell::new(0);
}

/// a nesting this deep is a runaway, not a design: said once, on the way past.
const FM_CONTEXT_DEPTH_LOUD: u32 = 8;

/// open a turn: freeze the live context into this thread's view. Everything
/// the turn reads through `with_context` sees this frozen value, so an edit
/// landing mid-turn — from this thread or any other — cannot be observed by
/// the event already in flight. Opening a turn inside a turn keeps the view
/// the outer one froze.
pub fn context_turn_begin() {
    let depth = FM_CONTEXT_DEPTH.with(|d| {
        let n = d.get() + 1;
        d.set(n);
        n
    });
    if depth > 1 {
        if depth == FM_CONTEXT_DEPTH_LOUD + 1 {
            eprintln!("miso: context: turns are nested {} deep — the frozen \
                       view is the outermost one's, but something is opening \
                       turns it does not close", depth);
        }
        return;
    }
    let cell = held_context();
    let frozen = cell
        .read()
        .unwrap_or_else(|p| p.into_inner())
        .clone();
    FM_CONTEXT_TURN.with(|t| {
        *t.borrow_mut() = Some(frozen);
    });
}

/// close a turn: drop the frozen view when the OUTERMOST turn closes. The next
/// turn re-freezes, and that is when edits made during this one become visible.
/// An end without a begin cannot go negative, and closes the view.
pub fn context_turn_end() {
    let depth = FM_CONTEXT_DEPTH.with(|d| {
        let n = d.get().saturating_sub(1);
        d.set(n);
        n
    });
    if depth > 0 {
        return;
    }
    FM_CONTEXT_TURN.with(|t| {
        *t.borrow_mut() = None;
    });
}

/// whether this thread is inside a turn.
pub fn in_context_turn() -> bool {
    FM_CONTEXT_TURN.with(|t| t.borrow().is_some())
}

/// read the context. Inside a turn this reads the frozen view; outside one
/// (startup, tooling) it reads a copy of the live value. No lock is ever held
/// while the caller's closure runs, so a read can never deadlock a writer.
pub fn with_context<R>(f: impl FnOnce(&Context) -> R) -> R {
    let mut f = Some(f);
    let framed = FM_CONTEXT_TURN.with(|t| {
        let held = t.borrow();
        match held.as_ref() {
            Some(ctx) => Some((f.take().unwrap())(ctx)),
            None => None,
        }
    });
    match framed {
        Some(r) => r,
        None => {
            let copy = held_context()
                .read()
                .unwrap_or_else(|p| p.into_inner())
                .clone();
            (f.take().unwrap())(&copy)
        }
    }
}

/// write the context. The closure runs under the write lock, so two concurrent
/// edits serialise and the last one stands. The lock is held for the closure's
/// duration: an `edit_context` closure must not call back into `with_context`
/// or `edit_context` (see edit.md, "the write lock is not re-entrant").
pub fn edit_context<R>(f: impl Fn(&mut Context) -> R) -> R {
    let out = {
        let cell = held_context();
        let mut live = cell.write().unwrap_or_else(|p| p.into_inner());
        f(&mut *live)
    };
    // read-your-own-writes: the same change is applied to this turn's frozen
    // view, so a later link in the SAME turn sees what an earlier one wrote.
    //
    // The boundary law is untouched by this and is the reason it is written
    // this way: the frozen view is not re-cloned from the live world (which
    // would let another device's edit in mid-turn) — the turn's OWN closure is
    // replayed against its OWN view. Foreign edits stay invisible until the
    // next turn; a turn's own edits stop being invisible to itself, which is
    // what read-modify-write across chain links requires. Migrating /ask found
    // this: two links appending to one list each read the frozen "[]" and the
    // second overwrote the first.
    FM_CONTEXT_MIRROR.with(|m| m.set(true));
    FM_CONTEXT_TURN.with(|t| {
        if let Some(view) = t.borrow_mut().as_mut() {
            let _ = f(view);
        }
    });
    FM_CONTEXT_MIRROR.with(|m| m.set(false));
    out
}

// true while the closure above is being replayed against the frozen view. The
// replay must change the value and NOTHING else: an op queued twice would put
// the same change on the wire twice under two different ids, which dedupe
// cannot collapse. Whatever queues ops consults this.
thread_local! {
    static FM_CONTEXT_MIRROR: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

pub fn in_context_mirror() -> bool {
    FM_CONTEXT_MIRROR.with(|m| m.get())
}

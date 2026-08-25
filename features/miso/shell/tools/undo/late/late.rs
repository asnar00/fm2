struct feature_Late;
impl feature_Late {
    // /undo records a step at its own link, scanning the outbox for the ops
    // this turn queued. Provenance puts every NEWER tool's update link
    // outside undo's — so a newer tool (cards, and every tool to come) writes
    // its ops after undo has already looked, and undo never sees them. The
    // snapshot undo takes is still right (it is taken before the outer links
    // write); only the scan is early. So: undo's own call stashes what it
    // knows, and this node — newest, therefore outermost — records at the
    // end of the whole update chain, when the outbox is complete.
    fn undo_record(state: String, before: serde_json::Value, from: usize, tool: String) -> String {
        let late = *FM_UNDO_LATE.lock().unwrap_or_else(|e| e.into_inner());
        if late {
            return existing.undo_record(state, before, from, tool);
        }
        *FM_UNDO_STASH.lock().unwrap_or_else(|e| e.into_inner()) = Some((before, from, tool));
        state
    }

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event);
        let stash = FM_UNDO_STASH.lock().unwrap_or_else(|e| e.into_inner()).take();
        match stash {
            Some((before, from, tool)) => {
                *FM_UNDO_LATE.lock().unwrap_or_else(|e| e.into_inner()) = true;
                let s = undo_record(state, before, from, tool);
                *FM_UNDO_LATE.lock().unwrap_or_else(|e| e.into_inner()) = false;
                s
            }
            None => state,
        }
    }
}

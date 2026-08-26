struct feature_OwnSlot;
impl feature_OwnSlot {
    // the broadcast slot lives with the world it belongs to. It was a fixed
    // /tmp path, so every miso on a machine — the deploy gate's rig, a
    // worker's rig, the dev server — shared one stream and talked over each
    // other (three failed gates on 2026-08-26). Under context_dir() each
    // state directory has its own; a handover's two processes share a
    // directory and so still share the slot, as they must.
    fn broadcast_file() -> String {
        let mine = format!("{}/broadcast.json", context_dir());
        // once, on the move: carry the old slot over so clients holding a
        // version number from it keep their place in the stream
        // — but only for the real state directory: a rig (MISO_CONTEXT_DIR set)
        // must start with an empty slot, not the machine's stale one, or it
        // inherits announcements meant for someone else's world
        if !std::path::Path::new(&mine).exists() && std::env::var("MISO_CONTEXT_DIR").is_err() {
            let old = existing.broadcast_file();
            if let Ok(raw) = std::fs::read_to_string(&old) {
                let _ = std::fs::create_dir_all(context_dir());
                let _ = std::fs::write(&mine, raw);
            }
        }
        mine
    }
}

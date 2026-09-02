struct feature_OneLevel;
impl feature_OneLevel {
    // ‹ means one level up, never "out". The tree already has a grammar for
    // one level — a tool's own button steps back (card page → set → home),
    // and /browse, /posts, /projects, /reports and /people each implement it
    // for their own `tool_<id>`. So this link does not navigate: it renames
    // the tap and hands `existing` the button ‹ stood for, which is why
    // nothing here writes `open_tool`. That var is bridged and /payload
    // republishes it at an older link, so a write from here would paint one
    // stale frame (misses.md, "navigation from the wrong side"); the
    // rewritten event moves the page at the links that own the move.
    fn update(state: String, event: String) -> String {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let is_click = e["type"].as_str().unwrap_or("") == "click";
        if is_click && e["ev"].as_str().unwrap_or("") == "tools_home" {
            let open = open_tool_read();
            let up = one_level_up(state.clone(), open.clone());
            if !up.is_empty() {
                let mut sent = e.clone();
                sent["ev"] = serde_json::json!(format!("tool_{}", up));
                let state = existing.update(state, sent.to_string());
                // the pop, only when the chain really climbed: /profile-first
                // drops navigation taps before the chain sees them, and a
                // dropped tap must not eat a level. A registry tool was never
                // on the stack, so only a nested one pops.
                if open_tool_read() == up
                    && one_level_nested(state.clone(), open) {
                    one_level_pop();
                }
                return state;
            }
            // a nested tool whose way in this node never saw: leave the event
            // alone and ‹ goes to the launcher, exactly as before this node
        }
        let was = open_tool_read();
        let state = existing.update(state, event);
        if is_click {
            one_level_note(state.clone(), was);
        }
        state
    }

    // the level above the open tool, as a tool id. A tool the registry names
    // is its own level above — its button is the step back. A tool the
    // registry does not name is nested, and the level above is the tool that
    // opened it. Empty means "no level above that this node knows".
    fn one_level_up(state: String, open: String) -> String {
        if open.is_empty() {
            return String::new();
        }
        if !one_level_nested(state, open.clone()) {
            return open;
        }
        let stack = one_level_read();
        match stack.last() {
            Some(id) => id.clone(),
            None => String::new(),
        }
    }

    // /current-only's test for a sub-tool, asked of the registry rather than
    // of the drawn row: a tool `tools_list` does not name is nested.
    fn one_level_nested(state: String, id: String) -> bool {
        let list: serde_json::Value = serde_json::from_str(&tools_list(state))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        for t in list.as_array().unwrap_or(&empty) {
            if t["id"].as_str().unwrap_or("") == id {
                return false;
            }
        }
        true
    }

    // after a tap that changed which tool is open: landing on a nested tool
    // records the tool left behind as its parent; landing anywhere else —
    // another registry tool, or the launcher — empties the stack, so it can
    // only ever hold the chain actually being stood in.
    fn one_level_note(state: String, was: String) {
        let now = open_tool_read();
        if now == was {
            return;
        }
        if now.is_empty() || !one_level_nested(state, now) {
            one_level_write(Vec::new());
            return;
        }
        if !was.is_empty() {
            one_level_push(was);
        }
    }

    // ---- the parent stack --------------------------------------------------

    fn one_level_read() -> Vec<String> {
        let raw = with_context(|c| c.one_level_parents_get());
        let v: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        v.as_array().unwrap_or(&empty).iter()
            .filter_map(|x| x.as_str().map(|s| s.to_string()))
            .collect()
    }

    // the closure handed to edit_context runs twice (live, then the frozen
    // view), so it clones rather than moves — /tools' own idiom.
    fn one_level_write(stack: Vec<String>) {
        let raw = serde_json::json!(stack).to_string();
        edit_context(|c| {
            let _ = c.edit_op("miso/shell/tools/ember/back/one-level",
                              "parents", serde_json::json!(raw.clone()));
        });
    }

    // capped at eight: a ninth level drops the oldest, the end furthest from
    // where the finger is, so the level about to be climbed to is always kept.
    fn one_level_push(id: String) {
        let mut stack = one_level_read();
        stack.push(id);
        while stack.len() > 8 {
            stack.remove(0);
        }
        one_level_write(stack);
    }

    fn one_level_pop() {
        let mut stack = one_level_read();
        stack.pop();
        one_level_write(stack);
    }

    // a relaunch starts at the launcher — /tools empties `open_tool` here, and
    // a remembered way in to a tool that is no longer open would be a lie.
    fn init() -> String {
        let state = existing.init();
        one_level_write(Vec::new());
        state
    }
}

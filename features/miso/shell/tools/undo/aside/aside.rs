struct feature_Aside;
impl feature_Aside {
    // the button only when there is something to undo. This is the newest
    // link on the controls chain, so every node that inserts in front of undo
    // has already run and found the button; taking it out here, last, leaves
    // their order intact and the row without an arrow that could do nothing.
    fn tool_controls(state: String) -> String {
        let html = existing.tool_controls(state);
        let open = open_tool_read();
        if open.is_empty() || undo_has(&open) {
            return html;
        }
        aside_strip(html)
    }

    // remove the whole undo button element from the row. The arrow inside is
    // an SVG, so the first </div> after the marker closes the button itself.
    fn aside_strip(html: String) -> String {
        match html.find("data-ev=\"ctx_undo\"") {
            Some(at) => match (html[..at].rfind("<div"), html[at..].find("</div>")) {
                (Some(start), Some(rel)) => format!("{}{}", &html[..start], &html[at + rel + 6..]),
                _ => html,
            },
            None => html,
        }
    }

    // the quiet seam: is this turn's edit NOT something the person did? Two
    // answers here. The undo press: /undo filed the inverse it minted as a
    // step of its own, so a second press redid and the stack never emptied
    // under the button; the ask wants the button gone when nothing is left.
    // A card the machine makes (CardEnsure — /me's blank profile on the first
    // 👤 open): the person opened a tool and did nothing, and an undo of it
    // would take their profile away. A later node that mints a card or a
    // value on the person's behalf extends this with its own event.
    fn undo_quiet(event: String) -> bool {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let kind = e["type"].as_str().unwrap_or("");
        if kind == "click" {
            return e["ev"].as_str().unwrap_or("") == "ctx_undo";
        }
        kind == "CardEnsure"
    }

    // the mark is held for the whole turn, outermost, so every inner link's
    // record call sees it.
    fn update(state: String, event: String) -> String {
        let quiet = undo_quiet(event.clone());
        if quiet {
            *FM_UNDO_QUIET.lock().unwrap_or_else(|e| e.into_inner()) = true;
        }
        let state = existing.update(state, event);
        if quiet {
            *FM_UNDO_QUIET.lock().unwrap_or_else(|e| e.into_inner()) = false;
        }
        state
    }

    fn undo_record(state: String, before: serde_json::Value, from: usize, tool: String) -> String {
        let quiet = *FM_UNDO_QUIET.lock().unwrap_or_else(|e| e.into_inner());
        if quiet {
            return state;
        }
        existing.undo_record(state, before, from, tool)
    }
}

struct feature_Undo;
impl feature_Undo {
    // every toolset's control row carries this, unconditionally: `tool_controls`
    // is only called while a tool is open, so registering without a tool-id test
    // is what "all toolsets" means. Dimmed when this tool has nothing to undo —
    // present, so the row does not change shape, and inert, because an undo of
    // nothing is not an action.
    fn tool_controls(state: String) -> String {
        let prev = existing.tool_controls(state.clone());
        let open = open_tool_read();
        if open.is_empty() {
            return prev;
        }
        let dim = if undo_has(&open) { "" } else { " dim" };
        format!("{}<div class=\"tool-button ctrl{}\" data-ev=\"ctx_undo\" title=\"undo\">\u{21b6}</div>",
                prev, dim)
    }

    // the whole feature, in one link of the update chain. Three acts in order:
    // remember what the world looked like before this event, let the event
    // happen, then read what it changed off the outbox and file it as one step.
    // An undo press is handled between the second and third, so the inverse it
    // issues is itself recorded — which is why pressing undo twice redoes.
    fn update(state: String, event: String) -> String {
        let open = open_tool_read();
        let watch = !open.is_empty();
        // the pre-event world. Taken through the frozen view, so it is exactly
        // what the turn is running under; taken only while a tool is open,
        // because that is the only time an undo could ever reach for it.
        let before = if watch {
            with_context(|c| c.snapshot())
        } else {
            serde_json::Value::Null
        };
        let sent = undo_outbox_len(state.clone());
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if watch && e["type"].as_str().unwrap_or("") == "click"
            && e["ev"].as_str().unwrap_or("") == "ctx_undo" {
            if let Some(step) = undo_take(&open) {
                undo_apply(step);
            }
        }
        // /converge drains the op outbox in ITS link, and this node is newer,
        // so every op minted at or beyond this point — this node's inverse, and
        // any sibling newer than converge, /square-taps today — would otherwise
        // wait for the next event. Shipping and stamping here puts them on the
        // wire in the turn that made them (undo.md, "the late link's ops").
        let state = ctx_stamp_outbox(ctx_ship_ops(state));
        // and the paint has to see it. /payload re-freezes the layer before the
        // render, but that link is inside this one, so an edit made out here
        // lands after the re-freeze and the frame shows the old number until
        // something else happens — which is invisible online, where the reply
        // arrives as another event, and very visible offline. Re-freezing after
        // a turn that produced ops is /payload's own move at the right moment.
        if undo_outbox_len(state.clone()) > sent {
            context_layer_begin();
        }
        if !watch {
            return state;
        }
        undo_record(state, before, sent, open)
    }

    // how many messages the outbox already held. What this turn added starts
    // here, which is a truthful boundary whatever the transport has or has not
    // managed to send.
    fn undo_outbox_len(state: String) -> usize {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let empty: Vec<serde_json::Value> = Vec::new();
        s["_send"].as_array().unwrap_or(&empty).len()
    }

    // file this turn's local var edits as one step. The ops are read off the
    // outbox rather than intercepted at the point of edit, because the outbox is
    // the one place every local edit ends up whatever feature made it — which is
    // what makes this work for tools that do not exist yet.
    fn undo_record(state: String, before: serde_json::Value, from: usize,
                   tool: String) -> String {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let empty: Vec<serde_json::Value> = Vec::new();
        let sent = s["_send"].as_array().unwrap_or(&empty).clone();
        let mut changes: Vec<serde_json::Value> = Vec::new();
        let mut i = from;
        while i < sent.len() {
            let m = sent[i].clone();
            i = i + 1;
            if m["type"].as_str().unwrap_or("") != "CtxOp" {
                continue;
            }
            let path = m["data"]["path"].as_str().unwrap_or("").to_string();
            let name = m["data"]["name"].as_str().unwrap_or("").to_string();
            if undo_skips(name.clone()) {
                continue;
            }
            // one entry per var: a turn that wrote the same var twice is still
            // one thing the person did, and the prior is the pre-event value.
            let mut seen = false;
            for c in changes.iter() {
                if c["path"].as_str().unwrap_or("") == path
                    && c["name"].as_str().unwrap_or("") == name {
                    seen = true;
                }
            }
            if seen {
                continue;
            }
            let rec = undo_var_before(before.clone(), path, name);
            if rec.is_null() {
                continue;
            }
            changes.push(rec);
        }
        if changes.is_empty() {
            return state;
        }
        undo_push(serde_json::json!({
            "tool": tool,
            "changes": serde_json::Value::Array(changes),
        }));
        state
    }

    // what undo declines to cover, and why. A chooser tick is not a toolset's
    // change — the chooser is a list of features, its ticks have their own clear
    // meaning, and an undo that silently switched a feature back on would be
    // answering a question nobody asked. Navigation is how you got to the tool
    // rather than something you did in it.
    fn undo_skips(name: String) -> bool {
        name == "enabled" || name == "open_tool" || name == "tools_catalog"
    }

    // find a var in the pre-event snapshot and describe how to put it back:
    // its merge (which verb the inverse speaks), its scope (which world owns
    // it), and its resolved value before the edit. A var whose merge has no
    // reversible write is left out, so a step never promises what it cannot do.
    fn undo_var_before(before: serde_json::Value, path: String,
                       name: String) -> serde_json::Value {
        let empty: Vec<serde_json::Value> = Vec::new();
        for v in before.as_array().unwrap_or(&empty) {
            if v["path"].as_str().unwrap_or("") != path {
                continue;
            }
            if v["name"].as_str().unwrap_or("") != name {
                continue;
            }
            let merge = v["merge"].as_str().unwrap_or("").to_string();
            if merge != "last-write" && merge != "counter" {
                return serde_json::Value::Null;
            }
            // `resolved` is what a reader would actually have got — the overlay
            // chain's answer, which for a global var is the layer's and not this
            // world's unread field.
            let prior = if v["resolved"].is_null() {
                v["value"].clone()
            } else {
                v["resolved"].clone()
            };
            return serde_json::json!({
                "path": path, "name": name, "merge": merge,
                "scope": v["scope"].clone(), "prior": prior,
            });
        }
        serde_json::Value::Null
    }

    // put one step back, as REAL ops through the ordinary door: the same
    // edit_op / edit_reset every tool uses, so an undo syncs, logs, dedupes and
    // relays exactly as the edit it reverses did. Nothing here knows what a tap
    // is; it knows what a merge is.
    fn undo_apply(step: serde_json::Value) {
        let empty: Vec<serde_json::Value> = Vec::new();
        let changes = step["changes"].as_array().unwrap_or(&empty).clone();
        for ch in changes {
            let path = ch["path"].as_str().unwrap_or("").to_string();
            let name = ch["name"].as_str().unwrap_or("").to_string();
            let merge = ch["merge"].as_str().unwrap_or("").to_string();
            let global = ch["scope"].as_str().unwrap_or("") == "global";
            let prior = ch["prior"].clone();
            if merge == "counter" {
                // a counter's inverse is a reset to the prior sum: it opens a
                // new epoch, which is what makes the fleet converge on it the
                // way it converges on zero (converge.md argues the direction).
                // The prior is read back through Counter's own Deserialize
                // rather than by indexing the JSON, so the wire shape of a
                // Counter stays the library's business.
                let was: Counter = serde_json::from_value(prior.clone())
                    .unwrap_or(Counter::zero());
                let n = serde_json::json!(was.sum);
                if global {
                    edit_layer(|c| {
                        let _ = c.edit_reset(&path, &name, n.clone());
                    });
                } else {
                    edit_context(|c| {
                        let _ = c.edit_reset(&path, &name, n.clone());
                    });
                }
            } else {
                if global {
                    edit_layer(|c| {
                        let _ = c.edit_op(&path, &name, prior.clone());
                    });
                } else {
                    edit_context(|c| {
                        let _ = c.edit_op(&path, &name, prior.clone());
                    });
                }
            }
        }
    }
}

struct feature_Delete;
impl feature_Delete {
    // ---- the test everything else asks ------------------------------------
    // a tombstone is a card with `deleted` stamped on it. Written once here so
    // no surface has to know the shape of the field.

    fn delete_gone(card: &serde_json::Value) -> bool {
        card["deleted"].as_u64().unwrap_or(0) > 0
    }

    // the emptying. What is left is the smallest thing that can still say "this
    // card was here and is finished": the id, the owner, the type, the times,
    // and one empty title block so the object keeps its shape. The words, the
    // picture and the location block (/location keeps a place as a block of the
    // body) leave the world at the moment of the tap — a tombstone is not a
    // hidden card, it is an emptied one.
    fn delete_tombstone(card: &serde_json::Value, now: u64) -> serde_json::Value {
        let mut out = card.clone();
        out["blocks"] = serde_json::json!([ { "kind": "title", "text": "" } ]);
        out["links"] = serde_json::json!([]);
        out["deleted"] = serde_json::json!(now);
        out["edited"] = serde_json::json!(now);
        out
    }

    // ---- the op ------------------------------------------------------------
    // CardDelete {id, t}: type-agnostic on purpose — the day a project or a
    // profile asks to be deleted, this event already does it and only a button
    // is missing. Owned cards only: a copy carries `from` (/exchange) and is
    // not yours to delete, which is decided here rather than by comparing
    // names, because the logged-in name is not in the world at all.

    fn update(state: String, event: String) -> String {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "CardDelete" {
            return existing.update(state, event);
        }
        // the pre-event world, taken BEFORE anything is written — the same
        // instant /undo takes its own snapshot, one link further in. See
        // `delete_record` for why this node records its own step.
        let before = with_context(|c| c.snapshot());
        let tool = open_tool_read();
        let state = existing.update(state, event.clone());
        let id = e["data"]["id"].as_str().unwrap_or("").to_string();
        let now = e["data"]["t"].as_u64().unwrap_or(0);
        if id.is_empty() || now == 0 {
            return state;
        }
        let mut list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        if !list.is_array() {
            list = serde_json::json!([]);
        }
        let mut changed = false;
        for c in list.as_array_mut().expect("cards is an array").iter_mut() {
            if c["id"].as_str().unwrap_or("") != id {
                continue;
            }
            if !c["from"].is_null() {
                println!("cards: refused a delete of {} — that card is a copy", id);
                continue;
            }
            if delete_gone(c) {
                continue;
            }
            *c = delete_tombstone(c, now);
            changed = true;
        }
        if !changed {
            return state;
        }
        cards_write(list.to_string());
        // the page you were reading is gone, so the surface goes back to the
        // set — and /plus-at-home puts the + back, which it takes away while a
        // card is open. A device-scoped write, so nothing goes on the wire.
        if browse_open_read() == id {
            browse_open_write(String::new());
        }
        delete_record(before, tool);
        state
    }

    // /undo scans the outbox for what a turn wrote, and /undo/late moved that
    // scan to the end of the chain — while noting, in its own hostile cases,
    // that "the pattern holds only while this is the outermost update link".
    // This node is newer than /late, so this write lands after the scan and
    // undo would never see it. Rather than reorder another node for one ask,
    // the step is filed here through /undo's own two library calls, with the
    // prior value read out of the snapshot taken at the top of this link.
    // The general fix belongs to /undo (every node newer than /late has this,
    // /kinds/new included) and is named in this node's spec.
    fn delete_record(before: serde_json::Value, tool: String) {
        if tool.is_empty() {
            return;
        }
        let rec = undo_var_before(before, "miso/loop/cards".to_string(),
                                  "cards".to_string());
        if rec.is_null() {
            return;
        }
        undo_push(serde_json::json!({
            "tool": tool,
            "changes": [rec]
        }));
    }

    // ---- nowhere to be seen ------------------------------------------------
    // three seams, and the map follows for nothing: /map draws whatever set
    // browse_set_html is handed.

    fn browse_cards(state: String) -> String {
        delete_sift(existing.browse_cards(state))
    }

    fn posts_set() -> Vec<serde_json::Value> {
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in existing.posts_set().iter() {
            if !delete_gone(c) {
                out.push(c.clone());
            }
        }
        out
    }

    // the lookup a consumer asks with: a deleted profile is not the profile.
    // Not reachable today — the button is on posts only — and here because the
    // op is type-agnostic and the next type to ask will meet this first.
    fn card_of_type(list: String, owner: String, kind: String) -> String {
        existing.card_of_type(delete_sift(list), owner, kind)
    }

    fn delete_sift(list: String) -> String {
        let v: serde_json::Value = serde_json::from_str(&list)
            .unwrap_or(serde_json::Value::Null);
        if !v.is_array() {
            return list;
        }
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in v.as_array().unwrap_or(&empty) {
            if !delete_gone(c) {
                out.push(c.clone());
            }
        }
        serde_json::Value::Array(out).to_string()
    }

    // a tombstone drawn as a page anyway — a card held on screen while the
    // delete arrives from another device. One dim line rather than an empty
    // card pretending to be editable.
    fn card_page_html(card: String) -> String {
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        if !delete_gone(&c) {
            return existing.card_page_html(card);
        }
        String::from("<div class=\"card-page deleted\"><div class=\"card-gone\">deleted</div></div>")
    }

    // ---- the control -------------------------------------------------------
    // on a post of your own, and nowhere else. In front of /undo's button
    // through /posts' own inserter (/glyphs — undo stays last in every row),
    // wearing the posts tool's colour rather than undo's blue.

    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state);
        if open_tool_read() != "posts" {
            return row;
        }
        let open = browse_open_read();
        if open.is_empty() || !delete_own_post(open) {
            return row;
        }
        posts_before_undo(row, delete_button())
    }

    // is the card on screen a post of your own that is still there? `from` is
    // /exchange's mark on a copy and the whole read-only test.
    fn delete_own_post(id: String) -> bool {
        let v: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in v.as_array().unwrap_or(&empty) {
            if c["id"].as_str().unwrap_or("") != id {
                continue;
            }
            return posts_is(c) && c["from"].is_null() && !delete_gone(c);
        }
        false
    }

    fn delete_button() -> String {
        let colour = tool_colour("posts".to_string());
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        format!("<div class=\"tool-button ctrl{}\" data-ev=\"posts_delete\" title=\"delete\">{}</div>",
                tint, delete_bin_svg())
    }

    // drawn, in currentColor, per /glyphs: a bin — lid, body, two lines — with
    // the same rounded strokes the plus and the bubble wear.
    fn delete_bin_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M4 7h16\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linecap=\"round\"/>",
            "<path d=\"M9.5 7V5.4A1.4 1.4 0 0 1 10.9 4h2.2a1.4 1.4 0 0 1 1.4 1.4V7\" ",
            "fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linejoin=\"round\"/>",
            "<path d=\"M6.6 7.6l0.9 11.6A2 2 0 0 0 9.5 21h5a2 2 0 0 0 2-1.8l0.9-11.6\" ",
            "fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linejoin=\"round\"/>",
            "<path d=\"M10.4 11v6M13.6 11v6\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2\" stroke-linecap=\"round\"/>",
            "</svg>"))
    }
}

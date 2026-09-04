struct feature_InPlace;
impl feature_InPlace {
    // ---- the list opens in the row, not below it -----------------------------
    // /armed made the publish level a level of its own: `tool_level` descended,
    // the row became ‹ and the lit sliders, and the page took the screen. Ash
    // asked for the list to pop up in the toolbar you are already in (#p31).
    //
    // The tree of tools still holds — the picker is the sub-tool's own popover
    // and there are no buttons on a page — but a popover is not a level, so
    // nothing here writes `open_tool` and nothing descends. Which is also why
    // /tools needed no seam: /armed opened two of its own (`armed_level_ev`,
    // `armed_level_lit`), and this node answers them.
    //
    // Open is a flag on the turn's STATE, not a var: /one-add's own idiom for
    // exactly this — a strip you come back to and a tap that closes it — and
    // it costs no op and cannot outlive a relaunch.

    fn armed_level_ev() -> String {
        "armed_pick".to_string()
    }

    fn armed_level_lit(state: String) -> bool {
        in_place_open(state)
    }

    fn in_place_open(state: String) -> bool {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["armed_picking"].as_bool().unwrap_or(false)
    }

    // ---- the taps -------------------------------------------------------------
    // Four rules, and three of them are one rule: anything that is not the
    // sliders closes the popover, which is /one-add's rule for its own picker.
    //
    // ‹ is the exception that has to be caught BEFORE the chain: /one-level
    // would climb out of the recording row, and the ask is that the row stays
    // where it was. So while the popover is open, ‹ means "close it" and the
    // event is never handed down — the second ‹ climbs as it always did.
    //
    // The flag is read off the state coming IN and written onto the state going
    // out, so a tap that both closes the popover and does something else (rec,
    // the camera) does both.
    fn update(state: String, event: String) -> String {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return existing.update(state, event);
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        let was = in_place_open(state.clone());
        if ev == "tools_home" && was {
            let mut s: serde_json::Value = serde_json::from_str(&state)
                .unwrap_or(serde_json::json!({}));
            s["armed_picking"] = serde_json::json!(false);
            return s.to_string();
        }
        let state = existing.update(state, event);
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if ev == "armed_pick" {
            // a second tap on the sliders puts it away, the way every
            // two-faced control here does
            s["armed_picking"] = serde_json::json!(!was);
            return s.to_string();
        }
        if was {
            s["armed_picking"] = serde_json::json!(false);
        }
        s.to_string()
    }

    // ---- the popover -----------------------------------------------------------
    // /armed's own list, in a box that sits on the row rather than a page that
    // replaces the screen. The page it drew for the `level` tool is left where
    // it is and simply never reached — untick this node and the level comes
    // back exactly as it was.
    //
    // Guarded on the recording row being the open one: the flag lives on the
    // state and nothing else clears it, so a frame painted at another level
    // must not find a popover in it.
    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        if !in_place_open(state) || open_tool_read() != "record" {
            return base;
        }
        format!("{}<div class=\"armed-pop\">{}</div>", base, armed_level_row())
    }
}

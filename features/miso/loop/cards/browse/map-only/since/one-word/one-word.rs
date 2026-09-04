struct feature_OneWord;
impl feature_OneWord {
    // ---- the slot holds one word --------------------------------------------
    // /since put four pills where the view picker was, and four words take 175
    // of the strip's 402 points where three glyphs took 96 — enough that
    // /title's project name had to be pinned into what was left. Ash asked for
    // the slot to show only the chosen filter and to drop the list on a tap.
    //
    // The pill keeps `.since-pill`'s look and class, so /since's stylesheet,
    // its long-press arming and its swallow all still apply, and this node
    // adds a shape rather than a second grammar.

    fn browse_slot_html() -> String {
        format!("<div class=\"since-pills\"><div class=\"since-pill since-on since-one\" data-ev=\"since_pick\" title=\"when\">{}</div></div>",
                one_word_now())
    }

    // the chosen word, and `all` for anything the var has never been set to —
    // the same reading /since's own row makes, so an empty var reads `all`.
    fn one_word_now() -> String {
        let p = since_period_read();
        if p == "today" || p == "week" || p == "month" {
            return p;
        }
        "all".to_string()
    }

    // ---- open is a flag on the turn's state ---------------------------------
    // /in-place's idiom for exactly this shape — a popover you come back from,
    // closed by the next tap — and its consequences are that idiom's: no op on
    // the wire, nothing stored, and it cannot outlive a relaunch. A popover is
    // not a level of the tree of tools, so nothing here writes `open_tool`.

    fn one_word_open(state: String) -> bool {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["since_picking"].as_bool().unwrap_or(false)
    }

    // Four rules, three of which are one rule: anything that is not the pill
    // closes the column — including a pick, which is what closes it after you
    // have chosen. ‹ is caught BEFORE the chain, so it closes the column and
    // does not also climb a level; the second ‹ climbs as it always did.
    //
    // The flag is read off the state coming in and written on the state going
    // out, so a tap that both closes the column and does something else does
    // both.
    fn update(state: String, event: String) -> String {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return existing.update(state, event);
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        let was = one_word_open(state.clone());
        if ev == "tools_home" && was {
            let mut s: serde_json::Value = serde_json::from_str(&state)
                .unwrap_or(serde_json::json!({}));
            s["since_picking"] = serde_json::json!(false);
            return s.to_string();
        }
        let state = existing.update(state, event);
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if ev == "since_pick" {
            // a second tap on the word puts it away, as every two-faced
            // control here does
            s["since_picking"] = serde_json::json!(!was);
            return s.to_string();
        }
        if was {
            s["since_picking"] = serde_json::json!(false);
        }
        s.to_string()
    }

    // ---- the column ----------------------------------------------------------
    // /since's own four, one to a line, hanging under the word they came from.
    // Drawn from `render` rather than from the slot because the slot takes no
    // state and open lives on the state.
    //
    // No guard on which tool is open is needed: the flag can only be set by a
    // tap on the word, which is only drawn where the slot is, and any other
    // click — `tool_reports` included — clears it on the same turn, before
    // this runs.
    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        if !one_word_open(state) {
            return base;
        }
        let p = one_word_now();
        format!("{}<div class=\"since-drop\">{}{}{}{}</div>",
                base,
                since_pill("today".to_string(), p == "today"),
                since_pill("week".to_string(), p == "week"),
                since_pill("month".to_string(), p == "month"),
                since_pill("all".to_string(), p == "all"))
    }
}

struct feature_ProfileFirst;
impl feature_ProfileFirst {
    // the predicate seam: is this card enough to start with? A picture
    // with data and a text block with words. A later ask ("a number too")
    // redefines this and adds its clause.
    fn profile_first_missing(card: String) -> bool {
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut pic = false;
        let mut line = false;
        for b in c["blocks"].as_array().unwrap_or(&empty) {
            let kind = b["kind"].as_str().unwrap_or("");
            if kind == "picture" && !b["data"].as_str().unwrap_or("").is_empty() {
                pic = true;
            }
            if kind == "text" && !b["text"].as_str().unwrap_or("").trim().is_empty() {
                line = true;
            }
        }
        !(pic && line)
    }

    // the gate's question, asked only of a world that has really arrived:
    // /veil's _joined is set on the join and never on its timeout, so a page
    // that could not reach the server is not gated — the failure direction
    // is "no gate", never "locked out". No profile card yet is a new person.
    fn profile_first_gated(state: String) -> bool {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if !s["_joined"].as_bool().unwrap_or(false) {
            return false;
        }
        let card = card_of_type(cards_read(), String::new(), "profile".to_string());
        card.is_empty() || profile_first_missing(card)
    }

    fn profile_first_own_id() -> String {
        let card = card_of_type(cards_read(), String::new(), "profile".to_string());
        if card.is_empty() {
            return String::new();
        }
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        c["id"].as_str().unwrap_or("").to_string()
    }

    // no way past: while gated, a navigation tap that would step off the
    // card is dropped BEFORE the chain sees it, so nothing repaints and
    // nothing is written. The two taps that lead TO the card still pass —
    // 👤 while another tool (or none) is open, and the own tile — because
    // they are the events the page half sends to bring a fresh person to
    // their card (/restore's idiom: the same event a finger would send).
    //
    // Dropping rather than writing back is deliberate: open_tool is a
    // bridged var and /payload republishes it at its own, older, link, so a
    // write made here would paint one stale frame (/turn-end names this)
    // and a republish from here would read to /one-way as a page write.
    fn update(state: String, event: String) -> String {
        if profile_first_gated(state.clone()) && profile_first_steps_off(event.clone()) {
            return state;
        }
        existing.update(state, event)
    }

    // which clicks are navigation away from the card. /tools' tool_ and
    // tools_home, and /browse's view and open events; everything else — the
    // card's own edits, the panel, the picture — is not this node's business.
    fn profile_first_steps_off(event: String) -> bool {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return false;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        if ev == "tool_account" {
            return open_tool_read() == "account";
        }
        if let Some(id) = ev.strip_prefix("browse_open:") {
            return id != profile_first_own_id();
        }
        ev == "tools_home" || ev.starts_with("tool_") || ev.starts_with("browse_")
    }

    // the sentence, inside the card page above the name; after the base
    // when the page is not there yet (/me's "making your card…" moment).
    // Its id is the marker the page half reads.
    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        if !profile_first_gated(state) {
            return base;
        }
        let line = "<div id=\"profileFirst\" class=\"profile-first\">add a picture and a line about you to start</div>";
        if let Some(at) = base.find("class=\"card-page\"") {
            if let Some(end) = base[at..].find('>') {
                let cut = at + end + 1;
                return format!("{}{}{}", &base[..cut], line, &base[cut..]);
            }
        }
        format!("{}{}", base, line)
    }
}

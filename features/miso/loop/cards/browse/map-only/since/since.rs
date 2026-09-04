struct feature_Since;
impl feature_Since {
    // ---- the two vars ------------------------------------------------------
    // read through the live context rather than the bridged loop state, for
    // /browse's own reason: /payload republishes part-way down the update
    // chain and this node's links sit outside it, so `s.<key>` in a render
    // that follows this node's own write would be one turn stale.

    fn since_period_read() -> String {
        with_context(|c| c.since_period_get())
    }

    fn since_period_write(period: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/cards/browse/map-only/since", "period",
                              serde_json::json!(period.clone()));
        });
    }

    fn since_marks_read() -> String {
        with_context(|c| c.since_day_starts_get())
    }

    fn since_marks_write(marks: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/cards/browse/map-only/since",
                              "day_starts", serde_json::json!(marks.clone()));
        });
    }

    // ---- the pills ---------------------------------------------------------
    // /map-only emptied the picker's slot and left this seam for whatever takes
    // it. Four pills in /audience's grade-pill grammar: a small closed set,
    // round, dim, one lit in the accent that already means CHOSEN (/taste 3).

    fn browse_slot_html() -> String {
        let p = since_period_read();
        format!("<div class=\"since-pills\">{}{}{}{}</div>",
                since_pill("today".to_string(), p == "today"),
                since_pill("week".to_string(), p == "week"),
                since_pill("month".to_string(), p == "month"),
                since_pill("all".to_string(), p != "today" && p != "week" && p != "month"))
    }

    // the four data-ev strings are written out rather than formatted, because
    // /sub-tool-cards' and /tool-words' long press read them out of this source
    // to name the control a finger is held on, and skip any that carry a
    // format placeholder.
    fn since_pill(which: String, on: bool) -> String {
        let lit = if on { " since-on" } else { "" };
        if which == "today" {
            return format!("<div class=\"since-pill{}\" data-ev=\"since_today\" title=\"today\">today</div>", lit);
        }
        if which == "week" {
            return format!("<div class=\"since-pill{}\" data-ev=\"since_week\" title=\"week\">week</div>", lit);
        }
        if which == "month" {
            return format!("<div class=\"since-pill{}\" data-ev=\"since_month\" title=\"month\">month</div>", lit);
        }
        format!("<div class=\"since-pill{}\" data-ev=\"since_all\" title=\"all\">all</div>", lit)
    }

    // ---- the events --------------------------------------------------------
    // a pill writes the period and leaves `open` alone: the picker cleared it
    // because a mode you cannot see is a control doing nothing, but narrowing
    // the set behind a card you are reading is visible the moment you go back.
    //
    // SinceMarks is the page half's answer to "when was local midnight" — an
    // event, the way a finger sends one, not a write to a bridged key
    // (misses.md, navigation from the wrong side).

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let kind = e["type"].as_str().unwrap_or("").to_string();
        if kind == "SinceMarks" {
            let marks = e["data"]["marks"].as_str().unwrap_or("").to_string();
            if !marks.is_empty() && marks != since_marks_read() {
                since_marks_write(marks);
            }
            return state;
        }
        if kind != "click" {
            return state;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        if ev == "since_today" || ev == "since_week" || ev == "since_month"
            || ev == "since_all" {
            since_period_write(ev["since_".len()..].to_string());
        }
        state
    }

    // ---- the cut -----------------------------------------------------------
    // one epoch millisecond floor, and 0 means keep everything. 0 is the answer
    // for `all`, for a period whose mark is missing, and for marks that have
    // not arrived — a filter that has not been told the time shows you your
    // world rather than an empty map.

    fn since_cut() -> u64 {
        let p = since_period_read();
        let at = if p == "today" {
            0
        } else if p == "week" {
            1
        } else if p == "month" {
            2
        } else {
            return 0;
        };
        let marks = since_marks_read();
        let parts: Vec<String> = marks.split(',').map(|s| s.to_string()).collect();
        if parts.len() < 3 {
            return 0;
        }
        parts[at].parse::<u64>().unwrap_or(0)
    }

    // the card's own moment: a post's `when` if it has one, and otherwise the
    // moment the card was made. The field is read straight off the card rather
    // than through /post-time's function, so this keeps working with that node
    // unticked — no card then carries a `when` it wrote, and everything falls
    // back to `created`, which is what the ask says a person and a project
    // count by anyway.
    fn since_time_of(card: &serde_json::Value) -> u64 {
        let w = card["when"].as_u64().unwrap_or(0);
        if w > 0 {
            return w;
        }
        card["created"].as_u64().unwrap_or(0)
    }

    // the test, with the two exemptions the spec names: your own profile card
    // (a list of people that does not contain you is somebody else's list),
    // and the card that is open (narrow the band behind it, do not close it).
    fn since_keep(card: &serde_json::Value) -> bool {
        let cut = since_cut();
        if cut == 0 {
            return true;
        }
        if card["type"].as_str().unwrap_or("") == "profile"
            && card["from"].is_null() {
            return true;
        }
        // the open card, but only while a tool is actually drawing it: `open`
        // outlives the tool that set it (/new writes it from the launcher, and
        // nothing clears it until the next tool tap), and an exemption that
        // did not check would leak one card into every surface's set for as
        // long as it sat there. Caught on the rig, 2026-09-04.
        let id = card["id"].as_str().unwrap_or("").to_string();
        if !id.is_empty() && id == browse_open_read()
            && !open_tool_read().is_empty() {
            return true;
        }
        since_time_of(card) >= cut
    }

    // ---- the two chains that carry a browsed set ----------------------------
    // /browse's seam (which /people and /projects take) and /posts' own set.
    // Narrowing here is upstream of everything that draws: the map's pins, the
    // band's data-ids, /on-people-map's data-post-ids and /flick's walk all
    // read what these two return, so they agree without being told and /reel's
    // own contract — the band lists the map's set — is untouched.

    fn browse_cards(state: String) -> String {
        let list: serde_json::Value =
            serde_json::from_str(&existing.browse_cards(state))
                .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in list.as_array().unwrap_or(&empty) {
            if since_keep(c) {
                out.push(c.clone());
            }
        }
        serde_json::Value::Array(out).to_string()
    }

    fn posts_set() -> Vec<serde_json::Value> {
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in existing.posts_set().iter() {
            if since_keep(c) {
                out.push(c.clone());
            }
        }
        out
    }
}

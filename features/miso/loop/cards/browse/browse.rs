struct feature_Browse;
impl feature_Browse {
    // ---- navigation, held on the device ----------------------------------
    // which view, and which card, are where-you-are and not what-you-own, so
    // both are device-scoped vars — the same declaration /tools gives
    // open_tool, and the same consequence: the write queues no op.
    //
    // every read here goes to the live context rather than to the bridged
    // loop state. /payload republishes part-way down the update chain and
    // this node's links sit outside it, so `s.open_tool` in a render that
    // follows this node's own write would be one turn stale.

    fn browse_view_read() -> String {
        with_context(|c| c.browse_view_get())
    }

    fn browse_view_write(view: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/cards/browse", "view",
                              serde_json::json!(view.clone()));
        });
    }

    fn browse_open_read() -> String {
        with_context(|c| c.browse_open_get())
    }

    fn browse_open_write(id: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/cards/browse", "open",
                              serde_json::json!(id.clone()));
        });
    }

    // ---- the toolbar ------------------------------------------------------

    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        let mut list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        if let Some(arr) = list.as_array_mut() {
            arr.push(serde_json::json!({
                "id": "cards", "label": "cards", "icon": browse_stack_svg() }));
        }
        list.to_string()
    }

    // ---- the view picker --------------------------------------------------
    // not a control-row sub-tool: the picker sits at the top left of the
    // screen, level with the lozenge, because that is where ash put it — and
    // he named a third view (a map) as its next member in the same breath. It
    // is furniture of the cards tool: the render chain draws it while the tool
    // is open, and nothing else on the page knows it exists.

    fn browse_picker_html() -> String {
        format!("<div class=\"browse-picker\">{}</div>", browse_views())
    }

    // the seam a third view joins at: redefine this, call existing, append one
    // more button. A member costs one link and no layout — the pill is a flex
    // row that grows to hold whatever it is given.
    fn browse_views() -> String {
        let view = browse_view_read();
        format!("{}{}",
                browse_view_button("grid".to_string(), view != "list"),
                browse_view_button("list".to_string(), view == "list"))
    }

    // lit for the view you are in, dim for the one you are not (/taste 2 —
    // hierarchy is dimness), and the lit one wears the one accent that
    // already means CHOSEN everywhere else (/taste 3).
    // the two data-ev strings are written out rather than formatted, because
    // /sub-tool-cards' long-press reads them out of this source to name the
    // control it is held on, and skips any that carry a format placeholder.
    fn browse_view_button(which: String, on: bool) -> String {
        let lit = if on { " browse-on" } else { "" };
        if which == "list" {
            return format!("<div class=\"browse-view{}\" data-ev=\"browse_list\" title=\"list\">{}</div>",
                           lit, browse_list_svg());
        }
        format!("<div class=\"browse-view{}\" data-ev=\"browse_grid\" title=\"grid\">{}</div>",
                lit, browse_grid_svg())
    }

    // ---- the events -------------------------------------------------------

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        // picking a view also puts the set back: the picker is on screen while
        // a card is open too, and a mode switch that changed a mode you cannot
        // see would be a control doing nothing.
        if ev == "browse_grid" || ev == "browse_list" {
            browse_view_write(ev["browse_".len()..].to_string());
            if !browse_open_read().is_empty() {
                browse_open_write(String::new());
            }
            return state;
        }
        if let Some(id) = ev.strip_prefix("browse_open:") {
            browse_open_write(id.to_string());
            return state;
        }
        // the way back, one level at a time. /tools has already run and, for
        // `tool_cards`, has closed the tool; with a card showing, that tap
        // means "back to the set" instead, so the tool is re-opened here.
        if ev == "tool_cards" {
            if !browse_open_read().is_empty() {
                browse_open_write(String::new());
                open_tool_write("cards".to_string());
            }
            return state;
        }
        // leaving by any other route puts the set back, so the tool always
        // opens where it started
        if (ev == "tools_home" || ev.starts_with("tool_"))
            && !browse_open_read().is_empty() {
            browse_open_write(String::new());
        }
        state
    }

    // ---- the display surface ----------------------------------------------

    // the seam for WHICH cards this surface draws: the whole set you hold,
    // in the order the world holds it. A node that re-aims the surface at a
    // subset — /people, and the project surface after it — redefines this
    // and returns its own list; the default is unchanged.
    fn browse_cards(state: String) -> String {
        let _ = state;
        cards_read()
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        if open_tool_read() != "cards" {
            return base;
        }
        let list: serde_json::Value = serde_json::from_str(&browse_cards(state))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let cards = list.as_array().unwrap_or(&empty);
        let picker = browse_picker_html();
        let open = browse_open_read();
        if !open.is_empty() {
            for c in cards.iter() {
                if c["id"].as_str().unwrap_or("") == open {
                    return format!("{}{}{}", base, picker,
                                   card_page_html(c.to_string()));
                }
            }
            // the card is gone — deleted elsewhere, or a world that arrived
            // without it. The set is the honest fallback, silently.
        }
        format!("{}{}{}", base, picker, browse_set_html(cards))
    }

    fn browse_set_html(cards: &Vec<serde_json::Value>) -> String {
        if cards.is_empty() {
            return String::from(
                "<div class=\"browse-empty\">no cards yet</div>");
        }
        if browse_view_read() == "list" {
            return browse_list_html(cards);
        }
        browse_grid_html(cards)
    }

    // the grid: /cards' own tile in /cards' own grid, each one wrapped in the
    // element that carries the tap. The wrapper rather than a splice into the
    // tile's markup, so a later node may restyle the tile freely.
    fn browse_grid_html(cards: &Vec<serde_json::Value>) -> String {
        let mut out = String::from("<div class=\"card-tiles browse-grid\">");
        for c in cards.iter() {
            let id = card_esc(c["id"].as_str().unwrap_or("").to_string());
            out.push_str(&format!(
                "<div class=\"browse-tap\" data-ev=\"browse_open:{}\">{}</div>",
                id, card_tile_html(c.to_string())));
        }
        out.push_str("</div>");
        out
    }

    // the list: the house .crow grammar (/taste 6) — the type where the
    // number sits, the title, the edited-when dim at the right.
    fn browse_list_html(cards: &Vec<serde_json::Value>) -> String {
        let mut newest = 0u64;
        for c in cards.iter() {
            let t = browse_when_of(c);
            if t > newest {
                newest = t;
            }
        }
        let this_year = browse_year(newest);
        let mut out = String::from("<div class=\"browse-list\">");
        for c in cards.iter() {
            let id = card_esc(c["id"].as_str().unwrap_or("").to_string());
            let kind = browse_row_left(c);
            let title = browse_title_of(c);
            let when = browse_when(browse_when_of(c), this_year);
            out.push_str(&format!(
                "<div class=\"crow browse-row\" data-ev=\"browse_open:{}\"><span class=\"cnum browse-type\">{}</span><div class=\"ctext browse-title\">{}</div><span class=\"browse-when\">{}</span></div>",
                id, kind, title, when));
        }
        out.push_str("</div>");
        out
    }

    // the seam for WHICH of a card's times its row shows. The default is
    // `edited`, which is what the set of everything you hold wants: the row
    // says when you last touched it. A card type whose date means something
    // else — a post is dated by the moment it records, not the moment it was
    // typed into — redefines this and says so. The default is unchanged, so
    // with no one redefining it every row reads exactly as it did.
    fn browse_when_of(card: &serde_json::Value) -> u64 {
        card["edited"].as_u64().unwrap_or(0)
    }

    // the seam for the left cell of a list row — where /taste 6 puts the
    // number. The default is the card's type, which is what the set of
    // everything you hold wants; a surface whose cards are all one type
    // redefines this and says something less redundant instead.
    fn browse_row_left(card: &serde_json::Value) -> String {
        card_esc(card["type"].as_str().unwrap_or("").to_string())
    }

    // the first title block's text, escaped. An untitled card gets nothing —
    // no invented placeholder, because it should look untitled.
    fn browse_title_of(card: &serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") == "title" {
                return card_esc(b["text"].as_str().unwrap_or("").to_string());
            }
        }
        String::new()
    }

    // ---- when, without a clock --------------------------------------------
    // a wasm build has no local time zone and no SystemTime, and `render`
    // carries no event time, so the date is arithmetic on the stored epoch
    // milliseconds, in UTC. Near midnight in summer that is a day out; a
    // relative "3h ago" needs the current time and is the later rung.

    fn browse_when(ms: u64, this_year: i64) -> String {
        if ms == 0 {
            return String::new();
        }
        let days = (ms / 86400000) as i64;
        let year = browse_civil_year(days);
        let month = browse_civil_month(days);
        let day = browse_civil_day(days);
        let names = ["jan", "feb", "mar", "apr", "may", "jun",
                     "jul", "aug", "sep", "oct", "nov", "dec"];
        let name = names[(month - 1) as usize];
        if year == this_year {
            return format!("{} {}", day, name);
        }
        format!("{} {} {}", day, name, year)
    }

    fn browse_year(ms: u64) -> i64 {
        if ms == 0 {
            return 0;
        }
        browse_civil_year((ms / 86400000) as i64)
    }

    // Howard Hinnant's civil_from_days, split into three so the chain parser
    // never sees a comma-bearing return type. `doy` is the day of the
    // March-based year the algorithm counts in.
    fn browse_civil_yoe(days: i64) -> i64 {
        let z = days + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let doe = z - era * 146097;
        (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365
    }

    fn browse_civil_doy(days: i64) -> i64 {
        let z = days + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let doe = z - era * 146097;
        let yoe = browse_civil_yoe(days);
        doe - (365 * yoe + yoe / 4 - yoe / 100)
    }

    fn browse_civil_month(days: i64) -> i64 {
        let mp = (5 * browse_civil_doy(days) + 2) / 153;
        if mp < 10 {
            mp + 3
        } else {
            mp - 9
        }
    }

    fn browse_civil_day(days: i64) -> i64 {
        let doy = browse_civil_doy(days);
        let mp = (5 * doy + 2) / 153;
        doy - (153 * mp + 2) / 5 + 1
    }

    fn browse_civil_year(days: i64) -> i64 {
        let z = days + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let y = browse_civil_yoe(days) + era * 400;
        if browse_civil_month(days) <= 2 {
            y + 1
        } else {
            y
        }
    }

    // ---- the glyphs -------------------------------------------------------
    // drawn, in currentColor, per /glyphs: white on the plain button and
    // black on /ember's tint, with no filter working to correct an asset.

    fn browse_stack_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<rect x=\"3\" y=\"7\" width=\"13\" height=\"14\" rx=\"2.5\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\"/>",
            "<path d=\"M8 4h10a3 3 0 0 1 3 3v11\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linecap=\"round\"/>",
            "</svg>"))
    }

    fn browse_grid_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<rect x=\"4\" y=\"4\" width=\"7\" height=\"7\" rx=\"1.8\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\"/>",
            "<rect x=\"13\" y=\"4\" width=\"7\" height=\"7\" rx=\"1.8\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\"/>",
            "<rect x=\"4\" y=\"13\" width=\"7\" height=\"7\" rx=\"1.8\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\"/>",
            "<rect x=\"13\" y=\"13\" width=\"7\" height=\"7\" rx=\"1.8\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\"/>",
            "</svg>"))
    }

    fn browse_list_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M4 6h16M4 12h16M4 18h16\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.4\" stroke-linecap=\"round\"/>",
            "</svg>"))
    }
}

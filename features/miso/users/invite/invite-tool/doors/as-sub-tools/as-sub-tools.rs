struct feature_AsSubTools;
impl feature_AsSubTools {
    // ---- the two ways in, as controls ---------------------------------------
    // the invite tool's sub-tools ride its control row, the way reset and −1
    // ride taps (/tools: "never put buttons on a page to choose between
    // actions"). This is the newest link on the chain, so /aside has already
    // taken undo out when there is nothing to undo and /current-only has
    // already dropped the parent's 👤 — inserting in front of whatever undo
    // button is left keeps undo last, which is every newcomer's job (/glyphs).
    //
    // `may` is read here as well as at /under-account's plus: a person who
    // cannot invite has no plus to reach this level with, and if a stale
    // frame ever showed the level anyway the two ways in are not offered.
    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state.clone());
        if open_tool_read() != "invite" {
            return row;
        }
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if !s["invite"]["may"].as_bool().unwrap_or(false) {
            return row;
        }
        let mut add = sub_tools_button("invite_qr".to_string(),
                                       "QR code".to_string(),
                                       sub_tools_qr_svg());
        add.push_str(&sub_tools_button("invite_name".to_string(),
                                       "by name".to_string(),
                                       sub_tools_name_svg()));
        sub_tools_before_undo(row, add)
    }

    // both wear the invite tool's own colour, not a colour of their own:
    // /ember's pick for "invite", the same tint the lit 👤 beside them wears
    // with more light. Two controls in one colour read as one pair — the two
    // ways into the same act (/glyphs, /posts' new button is the precedent).
    // `title` is the long-press card's fallback until /tool-words carries a
    // line for the event.
    fn sub_tools_button(ev: String, title: String, glyph: String) -> String {
        let colour = tool_colour("invite".to_string());
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        format!("<div class=\"tool-button ctrl{}\" data-ev=\"{}\" title=\"{}\">{}</div>",
                tint, ev, title, glyph)
    }

    // written out here rather than borrowed from /under-account's own
    // `before_undo`, so this node stands whichever of its siblings is ticked.
    // With undo already stripped by /aside there is no marker and the two
    // controls simply end the row.
    fn sub_tools_before_undo(row: String, add: String) -> String {
        if add.is_empty() {
            return row;
        }
        match row.find("data-ev=\"ctx_undo\"") {
            Some(at) => match row[..at].rfind("<div") {
                Some(start) => format!("{}{}{}", &row[..start], add, &row[start..]),
                None => format!("{}{}", row, add),
            },
            None => format!("{}{}", row, add),
        }
    }

    // ---- the page ----------------------------------------------------------
    // nothing is drawn. No `existing` call, exactly as /doors made none: the
    // two buttons leave and nothing takes their place. What remains is the
    // holder carrying the selected project, which the sheet reads to say where
    // the person is going — the same two attributes /doors put on its block,
    // under a name of this node's own so the two cannot both answer. The
    // holder is inside `.invite-page`, which this node's CSS does not draw;
    // attributes are still read from an undrawn element.
    fn invite_rows_html(inv: serde_json::Value) -> String {
        let _ = inv;
        let proj = current_project_card();
        let id = card_esc(proj["id"].as_str().unwrap_or("").to_string());
        let title = if proj.is_null() { String::new() } else { browse_title_of(&proj) };
        format!("<div class=\"invite-into\" data-project=\"{}\" data-project-title=\"{}\"></div>",
                id, title)
    }

    // ---- the glyphs ---------------------------------------------------------
    // drawn ink in currentColor, never a character: a QR code and a keyboard
    // both have emoji presentations that iOS would draw as colour bitmaps
    // (/glyphs, the undo arrow's lesson).

    // three finder squares and two rows of modules — the shape a phone camera
    // is pointed at
    fn sub_tools_qr_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<rect x=\"3.5\" y=\"3.5\" width=\"7\" height=\"7\" rx=\"1.6\" fill=\"none\" ",
            "stroke=\"currentColor\" stroke-width=\"2.2\"/>",
            "<rect x=\"13.5\" y=\"3.5\" width=\"7\" height=\"7\" rx=\"1.6\" fill=\"none\" ",
            "stroke=\"currentColor\" stroke-width=\"2.2\"/>",
            "<rect x=\"3.5\" y=\"13.5\" width=\"7\" height=\"7\" rx=\"1.6\" fill=\"none\" ",
            "stroke=\"currentColor\" stroke-width=\"2.2\"/>",
            "<path d=\"M14 15h1.5M19 15h1.5M14 20h6.5M19 17.5h0\" fill=\"none\" ",
            "stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linecap=\"round\"/>",
            "</svg>"))
    }

    // a keyboard: this road is the one where you type who they are. A person
    // with a pencil was the other candidate and is not used — 👤 already
    // stands beside it as the tool's own lit button, and the pencil is the
    // card page's edit mark; two meanings for one shape is one too many.
    fn sub_tools_name_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<rect x=\"2.5\" y=\"6\" width=\"19\" height=\"12\" rx=\"2.6\" fill=\"none\" ",
            "stroke=\"currentColor\" stroke-width=\"2.2\"/>",
            "<path d=\"M6.5 10h0M10 10h0M13.5 10h0M17 10h0M8 14h8\" fill=\"none\" ",
            "stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linecap=\"round\"/>",
            "</svg>"))
    }
}

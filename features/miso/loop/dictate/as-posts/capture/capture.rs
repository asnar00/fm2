struct feature_Capture;
impl feature_Capture {
    // ---- the extensible function -------------------------------------------
    // the capture kinds this row offers beyond the two that were already here.
    // The base is empty, so with both subfeatures unticked this node only
    // re-lays the controls it was handed.
    fn capture_extra(state: String) -> String {
        let _ = state;
        String::new()
    }

    // ---- the row -----------------------------------------------------------
    // the make-a-post controls, in one order: the capture kinds, then the
    // recording dot, then the plus, then /undo. They arrived one feature at a
    // time and each inserted itself in front of undo, so the row's order was
    // the order the features shipped in; this puts one order on it.
    //
    // The gate is /plus-at-home's rule, which /as-posts adopted: these belong
    // to the set of posts, not to a post you are reading.

    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state.clone());
        if open_tool_read() != "posts" || !browse_open_read().is_empty() {
            return row;
        }
        let plus = capture_grab(row.clone(), "posts_new".to_string());
        let rec = capture_grab(row.clone(), "dict_rec".to_string());
        let stop = capture_grab(row.clone(), "dict_stop".to_string());
        let row = capture_cut(row, "posts_new".to_string());
        let row = capture_cut(row, "dict_rec".to_string());
        let row = capture_cut(row, "dict_stop".to_string());
        // only one of rec/stop is ever drawn, and an absent control is the
        // empty string — the unticked state of the feature that would draw it.
        let set = format!("{}{}{}{}", capture_extra(state), rec, stop, plus);
        if set.is_empty() {
            return row;
        }
        posts_before_undo(row, set)
    }

    // one control out of the row, whole. A button in a control row nests no
    // div inside itself — the glyph is an <svg> and the pulse a <span> — so
    // the first `</div>` after the opening tag is the button's own, which is
    // the rule /posts' own inserter already depends on. A control that is not
    // in the row answers empty, and every caller treats that as "not there".
    fn capture_grab(row: String, ev: String) -> String {
        let needle = format!("data-ev=\"{}\"", ev);
        let at = match row.find(&needle) {
            Some(i) => i,
            None => return String::new(),
        };
        let start = match row[..at].rfind("<div") {
            Some(i) => i,
            None => return String::new(),
        };
        let end = match row[start..].find("</div>") {
            Some(j) => start + j + 6,
            None => return String::new(),
        };
        row[start..end].to_string()
    }

    fn capture_cut(row: String, ev: String) -> String {
        let piece = capture_grab(row.clone(), ev);
        if piece.is_empty() {
            return row;
        }
        row.replacen(&piece, "", 1)
    }

    // ---- the button --------------------------------------------------------
    // every kind's control is this one shape, wearing the posts tool's own
    // colour — /glyphs' rule for a tool's make button, and what makes four
    // controls read as one set rather than four arrivals (/taste 3).
    fn capture_button(ev: String, title: String, glyph: String) -> String {
        let colour = tool_colour("posts".to_string());
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        format!("<div class=\"tool-button ctrl capture{}\" data-ev=\"{}\" title=\"{}\">{}</div>",
                tint, ev, title, glyph)
    }
}

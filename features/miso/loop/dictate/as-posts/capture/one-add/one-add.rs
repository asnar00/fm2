struct feature_OneAdd;
impl feature_OneAdd {
    // ---- the mode, held on the device --------------------------------------
    // how you work is not what you own, so this is a device var — /browse's
    // own declaration for `view`, and the same consequence: the write queues
    // no op. Every read goes to the live context rather than to the bridged
    // loop state, because /payload republishes part-way down the update chain
    // and a render following this node's own write would be one turn stale.

    fn one_add_read() -> String {
        with_context(|c| c.one_add_mode_get())
    }

    fn one_add_write(mode: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/dictate/as-posts/capture/one-add", "mode",
                              serde_json::json!(mode.clone()));
        });
    }

    // ---- the row -----------------------------------------------------------
    // /capture laid four ways to make a post in one row; this folds them into
    // one add button and one setting. The gate is /capture's own — the posts
    // tool open with no card open — and everything here is a re-lay of
    // controls the row was handed, so unticking this node returns that row
    // exactly.

    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state.clone());
        if open_tool_read() != "posts" || !browse_open_read().is_empty() {
            return row;
        }
        // the plus IS the add button — it keeps its shape and its colour and
        // is given the chosen kind's event. Without it there is nothing to
        // fold, and the row goes back as it came.
        let plus = capture_grab(row.clone(), "posts_new".to_string());
        if plus.is_empty() {
            return row;
        }
        let photo = capture_grab(row.clone(), "capture_photo".to_string());
        let vid = capture_grab(row.clone(), "vid_rec".to_string());
        let rec = capture_grab(row.clone(), "dict_rec".to_string());
        // only one stop is ever drawn, and an absent control is the empty
        // string — the unticked state of the feature that would draw it.
        let stopping = format!("{}{}",
                               capture_grab(row.clone(), "vid_stop".to_string()),
                               capture_grab(row.clone(), "dict_stop".to_string()));
        let row = capture_cut(row, "posts_new".to_string());
        let row = capture_cut(row, "capture_photo".to_string());
        let row = capture_cut(row, "vid_rec".to_string());
        let row = capture_cut(row, "dict_rec".to_string());
        let row = capture_cut(row, "vid_stop".to_string());
        let row = capture_cut(row, "dict_stop".to_string());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let mode = one_add_mode(photo.clone(), vid.clone(), rec.clone());
        let face = one_add_mode_button(one_add_mode_glyph(mode.clone(), photo.clone(),
                                                          vid.clone(), rec.clone()));
        let set = if !stopping.is_empty() {
            // mid-recording the add slot is the stop the kind turned into, and
            // the mode control stays put so the row does not change shape.
            format!("{}{}", face, stopping)
        } else if s["oneadd_picking"].as_bool().unwrap_or(false) {
            one_add_choices(mode, photo, vid, rec)
        } else {
            format!("{}{}", face, one_add_add_button(plus, mode))
        };
        posts_before_undo(row, set)
    }

    // the stored mode, answered against what is actually drawn: a mode whose
    // kind is not in the row would point add at a button that is not there, so
    // it falls back to the kind that asks the phone for nothing.
    fn one_add_mode(photo: String, vid: String, rec: String) -> String {
        let mode = one_add_read();
        if mode == "photo" && !photo.is_empty() {
            return mode;
        }
        if mode == "video" && !vid.is_empty() {
            return mode;
        }
        if mode == "audio" && !rec.is_empty() {
            return mode;
        }
        "write".to_string()
    }

    // the event each kind's own feature already listens for. Nothing new runs
    // when add is tapped: /photo's capture-phase click, /video's and
    // /dictate's recording edges, /posts' own new button — all reached as
    // before, none of them touched.
    fn one_add_ev(mode: String) -> String {
        if mode == "photo" {
            return "capture_photo".to_string();
        }
        if mode == "video" {
            return "vid_rec".to_string();
        }
        if mode == "audio" {
            return "dict_rec".to_string();
        }
        "posts_new".to_string()
    }

    // ---- the buttons -------------------------------------------------------

    // the plus, with the chosen kind's event and the word for what it does.
    fn one_add_add_button(plus: String, mode: String) -> String {
        let ev = format!("data-ev=\"{}\"", one_add_ev(mode));
        let out = plus.replacen("data-ev=\"posts_new\"", &ev, 1);
        out.replacen("title=\"new\"", "title=\"add\"", 1)
    }

    // the quiet face: untinted, so the add button beside it is the one lit
    // thing in the pair (/taste 2 — hierarchy is dimness). No `capture` class,
    // so the collapsed row keeps the roomier gap; the picker's buttons carry
    // it and tighten as /capture's own set did.
    fn one_add_mode_button(glyph: String) -> String {
        format!("<div class=\"tool-button ctrl oneadd-mode\" data-ev=\"oneadd_pick\" title=\"mode\">{}</div>",
                glyph)
    }

    // one kind in the picker, wearing /capture's button shape. The chosen one
    // takes the accent that already means CHOSEN everywhere else (/taste 3),
    // as /browse's view picker does. The class list is closed by the tint (it
    // opens a style attribute), so every class is written BEFORE it.
    fn one_add_choice(mode: String, glyph: String, on: bool) -> String {
        let colour = tool_colour("posts".to_string());
        let lit = if on { " oneadd-on" } else { "" };
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        format!("<div class=\"tool-button ctrl capture{}{}\" data-ev=\"oneadd_mode:{}\" title=\"{}\">{}</div>",
                lit, tint, mode, mode, glyph)
    }

    // ---- the glyphs --------------------------------------------------------
    // a kind is offered only if its control was drawn, and its glyph is lifted
    // out of that control rather than drawn a second time — so the picker
    // obeys every toggle beneath it and never disagrees with the row.

    fn one_add_choices(mode: String, photo: String, vid: String, rec: String) -> String {
        let mut out = String::new();
        if !photo.is_empty() {
            out.push_str(&one_add_choice("photo".to_string(), one_add_glyph(photo),
                                         mode == "photo"));
        }
        if !vid.is_empty() {
            out.push_str(&one_add_choice("video".to_string(), one_add_glyph(vid),
                                         mode == "video"));
        }
        if !rec.is_empty() {
            out.push_str(&one_add_choice("audio".to_string(), one_add_glyph(rec),
                                         mode == "audio"));
        }
        out.push_str(&one_add_choice("write".to_string(), one_add_pencil_svg(),
                                     mode == "write"));
        out
    }

    fn one_add_mode_glyph(mode: String, photo: String, vid: String, rec: String) -> String {
        if mode == "photo" {
            return one_add_glyph(photo);
        }
        if mode == "video" {
            return one_add_glyph(vid);
        }
        if mode == "audio" {
            return one_add_glyph(rec);
        }
        one_add_pencil_svg()
    }

    // the inside of a control. A button in a control row nests no div — the
    // glyph is an <svg> — so the first `>` closes its opening tag and the last
    // `</div>` is its own, the rule /capture's grabber already depends on.
    fn one_add_glyph(control: String) -> String {
        let at = match control.find('>') {
            Some(i) => i + 1,
            None => return String::new(),
        };
        match control.rfind("</div>") {
            Some(j) => {
                if j < at {
                    return String::new();
                }
                control[at..j].to_string()
            }
            None => String::new(),
        }
    }

    // write is the one kind with no control of its own to borrow from: /posts'
    // plus is the add button and cannot also be the mode's face. Drawn, in
    // currentColor, per /glyphs — a nib, its body, and the line it writes on.
    fn one_add_pencil_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M14.6 4.6l4.8 4.8L9.6 19.2 4 20l0.8-5.6z\" fill=\"none\" ",
            "stroke=\"currentColor\" stroke-width=\"2.1\" stroke-linejoin=\"round\"/>",
            "<path d=\"M13.2 6l4.8 4.8\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2.1\" stroke-linecap=\"round\"/>",
            "</svg>"))
    }

    // ---- the events --------------------------------------------------------
    // the picker is open-on-tap, and open is a flag on the turn's state rather
    // than a var: a strip of four choices you come back to is the row ash
    // asked to be rid of. Choosing a kind closes it; so does any other tap.

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if ev == "oneadd_pick" {
            s["oneadd_picking"] = serde_json::json!(true);
            return s.to_string();
        }
        if ev.starts_with("oneadd_mode:") {
            one_add_write(ev["oneadd_mode:".len()..].to_string());
            s["oneadd_picking"] = serde_json::json!(false);
            return s.to_string();
        }
        if s["oneadd_picking"].as_bool().unwrap_or(false) {
            s["oneadd_picking"] = serde_json::json!(false);
            return s.to_string();
        }
        state
    }
}

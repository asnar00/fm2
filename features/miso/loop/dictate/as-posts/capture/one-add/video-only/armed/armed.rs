struct feature_Armed;
impl feature_Armed {
    // ---- the two settings, read from the live context -----------------------
    // every read goes through with_context rather than the bridged state, for
    // /flip's own reason: /payload republishes part-way down the update chain,
    // so a render after this node's own write would be one turn stale.

    fn armed_camera_read() -> String {
        with_context(|c| c.armed_camera_get())
    }

    fn armed_camera_write(camera: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/dictate/as-posts/capture/one-add/video-only/armed",
                              "camera", serde_json::json!(camera.clone()));
        });
    }

    fn armed_level_read() -> String {
        with_context(|c| c.armed_post_level_get())
    }

    fn armed_level_write(level: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/dictate/as-posts/capture/one-add/video-only/armed",
                              "post_level", serde_json::json!(level.clone()));
        });
    }

    // ---- the camera, taken over from /flip ----------------------------------
    // /flip holds the camera in `facing`, defaulting to back, and put its own
    // control in /one-add's kind picker — which /video-only no longer draws,
    // so since build 614 there has been no way to reach it. The camera button
    // in this row is where the choice lives now.
    //
    // The default is changed by owning the value, not by editing flip.vars: a
    // default written into /flip's own declaration would survive this node
    // being unticked, which is exactly what the toggle proof forbids. So
    // `flip_read` and `flip_write` are redefined onto this node's var, and
    // /flip's `facing` is dead while this node is composed. Untick, and
    // /flip's var, its default and its control are all its own again.
    //
    // These two are the only /flip names this node uses, and it does not CALL
    // either — so with /flip unticked they are two functions nobody calls, and
    // the camera button still chooses the camera through the page half.

    fn flip_read() -> String {
        armed_camera_read()
    }

    fn flip_write(facing: String) {
        armed_camera_write(facing)
    }

    // ---- the level a new post is stamped at ---------------------------------
    // /audience's seam, redefined. The base answers the author's own grade;
    // this answers the level they picked, when they picked one.
    //
    // Clamped, and the clamp is the whole ruling: a floor is the LOWEST rank
    // that holds the post, so a floor above the author's own rank would hide
    // the post from its own author — a volunteer choosing "admin" would post
    // into a room they are not in. Nobody may address a room above their own
    // rank, so a choice above it is not honoured and the author's own grade
    // stands. Choosing a level BELOW their rank is the whole point (a
    // candidate posting straight to volunteers), and promote still widens it
    // afterwards exactly as before.
    //
    // No `existing` call and no /audience name is used: with /audience
    // unticked this is one function nobody calls, and `card_new` never stamps
    // a floor at all.
    fn audience_new_floor(grade: String) -> String {
        let chosen = armed_level_read();
        if chosen.is_empty() {
            return grade;
        }
        if armed_rank(chosen.clone()) < armed_rank(grade.clone()) {
            return grade;
        }
        chosen
    }

    // where a word stands in the row this node itself draws, which is the same
    // order /audience ranks by. Read off this node's own list rather than
    // asked of /audience, so the dependency stays one-directional and
    // compile-free. A word not in the list ranks last — the widest floor —
    // which the caller only ever reaches for a value this node's pills wrote.
    fn armed_rank(word: String) -> usize {
        let words = armed_levels();
        let mut at = 0usize;
        for w in words.iter() {
            if w == &word {
                return at;
            }
            at = at + 1;
        }
        words.len()
    }

    // the six words, in /audience's order, highest rank first. Held here as
    // well as in audience.rs and audience.js because this node draws them and
    // must not fall over when /audience is not composed — the same call
    // audience.js made for its own pills. The order is the ruling
    // (saturday #p15) and a change to it is an ask, not a refactor.
    fn armed_levels() -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        out.push("admin".to_string());
        out.push("candidate".to_string());
        out.push("team".to_string());
        out.push("volunteer".to_string());
        out.push("supporter".to_string());
        out.push("public".to_string());
        out
    }

    // ---- the + arms, it does not record -------------------------------------
    // /one-add gives the plus the chosen kind's own event, and /video-only
    // makes that kind video — so the plus has been `vid_rec` since build 614
    // and a tap started filming at once. The ask (#p14) is that it opens a
    // row instead. This is the one seam that changes: the plus now carries
    // `tool_record`, which /tools answers like any `tool_<id>` by opening that
    // level. Nothing here writes `open_tool` — a write from a link newer than
    // /payload paints one stale frame (misses.md, "navigation from the wrong
    // side") — and nothing here navigates: the event does the moving at the
    // link that owns it.
    //
    // `record` is not in `tools_list`, so /one-level reads it as nested,
    // remembers `posts` as the way in, and ‹ climbs back to the posts list
    // with no help from here. `level` is nested under it the same way.
    //
    // Only the video kind is re-aimed: with /video unticked the mode falls
    // back to write and the plus is /posts' own new button, untouched.
    fn one_add_ev(mode: String) -> String {
        if mode == "video" {
            return "tool_record".to_string();
        }
        existing.one_add_ev(mode)
    }

    // ---- the two levels' rows ------------------------------------------------
    // the recording level carries the four buttons the ask names; the level
    // level carries the publish-level button, lit, as its own icon. Both go in
    // front of undo, which is every newcomer's job (/glyphs).
    //
    // /capture's whole set — the kinds, the plus, the stop — is gated on the
    // posts tool being open, so none of it is drawn at either level and this
    // row is this node's alone.
    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state.clone());
        let open = open_tool_read();
        if open == "record" {
            return armed_before_undo(row, armed_row(state));
        }
        if open == "level" {
            return armed_before_undo(row, armed_level_button(true));
        }
        row
    }

    // rec, stop, camera, publish level — in that order, which is the order of
    // the act: the two that do something first, the two that set it up after.
    // rec and stop are both always drawn and only one is ever live: the ask
    // named four buttons, and a row that changes shape under the finger is the
    // thing /one-add's own picker was taken away for. The dead one carries no
    // `data-ev` at all, so a tap on it sends nothing rather than sending
    // something that is quietly ignored.
    fn armed_row(state: String) -> String {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let recording = s["vid_recording"].as_bool().unwrap_or(false);
        let mut out = armed_act_button("vid_rec".to_string(), "rec".to_string(),
                                       armed_dot_svg(), !recording);
        out.push_str(&armed_act_button("vid_stop".to_string(), "stop".to_string(),
                                       armed_square_svg(), recording));
        out.push_str(&armed_camera_button());
        out.push_str(&armed_level_button(armed_level_lit(state)));
        out
    }

    // the two /extensible functions/ the level button hangs on: which event it
    // carries, and whether it is lit. Both were literals — `tool_level` and
    // `false` — and answer exactly what those literals answered. They are
    // functions so that a node making the list open IN this row, rather than
    // one level below it, has somewhere to say so without touching /tools.
    fn armed_level_ev() -> String {
        "tool_level".to_string()
    }

    fn armed_level_lit(state: String) -> bool {
        let _ = state;
        false
    }

    // the two acts wear the posts tool's own colour, because they are what the
    // plus was: the lit pair of the row (/glyphs — a tool's making button
    // wears the tool's colour). Dead, the tint comes off and the ink goes
    // quiet, so live and dead are told apart without reading the glyph.
    fn armed_act_button(ev: String, title: String, glyph: String, live: bool) -> String {
        if !live {
            return format!("<div class=\"tool-button ctrl armed-act off\" title=\"{}\">{}</div>",
                           title, glyph);
        }
        let colour = tool_colour("posts".to_string());
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        format!("<div class=\"tool-button ctrl armed-act{}\" data-ev=\"{}\" title=\"{}\">{}</div>",
                tint, ev, title, glyph)
    }

    // the camera you WILL get, not the act of flipping: readable without
    // colour and without a second tap, which is /flip's own reasoning for its
    // glyphs and the shapes it chose. Untinted, so the two acts beside it stay
    // the lit pair and this reads as the setting it is (/taste 2).
    fn armed_camera_button() -> String {
        let front = armed_camera_read() != "back";
        let title = if front { "front camera" } else { "back camera" };
        let glyph = if front { armed_face_svg() } else { armed_camera_svg() };
        format!("<div class=\"tool-button ctrl armed-set\" data-ev=\"armed_flip\" title=\"{}\">{}</div>",
                title, glyph)
    }

    // lit on the level it opens, the way an open tool's own button is.
    fn armed_level_button(sel: bool) -> String {
        let s = if sel { " sel" } else { "" };
        format!("<div class=\"tool-button ctrl armed-set{}\" data-ev=\"{}\" title=\"publish level\">{}</div>",
                s, armed_level_ev(), armed_sliders_svg())
    }

    // written out here rather than borrowed from /posts' `posts_before_undo`,
    // so this node stands whichever of its siblings is ticked.
    fn armed_before_undo(row: String, add: String) -> String {
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

    // ---- the taps ------------------------------------------------------------
    // rec and stop send /video's own two events, unchanged and unwrapped: the
    // recording edges, the minute cap, the poster and the filing are /video's
    // and this node does not touch any of them. `vid_rec` while the row is
    // open is exactly the tap the plus used to be.
    //
    // Two events are this node's own: the camera flip, and a pill on the level
    // page. And the lit button on a level means "one level up", the way every
    // open tool's own button does — handed to the chain as the ‹ event, which
    // /one-level already knows how to climb, rather than written as a
    // navigation var from a link newer than /payload.
    fn update(state: String, event: String) -> String {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return existing.update(state, event);
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        if ev == "tool_level" && open_tool_read() == "level" {
            let mut sent = e.clone();
            sent["ev"] = serde_json::json!("tools_home");
            return existing.update(state, sent.to_string());
        }
        // written BEFORE the chain beneath runs, not after: /payload publishes
        // the bridged state part-way down it, and a write made after that
        // publishes one turn late — the button's own title is right (it reads
        // the live context) while `feature_Flip.facing()`, which reads the
        // bridge, still answers the old camera. Rig-found, 2026-09-04: the
        // glyph said back and the constraint said user. /flip's note is about
        // the same edge from the render's side.
        if ev == "armed_flip" {
            armed_camera_write(if armed_camera_read() == "back" {
                "front".to_string()
            } else {
                "back".to_string()
            });
            return existing.update(state, event);
        }
        if let Some(pick) = ev.strip_prefix("armed_lvl_") {
            let want = pick.to_string();
            if want.is_empty() || armed_rank(want.clone()) < armed_levels().len() {
                armed_level_write(want);
            }
            return existing.update(state, event);
        }
        existing.update(state, event)
    }

    // ---- the level page -------------------------------------------------------
    // one level below the recording row: the publish options, as one list of
    // pills. A pill is picked and the choice is made — no form, no select, no
    // save (/taste 6, and /audience's own grade pills, which this row is drawn
    // to match).
    //
    // It wears its OWN class and not `.card-page`, though it is drawn to look
    // exactly like one. `.card-page` is what /editing calls a card: it puts a
    // pencil in the control row for every one it finds in the DOM after each
    // paint, and this page has no card to edit. Taking that pencil out again
    // from here was tried and is a page-freezing bug — /editing re-places it
    // on the next paint and the two chase each other. Not being a card is the
    // honest answer; armed.css carries the frame.
    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        if open_tool_read() != "level" {
            return base;
        }
        format!("{}<div class=\"armed-page\">{}</div>", base, armed_level_row())
    }

    // "same as me" first — it is the default and the rule every other entry is
    // a departure from — then the six words in rank order. The row carries a
    // one-word name, which is a name and not an explanation (/taste 7).
    fn armed_level_row() -> String {
        armed_level_box("publish level".to_string(),
                        armed_level_entries("armed_lvl_".to_string(),
                                            armed_level_read()))
    }

    // ---- the list, as two /extensible functions/ -----------------------------
    // split out of `armed_level_row` so a surface that wants the SAME list for
    // a different subject can have it rather than a copy of it: a picker on a
    // post's page names its own event prefix and its own lit level and gets
    // these rows, with whatever wording and shape the nodes beneath have given
    // them. Both answer exactly what the expression they came out of answered.

    // one row per level, lit where it matches, each carrying `prefix` plus its
    // own word as its event. The row with nothing after the prefix is
    // "same as me".
    fn armed_level_entries(prefix: String, lit: String) -> String {
        let mut pills = armed_pill(prefix.clone(), "same as me".to_string(),
                                   lit.is_empty());
        for g in armed_levels().iter() {
            pills.push_str(&armed_pill(format!("{}{}", prefix, g), g.clone(),
                                       &lit == g));
        }
        pills
    }

    // the named box the rows sit in.
    fn armed_level_box(what: String, entries: String) -> String {
        format!("<div class=\"armed-row\"><div class=\"armed-what\">{}</div><div class=\"armed-list\">{}</div></div>",
                what, entries)
    }

    // one option. `data-ev` and nothing else: the pill is inside #app, so
    // /loop's delegated listener sends the tap through the Rust chain and the
    // repaint that follows draws the new lit one. The value rides the event
    // name — `armed_lvl_` with nothing after it is "same as me".
    fn armed_pill(ev: String, label: String, on: bool) -> String {
        let lit = if on { " on" } else { "" };
        format!("<span class=\"armed-pill{}\" data-ev=\"{}\">{}</span>", lit, ev, label)
    }

    // ---- the glyphs ------------------------------------------------------------
    // drawn ink in currentColor, never a character: every one of these has an
    // emoji presentation iOS would draw as a colour bitmap (/glyphs).

    // rec: the filled dot every recorder has worn since tape.
    fn armed_dot_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<circle cx=\"12\" cy=\"12\" r=\"6.5\" fill=\"currentColor\"/>",
            "</svg>"))
    }

    // stop: the filled square, the dot's twin.
    fn armed_square_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<rect x=\"6.5\" y=\"6.5\" width=\"11\" height=\"11\" rx=\"2\" ",
            "fill=\"currentColor\"/>",
            "</svg>"))
    }

    // the back camera: /capture/video's own camera mark, redrawn here so this
    // node needs nothing of /video's at compile time.
    fn armed_camera_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<rect x=\"2.5\" y=\"6.5\" width=\"14\" height=\"11\" rx=\"2.4\" fill=\"none\" ",
            "stroke=\"currentColor\" stroke-width=\"2.1\"/>",
            "<path d=\"M16.5 11.5l5-3v11l-5-3z\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2.1\" stroke-linejoin=\"round\"/>",
            "</svg>"))
    }

    // the front camera: the person it points at — head and shoulders, the
    // shape 👤 stands for everywhere else here (/flip's own pick).
    fn armed_face_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<circle cx=\"12\" cy=\"8.2\" r=\"3.6\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2.1\"/>",
            "<path d=\"M4.8 20.2c0 -3.7 3.2 -5.8 7.2 -5.8s7.2 2.1 7.2 5.8\" fill=\"none\" ",
            "stroke=\"currentColor\" stroke-width=\"2.1\" stroke-linecap=\"round\"/>",
            "</svg>"))
    }

    // publish level: two sliders. A setting, said as a shape.
    fn armed_sliders_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M4 7.5h16M4 16.5h16\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2.2\" stroke-linecap=\"round\"/>",
            "<circle cx=\"9\" cy=\"7.5\" r=\"2.6\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2.2\"/>",
            "<circle cx=\"15.5\" cy=\"16.5\" r=\"2.6\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2.2\"/>",
            "</svg>"))
    }
}

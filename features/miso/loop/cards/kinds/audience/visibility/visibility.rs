struct feature_Visibility;
impl feature_Visibility {
    // ---- the button ----------------------------------------------------------
    // promote was one rung per tap and one direction: three taps to get a
    // candidate's post to the volunteers, and no way back. Ash asked for a
    // visibility toolbar that pops the same panel the recording row's settings
    // do (#p114), so the arrow goes and a picker takes its place.
    //
    // /audience's arrow is cut out of the row rather than gated off in its own
    // link — /plus-at-home's idiom — so /audience keeps every one of its own
    // tests and comes straight back when this node is unticked.
    //
    // The gate is /audience's own, minus one: your own post (a copy carries
    // `from`), in a project, and NOT stopped at public — because a picker goes
    // back down as well as up, and "already the widest" was only a reason to
    // hide a button that could not move.
    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state.clone());
        if open_tool_read() != "posts" {
            return row;
        }
        let open = browse_open_read();
        if open.is_empty() {
            return row;
        }
        let c = audience_card_by_id(open);
        if c.is_null() || !posts_is(&c) || !c["from"].is_null() {
            return row;
        }
        if audience_in_of(&c).is_empty() {
            return row;
        }
        let row = vis_strip(row, "posts_promote".to_string());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        posts_before_undo(row, vis_button(vis_open(&s)))
    }

    // an eye: what a person can see, said as a shape. Drawn in currentColor per
    // /glyphs — 👁 has an emoji presentation iOS would draw as a colour bitmap.
    // It wears the posts tool's colour, as the arrow it replaces did, and
    // lights while its panel is up the way every two-faced control here does.
    fn vis_button(lit: bool) -> String {
        let colour = tool_colour("posts".to_string());
        let sel = if lit { " sel" } else { "" };
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        format!("<div class=\"tool-button ctrl{}{}\" data-ev=\"posts_visibility\" title=\"visibility\">{}</div>",
                sel, tint, vis_eye_svg())
    }

    // /plus-at-home's cut, by the element's own event.
    fn vis_strip(html: String, ev: String) -> String {
        let marker = format!("data-ev=\"{}\"", ev);
        match html.find(marker.as_str()) {
            Some(at) => match (html[..at].rfind("<div"), html[at..].find("</div>")) {
                (Some(start), Some(rel)) => format!("{}{}", &html[..start],
                                                    &html[at + rel + 6..]),
                _ => html,
            },
            None => html,
        }
    }

    // ---- the panel -----------------------------------------------------------
    // the SAME list the recording row draws, not a copy of it: /armed's two
    // seams are asked for these rows with this node's own event prefix and the
    // post's own floor lit. Whatever /own-role, /explained and /plain-words
    // have done to that list — six rows, a sentence each, the plain words —
    // this surface gets for nothing and can never drift from.
    //
    // Open is a flag on the turn's state, and the box is /in-place's own
    // `.armed-pop`: one popover shape in the app, not two — the same ground,
    // the same place above the row, the same rise.
    fn vis_open(s: &serde_json::Value) -> bool {
        s["vis_picking"].as_bool().unwrap_or(false)
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if !vis_open(&s) || open_tool_read() != "posts" {
            return base;
        }
        let open = browse_open_read();
        if open.is_empty() {
            return base;
        }
        let c = audience_card_by_id(open);
        if c.is_null() || !posts_is(&c) || !c["from"].is_null() {
            return base;
        }
        format!("{}<div class=\"armed-pop\">{}</div>", base,
                armed_level_box("visibility".to_string(),
                                armed_level_entries("vis_lvl_".to_string(),
                                                    audience_floor_of(&c))))
    }

    // ---- the taps ------------------------------------------------------------
    // /in-place's rules, for /in-place's reasons: anything that is not the eye
    // closes the panel, and ‹ is caught before the chain so the first press
    // puts the panel away and leaves the card open — the second goes back to
    // the list, as it always did. A tap on bare ground is /in-place's own
    // listener, which sends `armed_close` whenever an `.armed-pop` is showing
    // and finds this one too.
    //
    // A pick arrives as `PostSetFloor` rather than as the click, because the
    // write needs a clock and there is none inside `update` (misses.md, the
    // clock in wasm) — visibility.js sends it, as /audience's promote does.
    // The same event closes the panel, so one tap does both things.
    fn update(state: String, event: String) -> String {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let kind = e["type"].as_str().unwrap_or("").to_string();
        if kind == "PostSetFloor" {
            // the pre-event world and the open tool are taken HERE, at the top
            // of the link, because `existing` may only be called from the
            // function whose chain it is — so the chain runs here and the write
            // is handed everything it needs.
            let before = with_context(|c| c.snapshot());
            let tool = open_tool_read();
            let state = existing.update(state, event);
            return vis_set(state, e, before, tool);
        }
        if kind != "click" {
            return existing.update(state, event);
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        let was: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let open = vis_open(&was);
        if ev == "tools_home" && open {
            let mut s: serde_json::Value = serde_json::from_str(&state)
                .unwrap_or(serde_json::json!({}));
            s["vis_picking"] = serde_json::json!(false);
            return s.to_string();
        }
        let state = existing.update(state, event);
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if ev == "posts_visibility" {
            s["vis_picking"] = serde_json::json!(!open);
            return s.to_string();
        }
        if open {
            s["vis_picking"] = serde_json::json!(false);
        }
        s.to_string()
    }

    // ---- the write -----------------------------------------------------------
    // a NEW event, not a widened `PostPromote`: promote means one rung, one
    // way, and /undo, the black box and anything reading the log are entitled
    // to keep that meaning. This one says where the floor is to be, and says it
    // once.
    //
    // It travels exactly the road promote travelled — the same `cards_write`,
    // the same `edited` bump — so `exchange_share` hands the post out to the
    // project as it always did, and a node watching for a floor that moved sees
    // this write like any other. Nothing new is transported.
    fn vis_set(state: String, e: serde_json::Value, before: serde_json::Value,
               tool: String) -> String {
        let id = e["data"]["id"].as_str().unwrap_or("").to_string();
        let asked = e["data"]["floor"].as_str().unwrap_or("").to_string();
        let now = e["data"]["t"].as_u64().unwrap_or(0);
        if id.is_empty() || now == 0 || !audience_is_grade(asked.clone()) {
            return vis_shut(state);
        }
        let all = cards_read();
        let mut list: serde_json::Value = serde_json::from_str(&all)
            .unwrap_or(serde_json::json!([]));
        if !list.is_array() {
            return vis_shut(state);
        }
        let mut changed = false;
        for c in list.as_array_mut().expect("cards is an array").iter_mut() {
            if c["id"].as_str().unwrap_or("") != id {
                continue;
            }
            // a copy is not yours to set — /exchange's structural test, the one
            // /delete and promote both use, never a comparison against a name
            if !c["from"].is_null() {
                println!("visibility: refused a floor on {} — that post is a copy", id);
                continue;
            }
            if !posts_is(c) || audience_in_of(c).is_empty() {
                continue;
            }
            let want = vis_clamped(all.clone(), c, asked.clone());
            // a pick on the level it is already at is not a write: no edit, no
            // hand-out, nothing on /undo's stack
            if audience_floor_of(c) == want {
                continue;
            }
            c["floor"] = serde_json::json!(want);
            c["edited"] = serde_json::json!(now);
            changed = true;
        }
        if !changed {
            return vis_shut(state);
        }
        cards_write(list.to_string());
        audience_record(before, tool);
        vis_shut(state)
    }

    // never above the author's own role, which is the recording row's clamp for
    // the recording row's reason: a floor above the author's rank would hide
    // the post from the person who wrote it. An author with no role in the
    // project the post is filed in is not clamped — there is nothing to clamp
    // to, and `card_new` would not have stamped a floor there at all.
    fn vis_clamped(all: String, card: &serde_json::Value, asked: String) -> String {
        let proj = audience_project_in(all, audience_in_of(card));
        if proj.is_null() {
            return asked;
        }
        let mine = audience_grade_in(&proj, card["owner"].as_str().unwrap_or("").to_string());
        if mine.is_empty() {
            return asked;
        }
        if audience_rank(asked.clone()) < audience_rank(mine.clone()) {
            return mine;
        }
        asked
    }

    // the panel closes on the pick whatever the pick did, so a tap never leaves
    // it standing over its own answer.
    fn vis_shut(state: String) -> String {
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["vis_picking"] = serde_json::json!(false);
        s.to_string()
    }

    // ---- the glyph -----------------------------------------------------------
    // an eye: an outline and a pupil, in currentColor. What a person can see,
    // said as a shape (/glyphs).
    fn vis_eye_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M2.5 12s3.6-6 9.5-6 9.5 6 9.5 6-3.6 6-9.5 6-9.5-6-9.5-6z\" ",
            "fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.1\" ",
            "stroke-linejoin=\"round\"/>",
            "<circle cx=\"12\" cy=\"12\" r=\"2.6\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2.1\"/>",
            "</svg>"))
    }
}

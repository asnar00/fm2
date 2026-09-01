struct feature_Video;
impl feature_Video {
    // ---- the control -------------------------------------------------------
    // the second kind in the set, after the camera and in front of the dot.
    // Recording or not is the same two-faced control the dot is: the tap
    // writes intent into state and the page half follows its edges.

    fn capture_extra(state: String) -> String {
        let prev = existing.capture_extra(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["vid_recording"].as_bool().unwrap_or(false) {
            return format!("{}{}", prev, video_stop_button());
        }
        format!("{}{}", prev,
                capture_button("vid_rec".to_string(), "video".to_string(),
                               video_camera_svg()))
    }

    // the recording face, with /dictate's own breathing dot. Written out
    // rather than borrowed because /capture's button has no pulse: the class
    // list is closed by the tint (it opens a style attribute), so every class
    // this button wears is written BEFORE it.
    fn video_stop_button() -> String {
        let colour = tool_colour("posts".to_string());
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        format!("<div class=\"tool-button ctrl capture recording{}\" data-ev=\"vid_stop\" title=\"stop\">{}<span class=\"rec-dot\"></span></div>",
                tint, video_square_svg())
    }

    // ---- the intent --------------------------------------------------------
    // one camera at a time, and one microphone: starting a video stops a
    // recording and starting a recording stops a video. The page halves watch
    // their own flag, so each sees an edge and does the right hardware thing;
    // a recording cut short this way is saved, not lost — /dictate's stop
    // path is the same one the stop button takes.

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
        if ev == "vid_rec" {
            s["vid_recording"] = serde_json::json!(true);
            s["dict_recording"] = serde_json::json!(false);
            return s.to_string();
        }
        if ev == "vid_stop" {
            s["vid_recording"] = serde_json::json!(false);
            return s.to_string();
        }
        if ev == "dict_rec" {
            s["vid_recording"] = serde_json::json!(false);
            return s.to_string();
        }
        state
    }

    // ---- a video is a recording ---------------------------------------------
    // /as-posts already answers the four arrivals a recording has — made
    // here, announced by another device, in the boot index, already in
    // IndexedDB — with one pass over `dict_files`. A video rides that pass
    // whole: the same meta, the same id-per-device, the same minting, the
    // same transcript landing in the words. Only the block's kind differs, so
    // that is all this redefines.

    fn as_posts_card(owner: String, file: &serde_json::Value, t: u64) -> serde_json::Value {
        let mut card = existing.as_posts_card(owner, file, t);
        if file["kind"].as_str().unwrap_or("") != "video" {
            return card;
        }
        if let Some(blocks) = card["blocks"].as_array_mut() {
            for b in blocks.iter_mut() {
                if b["kind"].as_str().unwrap_or("") == "audio" {
                    b["kind"] = serde_json::json!("video");
                }
            }
        }
        card
    }

    fn video_block(card: &serde_json::Value) -> serde_json::Value {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") == "video" {
                return b.clone();
            }
        }
        serde_json::Value::Null
    }

    // ---- the page ----------------------------------------------------------
    // a mount before the words, where /as-posts puts its play row. The player
    // itself is the page half's: a <video> wants a blob URL, and the blob is
    // in IndexedDB or, the first time, on the exchange. `data-rec` is what
    // /as-posts' transcribing hint looks for, so a video says "transcribing…"
    // in its empty words exactly as a recording does.

    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let v = video_block(&c);
        if v.is_null() {
            return html;
        }
        let row = if c["from"].is_null() {
            video_mount(&v)
        } else {
            video_foreign(&v)
        };
        match html.find("<div class=\"card-text") {
            Some(at) => format!("{}{}{}", &html[..at], row, &html[at..]),
            None => html,
        }
    }

    fn video_mount(block: &serde_json::Value) -> String {
        let id = card_esc(block["id"].as_str().unwrap_or("").to_string());
        let dur = as_posts_mmss(block["dur"].as_u64().unwrap_or(0));
        format!(concat!("<div class=\"post-video\" data-vid=\"{}\" data-rec=\"{}\">",
                        "<span class=\"post-dur\">{}</span></div>"),
                id, id, dur)
    }

    // a copy carries the block and the words; the bytes do not, because
    // /mirror's blob route is per-user. The control says what it is rather
    // than pretending it will play (/taste 7) — /as-posts' own ruling, in the
    // same words, for the same reason.
    fn video_foreign(block: &serde_json::Value) -> String {
        let dur = as_posts_mmss(block["dur"].as_u64().unwrap_or(0));
        format!(concat!("<div class=\"post-video dim\">",
                        "<span class=\"post-video-glyph\">{}</span>",
                        "<span class=\"post-dur\">{}</span>",
                        "<span class=\"post-play-note\">video stays with its owner</span>",
                        "</div>"),
                video_camera_svg(), dur)
    }

    // ---- the tile ----------------------------------------------------------
    // a video is recognisable in the grid without opening it, the way a
    // recording is. The row is not marked: /portrait's cells are the author,
    // the words and the date, and a fourth mark in a row is more than was
    // asked for (/as-posts' ruling).

    fn card_tile_html(card: String) -> String {
        let html = existing.card_tile_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        if video_block(&c).is_null() {
            return html;
        }
        let mark = format!("<span class=\"tile-video\">{}</span>",
                           video_camera_svg());
        match html.find("<div class=\"card-tile-title\"") {
            Some(at) => format!("{}{}{}", &html[..at], mark, &html[at..]),
            None => html,
        }
    }

    // ---- the glyphs --------------------------------------------------------
    // drawn, in currentColor, per /glyphs. A body with a lens barrel is a
    // video camera; a rounded square is stop.

    fn video_camera_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<rect x=\"2.8\" y=\"6.6\" width=\"12.4\" height=\"10.8\" rx=\"2.4\" ",
            "fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.1\"/>",
            "<path d=\"M15.2 12l6 -3.4v6.8z\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2.1\" stroke-linejoin=\"round\"/>",
            "</svg>"))
    }

    fn video_square_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<rect x=\"6.5\" y=\"6.5\" width=\"11\" height=\"11\" rx=\"2.4\" ",
            "fill=\"currentColor\"/>",
            "</svg>"))
    }
}

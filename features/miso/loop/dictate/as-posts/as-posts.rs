struct feature_AsPosts;
impl feature_AsPosts {
    // ---- the tool retires --------------------------------------------------
    // /people's idiom: filter the registry chain. `tools_catalog` is written
    // at init from this same chain, so the chooser's catalog and /long-press'
    // sub-tool cards lose 🎤 for nothing. /dictate's render and /transcript's
    // panel only draw while `dictate` is the open tool, which can no longer
    // happen; its update and its page half keep working, which is the point.

    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        let list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let kept: Vec<serde_json::Value> = list.as_array().unwrap_or(&empty).iter()
            .filter(|t| t["id"].as_str() != Some("dictate"))
            .cloned().collect();
        serde_json::Value::Array(kept).to_string()
    }

    // ---- the record control ------------------------------------------------
    // while the posts tool is open and no card is open — /plus-at-home's rule
    // for the + is this control's too: these belong to the set of posts, not
    // to a post you are reading. In front of /undo's button through /posts'
    // own inserter (/glyphs — undo is last in every row).
    //
    // The events are /dictate's own, so its update and its page half do the
    // recording with nothing added and nothing wrapped.

    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state.clone());
        if open_tool_read() != "posts" || !browse_open_read().is_empty() {
            return row;
        }
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let on = s["dict_recording"].as_bool().unwrap_or(false);
        posts_before_undo(row, as_posts_rec_button(on))
    }

    // the tool's own colour, like the + beside it. The class list is closed by
    // the tint (it opens a style attribute), so every class this button wears
    // is written BEFORE it.
    fn as_posts_rec_button(on: bool) -> String {
        let colour = tool_colour("posts".to_string());
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        if on {
            format!("<div class=\"tool-button ctrl recording{}\" data-ev=\"dict_stop\" title=\"stop\">{}<span class=\"rec-dot\"></span></div>",
                    tint, as_posts_stop_svg())
        } else {
            format!("<div class=\"tool-button ctrl{}\" data-ev=\"dict_rec\" title=\"record\">{}</div>",
                    tint, as_posts_dot_svg())
        }
    }

    // ---- a recording becomes a post ----------------------------------------
    // one pass over dict_files after every event, rather than a handler on
    // RecSaved: the same answer is owed to four arrivals — a recording made
    // here, one /mirror announces from this person's other device, the index
    // that comes back at boot, and the notes already in IndexedDB when this
    // node ships. One rule asked every turn gets all four right and cannot be
    // raced. The pass is skipped outright when there are no recordings, which
    // is every turn on a device that has never recorded.

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event);
        as_posts_sync(state)
    }

    fn as_posts_sync(state: String) -> String {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let empty: Vec<serde_json::Value> = Vec::new();
        let files = s["dict_files"].as_array().unwrap_or(&empty).clone();
        if files.is_empty() {
            return state;
        }
        // the name is behind the cookie and never in the world, and update
        // runs in the page's wasm where there is no cookie to read. Your own
        // profile card is where it landed. Until there is one, nothing is
        // minted: a card under the wrong owner could not be handed on
        // (/exchange) and could not be corrected (/guard/owner).
        let owner = as_posts_owner();
        if owner.is_empty() {
            return state;
        }
        let mut list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        if !list.is_array() {
            return state;
        }
        let mut changed = false;
        for f in files.iter() {
            let rec = f["id"].as_str().unwrap_or("").to_string();
            let t = f["t"].as_u64().unwrap_or(0);
            if rec.is_empty() || t == 0 {
                continue;
            }
            let mut found = false;
            for c in list.as_array_mut().expect("cards is an array").iter_mut() {
                if c["rec"].as_str().unwrap_or("") != rec {
                    continue;
                }
                found = true;
                if as_posts_land(c, f) {
                    changed = true;
                }
            }
            if found {
                continue;
            }
            let card = as_posts_card(owner.clone(), f, t);
            list.as_array_mut().expect("cards is an array").push(card);
            changed = true;
        }
        if changed {
            cards_write(list.to_string());
        }
        state
    }

    // your own name, off your own card. /me's lookup, which /exchange narrowed
    // to cards you own rather than copies you hold.
    fn as_posts_owner() -> String {
        let card = card_of_type(cards_read(), String::new(), "profile".to_string());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        c["owner"].as_str().unwrap_or("").trim().to_string()
    }

    // the card: /cards' own three-block body plus the audio block at index 3,
    // so title, picture and text keep the indices /keep and /frame send.
    // `created` is the recording's own moment, so the id /cards mints is
    // <owner>.<t> — the same id on this person's every device, which is what
    // lets two of them agree with no protocol at all. `when` says the same
    // thing to /post-time, so an old note sorts among the posts of the day it
    // was made.
    fn as_posts_card(owner: String, file: &serde_json::Value, t: u64) -> serde_json::Value {
        let mut card = card_new(owner, "post".to_string(), t);
        card["blocks"][0]["text"] = serde_json::json!("");
        card["blocks"].as_array_mut().expect("blocks is an array")
            .push(serde_json::json!({
                "kind": "audio",
                "id": file["id"].clone(),
                "dur": file["dur"].as_u64().unwrap_or(0),
                "mime": file["mime"].as_str().unwrap_or("")
            }));
        // the key lives on the card, not in the block: /delete's tombstone
        // empties `blocks` but clones the rest, so `rec` survives a delete and
        // a deleted recording is never resurrected by the next pass.
        card["rec"] = file["id"].clone();
        card["when"] = serde_json::json!(t);
        card["when_from"] = serde_json::json!("recording");
        let _ = as_posts_land(&mut card, file);
        card
    }

    // the transcript into the words, and NEVER over the user's own. /keep
    // writes a block's `text` and nothing else, so the test cannot be a flag
    // an edit clears: `auto` is a hash of the words this node last wrote, and
    // the words are replaced only while they are empty or still hash to it.
    // One keystroke and the hash stops matching, for good.
    fn as_posts_land(card: &mut serde_json::Value, file: &serde_json::Value) -> bool {
        let text = file["transcript"].as_str().unwrap_or("").to_string();
        if text.is_empty() {
            return false;
        }
        let mut changed = false;
        if let Some(blocks) = card["blocks"].as_array_mut() {
            for b in blocks.iter_mut() {
                if b["kind"].as_str().unwrap_or("") != "text" {
                    continue;
                }
                let cur = b["text"].as_str().unwrap_or("").to_string();
                if cur == text {
                    continue;
                }
                let mine = cur.trim().is_empty()
                    || b["auto"].as_u64().unwrap_or(0) == as_posts_hash(cur);
                if !mine {
                    continue;
                }
                b["auto"] = serde_json::json!(as_posts_hash(text.clone()));
                b["text"] = serde_json::json!(text.clone());
                changed = true;
            }
        }
        if changed {
            // there is no clock inside update — time rides on the event, and
            // this pass has none of its own. /guard needs only that the change
            // look newer than what it replaces, so one millisecond is enough.
            let was = card["edited"].as_u64().unwrap_or(0);
            card["edited"] = serde_json::json!(was + 1);
        }
        changed
    }

    // FNV-1a, and never zero: an absent `auto` reads as zero, and must not
    // match any words that were actually written.
    fn as_posts_hash(s: String) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for b in s.as_bytes() {
            h ^= *b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        if h == 0 {
            return 1;
        }
        h
    }

    // ---- the page ----------------------------------------------------------
    // a play row before the words. The tap is /dictate's own dict_play_<id>,
    // so playing and stopping are the page half that already fetches the blob
    // — from the exchange the first time, with /mirror.

    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let a = as_posts_audio(&c);
        if a.is_null() {
            return html;
        }
        let row = if c["from"].is_null() {
            as_posts_play_row(&a)
        } else {
            as_posts_foreign_row(&a)
        };
        match html.find("<div class=\"card-text") {
            Some(at) => format!("{}{}{}", &html[..at], row, &html[at..]),
            None => html,
        }
    }

    fn as_posts_audio(card: &serde_json::Value) -> serde_json::Value {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") == "audio" {
                return b.clone();
            }
        }
        serde_json::Value::Null
    }

    // both glyphs are drawn and CSS picks between them, because which
    // recording is playing lives in the loop state and not on the card;
    // `render` puts the class on. `data-rec` is what the transcribing hint
    // looks for.
    fn as_posts_play_row(audio: &serde_json::Value) -> String {
        let id = card_esc(audio["id"].as_str().unwrap_or("").to_string());
        let dur = as_posts_mmss(audio["dur"].as_u64().unwrap_or(0));
        format!(concat!("<div class=\"post-play\" data-ev=\"dict_play_{}\" data-rec=\"{}\">",
                        "<span class=\"post-play-glyph play\">{}</span>",
                        "<span class=\"post-play-glyph stop\">{}</span>",
                        "<span class=\"post-dur\">{}</span></div>"),
                id, id, as_posts_triangle_svg(), as_posts_stop_svg(), dur)
    }

    // a copy carries the audio block and the words; the blob does not, because
    // /mirror's blob route is per-user. So the control says what it is rather
    // than pretending it will play (/taste 7) — and carries no data-ev, so
    // there is nothing for a tap to fire.
    fn as_posts_foreign_row(audio: &serde_json::Value) -> String {
        let dur = as_posts_mmss(audio["dur"].as_u64().unwrap_or(0));
        format!(concat!("<div class=\"post-play dim\">",
                        "<span class=\"post-play-glyph play\">{}</span>",
                        "<span class=\"post-dur\">{}</span>",
                        "<span class=\"post-play-note\">audio stays with its owner</span>",
                        "</div>"),
                as_posts_triangle_svg(), dur)
    }

    fn as_posts_mmss(secs: u64) -> String {
        format!("{}:{:02}", secs / 60, secs % 60)
    }

    // ---- the two things that need the loop state ---------------------------
    // the played row's glyph, and the placeholder while the phone is still
    // listening. Both are one string swap on the drawn page: only one card
    // page is ever open, and both marks name the recording they belong to.

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let playing = s["dict_playing"].as_str().unwrap_or("").to_string();
        let base = if playing.is_empty() {
            base
        } else {
            base.replace(&format!("<div class=\"post-play\" data-ev=\"dict_play_{}\"", playing),
                         &format!("<div class=\"post-play playing\" data-ev=\"dict_play_{}\"", playing))
        };
        let queued = s["dict_transcribe"]["id"].as_str().unwrap_or("").to_string();
        if queued.is_empty() || !base.contains(&format!("data-rec=\"{}\"", queued)) {
            return base;
        }
        // a placeholder is only ever seen on an empty block (/cards draws it
        // with :empty::before), so this says "transcribing…" exactly while
        // there are no words to say instead.
        base.replace("data-ph=\"say something\"", "data-ph=\"transcribing…\"")
    }

    // ---- the tile ----------------------------------------------------------
    // a recording is recognisable in the grid without opening it. The row is
    // not marked: /portrait's cells are the author, the words and the date,
    // and a fourth mark in a row is more than was asked for.

    fn card_tile_html(card: String) -> String {
        let html = existing.card_tile_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        if as_posts_audio(&c).is_null() {
            return html;
        }
        let mark = format!("<span class=\"tile-audio\">{}</span>",
                           as_posts_triangle_svg());
        match html.find("<div class=\"card-tile-title\"") {
            Some(at) => format!("{}{}{}", &html[..at], mark, &html[at..]),
            None => html,
        }
    }

    // ---- the glyphs --------------------------------------------------------
    // drawn, in currentColor, per /glyphs — never a character with an emoji
    // presentation. A filled dot is what recording is; a rounded square is
    // stop; a triangle is play.

    fn as_posts_dot_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<circle cx=\"12\" cy=\"12\" r=\"6.5\" fill=\"currentColor\"/>",
            "</svg>"))
    }

    fn as_posts_stop_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<rect x=\"6.5\" y=\"6.5\" width=\"11\" height=\"11\" rx=\"2.4\" fill=\"currentColor\"/>",
            "</svg>"))
    }

    fn as_posts_triangle_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M8.5 5.6l10 6.4-10 6.4z\" fill=\"currentColor\" ",
            "stroke=\"currentColor\" stroke-width=\"2\" stroke-linejoin=\"round\"/>",
            "</svg>"))
    }
}
